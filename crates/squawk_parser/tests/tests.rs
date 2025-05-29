// based on https://github.com/rust-lang/rust-analyzer/blob/d8887c0758bbd2d5f752d5bd405d4491e90e7ed6/crates/parser/src/tests.rs
use camino::Utf8Path;
use dir_test::{dir_test, Fixture};
use insta::{assert_snapshot, with_settings};
use squawk_parser::{parse, LexedStr};
use std::fmt::Write;
use xshell::{cmd, Shell};

#[dir_test(
    dir: "$CARGO_MANIFEST_DIR/tests/data/ok",
    glob: "*.sql",
)]
fn parser_ok(fixture: Fixture<&str>) {
    let content = fixture.content();
    let absolute_fixture_path = Utf8Path::new(fixture.path());
    let input_file = absolute_fixture_path;
    let test_name = absolute_fixture_path
        .file_name()
        .and_then(|x| x.strip_suffix(".sql"))
        .unwrap();

    let (parsed, errors) = parse_text(content);

    with_settings!({
      omit_expression => true,
      input_file => input_file,
    }, {
      assert_snapshot!(format!("{}_ok", test_name), parsed);
    });

    // We check that all of our tests in `ok` also pass the Postgres parser,
    // if they don't, they should be moved to the `err` directory.
    assert!(
        errors.is_empty(),
        "tests defined in the `ok` can't have parser errors."
    );
    // skipping pg17 specific stuff since our parser isn't using the latest parser
    if !test_name.ends_with("pg17") {
        let pg_result = pg_query::parse(content);
        if let Err(e) = &pg_result {
            assert!(
                &pg_result.is_ok(),
                "tests defined in the `ok` can't have Postgres parser errors. Found {}",
                e
            );
        }
    }
}

#[dir_test(
    dir: "$CARGO_MANIFEST_DIR/tests/data/err",
    glob: "*.sql",
)]
fn parser_err(fixture: Fixture<&str>) {
    let content = fixture.content();
    let absolute_fixture_path = Utf8Path::new(fixture.path());
    let input_file = absolute_fixture_path;
    let test_name = absolute_fixture_path
        .file_name()
        .and_then(|x| x.strip_suffix(".sql"))
        .unwrap();

    let (parsed, errors) = parse_text(content);

    with_settings!({
      omit_expression => true,
      input_file => input_file,
    }, {
      assert_snapshot!(format!("{}_err", test_name), parsed);
    });

    assert!(
        !errors.is_empty(),
        "tests defined in the `err` directory must have parser errors."
    );
}

// 102 failing
#[dir_test(
    dir: "$CARGO_MANIFEST_DIR/tests/data/regression_suite",
    glob: "*.sql",
)]
fn regression_suite(fixture: Fixture<&str>) {
    let content = fixture.content();
    let absolute_fixture_path = Utf8Path::new(fixture.path());
    let input_file = absolute_fixture_path;
    let test_name = absolute_fixture_path
        .file_name()
        .and_then(|x| x.strip_suffix(".sql"))
        .unwrap();

    let (_parsed, errors) = parse_text(content);

    if !errors.is_empty() {
        with_settings!({
          omit_expression => true,
          input_file => input_file
        }, {
          assert_snapshot!(format!("regression_{}", test_name), errors.join(""));
        });
    }
}

// Trying to burn down the errors in the postgres regression suite
#[test]
fn regression_suite_errors() {
    let sh = Shell::new().unwrap();
    sh.change_dir(env!("CARGO_MANIFEST_DIR"));

    let output = cmd!(sh, "rg -c ERROR tests/snapshots")
        .ignore_status()
        .read()
        .expect("Failed to execute ripgrep command");

    let mut out = vec![];
    for l in output.lines() {
        // over other tests that should have errors
        if l.contains("regression") {
            out.push(l);
        }
    }
    out.sort();
    assert_snapshot!(out.join("\n"));
}

fn parse_text(text: &str) -> (String, Vec<std::string::String>) {
    let lexed = LexedStr::new(text);
    let input = lexed.to_input();
    let output = parse(&input);

    let mut buf = String::new();
    let mut errors = Vec::new();
    let mut indent = String::new();
    let mut depth = 0;
    let mut len = 0;
    lexed.intersperse_trivia(&output, &mut |step| match step {
        squawk_parser::StrStep::Token { kind, text } => {
            assert!(depth > 0);
            len += text.len();
            writeln!(buf, "{indent}{kind:?} {text:?}").unwrap();
        }
        squawk_parser::StrStep::Enter { kind } => {
            assert!(depth > 0 || len == 0);
            depth += 1;
            writeln!(buf, "{indent}{kind:?}").unwrap();
            indent.push_str("  ");
        }
        squawk_parser::StrStep::Exit => {
            assert!(depth > 0);
            depth -= 1;
            indent.pop();
            indent.pop();
        }
        squawk_parser::StrStep::Error { msg, pos } => {
            assert!(depth > 0);
            let err = "ERROR";
            errors.push(format!("{err}@{pos}: {msg}\n"));
        }
    });
    assert_eq!(
        len,
        text.len(),
        "didn't parse all text.\nParsed:\n{}\n\nAll:\n{}\n",
        &text[..len],
        text
    );

    for (token, msg) in lexed.errors() {
        let pos = lexed.text_start(token);
        let err = "ERROR";
        errors.push(format!("{err}@{pos}: {msg}\n"));
    }

    if !errors.is_empty() {
        buf.push_str("---\n");
        for e in &errors {
            buf.push_str(&e);
        }
    }
    (buf, errors)
}
