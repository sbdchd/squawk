use snapbox::{cmd::Command, file};

#[test]
fn example_sql_svg() {
    let expected = file!["../src/snapshots/example.svg": TermSvg];
    let bin_path = snapbox::cmd::cargo_bin("squawk");
    Command::new(bin_path)
        .env("CLICOLOR_FORCE", "1")
        .env("SQUAWK_DISABLE_GITHUB_ANNOTATIONS", "1")
        .arg("../../example.sql")
        .assert()
        .code(1) // squawk returns 1 when it finds violations
        .stderr_eq("")
        .stdout_eq(expected.raw());
}
