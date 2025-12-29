// based on https://github.com/rust-lang/rust-analyzer/blob/d8887c0758bbd2d5f752d5bd405d4491e90e7ed6/crates/parser/src/tests.rs
use annotate_snippets::{AnnotationKind, Level, Renderer, Snippet, renderer::DecorStyle};
use camino::Utf8Path;
use dir_test::{Fixture, dir_test};
use insta::{assert_snapshot, with_settings};
use squawk_parser::{LexedStr, parse};
use std::fmt::Write;

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
        errors.is_none(),
        "tests defined in the `ok` can't have parser errors."
    );
    // skipping pg17/pg18/pg19 specific stuff since our parser isn't using the latest parser
    if !test_name.ends_with("pg17") && !test_name.ends_with("pg18") && !test_name.ends_with("pg19")
    {
        let pg_result = pg_query::parse(content);
        if let Err(e) = &pg_result {
            assert!(
                &pg_result.is_ok(),
                "tests defined in the `ok` can't have Postgres parser errors. Found {e}",
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
        errors.is_some(),
        "tests defined in the `err` directory must have parser errors."
    );
}

#[dir_test(
    dir: "$CARGO_MANIFEST_DIR/../../postgres/regression_suite",
    glob: "*.sql",
)]
fn regression_suite(fixture: Fixture<&str>) {
    let content = fixture.content();
    let absolute_fixture_path = Utf8Path::new(fixture.path());
    if absolute_fixture_path.to_string().contains("psql") {
        return;
    }
    let input_file = absolute_fixture_path;
    let test_name = absolute_fixture_path
        .file_name()
        .and_then(|x| x.strip_suffix(".sql"))
        .unwrap();

    let (_parsed, errors) = parse_text(content);

    let expect_errors = test_name == "errors";

    let snapshot_name = format!("regression_{test_name}");

    let no_errors = errors.is_none();
    let has_errors = !no_errors;

    with_settings!({
      omit_expression => true,
      input_file => input_file
    }, {
      assert_snapshot!(snapshot_name, errors.unwrap_or_default());
    });

    if expect_errors {
        assert!(
            has_errors,
            "the errors.sql regression test must have errors"
        );
    } else {
        assert!(
            no_errors,
            "tests defined in the regression suite can't have parser errors."
        );
    }
}

fn parse_text(text: &str) -> (String, Option<String>) {
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
            errors.push((pos, msg.to_string()));
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
        errors.push((pos, msg.to_string()));
    }

    let error_message = if !errors.is_empty() {
        errors.sort_by_key(|(pos, _)| *pos);

        let renderer = Renderer::plain().decor_style(DecorStyle::Unicode);

        let mut out = "---\n".to_owned();

        for (pos, msg) in &errors {
            let group = Level::ERROR.primary_title(msg).id("syntax-error").element(
                Snippet::source(text)
                    .fold(true)
                    .annotation(AnnotationKind::Primary.span(*pos..*pos + 1)),
            );
            let rendered = renderer.render(&[group]).to_string();

            out.push_str(&rendered);
            out.push('\n');
        }
        Some(out)
    } else {
        None
    };

    if let Some(error_message) = error_message.clone() {
        buf.push_str(&error_message);
    }

    (buf, error_message)
}
