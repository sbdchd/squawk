use camino::Utf8Path;
use dir_test::{Fixture, dir_test};
use insta::{assert_snapshot, with_settings};
use squawk_fmt::token_compare::assert_no_dropped_tokens;

#[dir_test(
    dir: "$CARGO_MANIFEST_DIR/tests/before",
    glob: "*.sql",
)]
fn fmt(fixture: Fixture<&str>) {
    let content = fixture.content();
    let absolute_fixture_path = Utf8Path::new(fixture.path());
    let test_name = absolute_fixture_path
        .file_name()
        .and_then(|x| x.strip_suffix(".sql"))
        .unwrap();

    let formatted = squawk_fmt::fmt_str(content).unwrap();

    assert_no_dropped_tokens(content, &formatted);
    assert_parses(&formatted);

    with_settings!({
        omit_expression => true,
        input_file => absolute_fixture_path,
        snapshot_path => "after",
        prepend_module_to_snapshot => false,
    }, {
        assert_snapshot!(test_name, formatted);
    });
}

fn fmt_with_line_ending(line_ending: &str) -> String {
    let sql = [
        "-- a comment",
        "select 1;",
        "",
        "/* a comment",
        " * spanning lines",
        " */",
        "select  'a',  'really long string                                                    ';",
        "",
    ]
    .join(line_ending);

    match squawk_fmt::fmt_str(&sql) {
        Ok(formatted) => {
            assert_no_dropped_tokens(&sql, &formatted);
            assert_parses(&formatted);
            formatted.replace('\r', "<CR>")
        }
        Err(err) => format!("error: {err}"),
    }
}

#[test]
fn fmt_lf_line_endings() {
    assert_snapshot!(fmt_with_line_ending("\n"), @"
    -- a comment
    select 1;

    /* a comment
     * spanning lines
     */
    select
      'a',
      'really long string                                                    ';
    ");
}

#[test]
fn fmt_crlf_line_endings() {
    assert_snapshot!(fmt_with_line_ending("\r\n"), @"
    -- a comment<CR>
    select 1;<CR>
    <CR>
    /* a comment<CR>
     * spanning lines<CR>
     */<CR>
    select<CR>
      'a',<CR>
      'really long string                                                    ';<CR>
    ");
}

#[test]
fn fmt_cr_line_endings() {
    assert_snapshot!(fmt_with_line_ending("\r"), @"-- a comment<CR>select 1;<CR><CR>/* a comment<CR> * spanning lines<CR> */<CR>select<CR>  'a',<CR>  'really long string                                                    ';<CR>");
}

fn assert_parses(formatted: &str) {
    let parse = squawk_syntax::ast::SourceFile::parse(formatted);
    assert!(
        parse.errors().is_empty(),
        "formatted output has syntax errors:\n{}\n\nformatted output:\n{formatted}",
        parse
            .errors()
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n")
    );
}
