use snapbox::cmd::Command;

#[test]
fn malformed_include_patterns_report_the_parse_error() {
    let directory = tempfile::tempdir().unwrap();
    std::fs::write(directory.path().join("migration.sql"), "select 1;").unwrap();

    for args in [
        vec!["migration.sql", "["],
        vec!["[", "migration.sql"],
        vec!["["],
        vec!["--no-error-on-unmatched-pattern", "["],
    ] {
        Command::new(snapbox::cmd::cargo_bin!("squawk"))
            .current_dir(directory.path())
            .args(args)
            .assert()
            .code(1)
            .stdout_eq("")
            .stderr_eq(
                "Failed to find files: Pattern syntax error near position 0: invalid range pattern\n",
            );
    }
}

#[test]
fn valid_unmatched_pattern_alongside_a_file_is_allowed() {
    let directory = tempfile::tempdir().unwrap();
    std::fs::write(directory.path().join("migration.sql"), "select 1;").unwrap();

    Command::new(snapbox::cmd::cargo_bin!("squawk"))
        .current_dir(directory.path())
        .args(["migration.sql", "missing-*.sql"])
        .assert()
        .code(0)
        .stderr_eq("");
}
