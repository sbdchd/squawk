use snapbox::{cmd::Command, file};

#[test]
fn example_sql_svg() {
    let expected = file!["../src/snapshots/example.svg": TermSvg];
    let bin_path = snapbox::cmd::cargo_bin!("squawk");
    Command::new(bin_path)
        .env("CLICOLOR_FORCE", "1")
        .env("SQUAWK_DISABLE_GITHUB_ANNOTATIONS", "1")
        .arg("check")
        .arg("../../example.sql")
        .assert()
        .code(1) // squawk returns 1 when it finds violations
        .stderr_eq("")
        .stdout_eq(expected.raw());
}

#[test]
fn top_level_paths() {
    let expected = file!["../src/snapshots/example.svg": TermSvg];
    let bin_path = snapbox::cmd::cargo_bin!("squawk");
    Command::new(bin_path)
        .env("CLICOLOR_FORCE", "1")
        .env("SQUAWK_DISABLE_GITHUB_ANNOTATIONS", "1")
        .arg("../../example.sql")
        .assert()
        .code(1)
        .stderr_eq("")
        .stdout_eq(expected.raw());
}

#[test]
fn top_level_check_args_conflict_with_other_commands() {
    let bin_path = snapbox::cmd::cargo_bin!("squawk");
    for command in ["server", "upload-to-github"] {
        for arg in [
            "--debug=ast",
            "--reporter=gcc",
            "--stdin-filepath=example.sql",
        ] {
            let mut args = vec![arg, command];
            if command == "upload-to-github" {
                args.extend([
                    "--github-repo-owner=owner",
                    "--github-repo-name=repo",
                    "--github-pr-number=1",
                ]);
            }
            Command::new(bin_path)
                .args(args)
                .assert()
                .code(2)
                .stderr_eq(format!(
                    "error: paths, `--debug`, `--reporter`, and `--stdin-filepath` cannot be used with `{command}`\n\nUsage: squawk [OPTIONS] [path]... [COMMAND]\n\nFor more information, try '--help'.\n",
                ));
        }
    }
}

#[test]
fn top_level_and_check_args_conflict() {
    let bin_path = snapbox::cmd::cargo_bin!("squawk");
    Command::new(bin_path)
        .arg("--reporter=gcc")
        .arg("check")
        .arg("../../example.sql")
        .assert()
        .code(2)
        .stderr_eq(
            "error: paths, `--debug`, `--reporter`, and `--stdin-filepath` must come after `check`\n\nUsage: squawk [OPTIONS] [path]... [COMMAND]\n\nFor more information, try '--help'.\n",
        );
}
