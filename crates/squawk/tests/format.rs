use std::path::Path;

use snapbox::cmd::Command;

fn squawk(dir: &Path) -> Command {
    Command::new(snapbox::cmd::cargo_bin!("squawk")).current_dir(dir)
}

fn setup() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir(dir.path().join("sub")).unwrap();
    std::fs::write(dir.path().join("a.sql"), "select   1;\n").unwrap();
    std::fs::write(dir.path().join("b.sql"), "select 1;\n").unwrap();
    std::fs::write(dir.path().join("sub/c.sql"), "SELECT a,b from t;\n").unwrap();
    dir
}

fn read(dir: &Path, path: &str) -> String {
    std::fs::read_to_string(dir.join(path)).unwrap()
}

#[test]
fn write_formats_files_in_place() {
    let dir = setup();
    squawk(dir.path())
        .args(["format", "."])
        .assert()
        .code(0)
        .stderr_eq("")
        .stdout_eq("2 files reformatted, 1 file left unchanged\n");
    assert_eq!(read(dir.path(), "a.sql"), "select 1;\n");
    assert_eq!(read(dir.path(), "sub/c.sql"), "select a, b from t;\n");

    squawk(dir.path())
        .args(["format", "."])
        .assert()
        .code(0)
        .stdout_eq("3 files left unchanged\n");
}

#[test]
fn exit_non_zero_on_format() {
    let dir = setup();
    squawk(dir.path())
        .args(["format", "--exit-non-zero-on-format", "a.sql"])
        .assert()
        .code(1)
        .stdout_eq("1 file reformatted\n");
}

#[test]
fn check_does_not_write() {
    let dir = setup();
    squawk(dir.path())
        .args(["format", "--check", "."])
        .assert()
        .code(1)
        .stderr_eq("")
        .stdout_eq(
            "Would reformat: ./a.sql\nWould reformat: ./sub/c.sql\n2 files would be reformatted, 1 file already formatted\n",
        );
    assert_eq!(read(dir.path(), "a.sql"), "select   1;\n");

    squawk(dir.path())
        .args(["format", "--check", "b.sql"])
        .assert()
        .code(0)
        .stdout_eq("1 file already formatted\n");
}

#[test]
fn diff_writes_summary_to_stderr() {
    let dir = setup();
    squawk(dir.path())
        .args(["format", "--diff", "a.sql", "b.sql"])
        .assert()
        .code(1)
        .stdout_eq("--- a.sql\n+++ a.sql\n@@ -1 +1 @@\n-select   1;\n+select 1;\n")
        .stderr_eq("1 file would be reformatted, 1 file already formatted\n");
    assert_eq!(read(dir.path(), "a.sql"), "select   1;\n");
}

#[test]
fn syntax_error_exits_with_error() {
    let dir = setup();
    std::fs::write(dir.path().join("bad.sql"), "select (;\n").unwrap();
    squawk(dir.path())
        .args(["format", "a.sql", "bad.sql"])
        .assert()
        .code(2)
        .stdout_eq("1 file reformatted\n")
        .stderr_eq(
            "error[syntax-error]: expected an expression, found `;`
  ╭▸ bad.sql:1:9
  │
1 │ select (;
  ╰╴        ━
error[syntax-error]: expected `)`
  ╭▸ bad.sql:1:9
  │
1 │ select (;
  ╰╴        ━
",
        );
    assert_eq!(read(dir.path(), "a.sql"), "select 1;\n");
    assert_eq!(read(dir.path(), "bad.sql"), "select (;\n");
}

#[test]
fn stdin() {
    let dir = setup();
    squawk(dir.path())
        .arg("format")
        .stdin("SELECT   1;\n")
        .assert()
        .code(0)
        .stdout_eq("select 1;\n");

    squawk(dir.path())
        .args(["format", "--check"])
        .stdin("SELECT   1")
        .assert()
        .code(1)
        .stdout_eq("");

    squawk(dir.path())
        .args(["format", "--diff", "--stdin-filepath=x.sql"])
        .stdin("select   1\n")
        .assert()
        .code(1)
        .stdout_eq("--- x.sql\n+++ x.sql\n@@ -1 +1 @@\n-select   1\n+select 1\n");
}

#[test]
fn preserves_bom_and_line_endings() {
    let dir = setup();
    std::fs::write(
        dir.path().join("crlf.sql"),
        "\u{feff}SELECT 1;\r\nSELECT 2;\r\n",
    )
    .unwrap();
    squawk(dir.path())
        .args(["format", "crlf.sql"])
        .assert()
        .code(0);
    assert_eq!(
        read(dir.path(), "crlf.sql"),
        "\u{feff}select 1;\r\nselect 2;\r\n"
    );
}

#[test]
fn unmatched_pattern() {
    let dir = setup();
    squawk(dir.path())
        .args(["format", "missing.sql"])
        .assert()
        .code(1)
        .stderr_eq(
            r#"Failed to find files for provided patterns: ["missing.sql"]
"#,
        );

    squawk(dir.path())
        .args(["format", "--no-error-on-unmatched-pattern", "missing.sql"])
        .assert()
        .code(0);
}

#[test]
fn top_level_check_args_conflict_with_format() {
    let dir = setup();
    squawk(dir.path())
        .args(["--reporter=gcc", "format"])
        .assert()
        .code(2)
        .stderr_eq(
            "error: paths, `--debug`, `--reporter`, and `--stdin-filepath` cannot be used before `format`\n\nUsage: squawk [OPTIONS] [path]... [COMMAND]\n\nFor more information, try '--help'.\n",
        );
}

#[test]
fn bom_is_not_a_column_in_syntax_errors() {
    let dir = setup();
    squawk(dir.path())
        .arg("format")
        .stdin("\u{feff}select (")
        .assert()
        .code(2)
        .stderr_eq(
            "error[syntax-error]: expected `)`
  ╭▸ stdin:1:9
  │
1 │ select (
  ╰╴        ━
",
        );
}
