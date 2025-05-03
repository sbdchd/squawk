// based on https://github.com/rust-lang/rust-analyzer/blob/d8887c0758bbd2d5f752d5bd405d4491e90e7ed6/crates/parser/src/tests.rs
use camino::Utf8Path;
use dir_test::{dir_test, Fixture};
use insta::{assert_snapshot, with_settings};
use std::fmt::Write;

use crate::{parse, LexedStr};

fn parse_text(text: &str) -> (String, bool) {
    let lexed = LexedStr::new(text);
    let input = lexed.to_input();
    let output = parse(&input);

    let mut buf = String::new();
    let mut errors = Vec::new();
    let mut indent = String::new();
    let mut depth = 0;
    let mut len = 0;
    lexed.intersperse_trivia(&output, &mut |step| match step {
        crate::StrStep::Token { kind, text } => {
            assert!(depth > 0);
            len += text.len();
            writeln!(buf, "{indent}{kind:?} {text:?}").unwrap();
        }
        crate::StrStep::Enter { kind } => {
            assert!(depth > 0 || len == 0);
            depth += 1;
            writeln!(buf, "{indent}{kind:?}").unwrap();
            indent.push_str("  ");
        }
        crate::StrStep::Exit => {
            assert!(depth > 0);
            depth -= 1;
            indent.pop();
            indent.pop();
        }
        crate::StrStep::Error { msg, pos } => {
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

    let has_errors = !errors.is_empty();
    if has_errors {
        buf.push_str("---\n");
        for e in errors {
            buf.push_str(&e);
        }
    }
    (buf, has_errors)
}

#[dir_test(
    dir: "$CARGO_MANIFEST_DIR/test_data",
    glob: "**/*.sql",
)]
fn sqltest(fixture: Fixture<&str>) {
    let content = fixture.content();
    let absolute_fixture_path = Utf8Path::new(fixture.path());
    let input_file = absolute_fixture_path;
    let test_name = absolute_fixture_path
        .file_name()
        .and_then(|x| x.strip_suffix(".sql"))
        .unwrap();

    let parent_dir = input_file.parent().and_then(|x| x.file_name()).unwrap();

    let (parsed, has_errors) = parse_text(content);

    with_settings!({
      omit_expression => true,
      input_file => input_file,
    }, {
      assert_snapshot!(format!("{}_{}", test_name, parent_dir), parsed);
    });

    if parent_dir == "ok" {
        // We check that all of our tests in `ok` also pass the Postgres parser,
        // if they don't, they should be moved to the `err` directory.
        assert!(
            !has_errors,
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
    } else {
        assert_eq!(parent_dir, "err");
        assert!(
            has_errors,
            "tests defined in the `err` directory must have parser errors."
        );
    }
}
