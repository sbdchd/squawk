use annotate_snippets::{AnnotationKind, Level, Renderer, Snippet};
use camino::Utf8Path;
use dir_test::{Fixture, dir_test};
use insta::{assert_snapshot, with_settings};
use squawk_fmt::token_compare::assert_no_dropped_tokens;
use squawk_lexer::{Token, TokenKind, tokenize};

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

    let formatted = squawk_fmt::fmt_str(content, Default::default()).unwrap();

    assert_no_dropped_tokens(content, &formatted);
    assert_parses(&formatted);
    assert_no_extra_spaces(&formatted);
    assert_comment_spacing(&formatted);
    assert_eq!(
        squawk_fmt::fmt_str(&formatted, Default::default()).unwrap(),
        formatted,
        "formatting isn't idempotent"
    );
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

    match squawk_fmt::fmt_str(&sql, Default::default()) {
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

#[test]
fn configurable_indent() {
    let sql =
        "select 'a', 'really long string                                                    ';\n";
    let options = squawk_fmt::FormatOptions {
        indent: 4,
        ..Default::default()
    };

    assert_snapshot!(squawk_fmt::fmt_str(sql, options).unwrap(), @"
    select
        'a',
        'really long string                                                    ';
    ");
}

#[test]
fn configurable_width() {
    let sql = "select first_column, second_column;\n";
    let options = squawk_fmt::FormatOptions {
        width: 20,
        ..Default::default()
    };

    assert_snapshot!(squawk_fmt::fmt_str(sql, options).unwrap(), @"
    select
      first_column,
      second_column;
    ");
}

#[test]
fn normalizes_line_endings_inside_block_comments() {
    let sql = "select 1;\r\n/* a\n * comment\n */\nselect 2;\n";
    let expected = "select 1;\r\n/* a\r\n * comment\r\n */\r\nselect 2;\r\n";

    let formatted = squawk_fmt::fmt_str(sql, Default::default()).unwrap();
    assert_eq!(formatted, expected);
    assert_eq!(
        squawk_fmt::fmt_str(&formatted, Default::default()).unwrap(),
        expected
    );
}

#[test]
fn removes_trailing_whitespace_from_comments() {
    let sql = "select 1; -- ok   \n/* a  \n * comment\t\n */\nselect 2;\n";
    let expected = "select 1; -- ok\n/* a\n * comment\n */\nselect 2;\n";

    assert_eq!(
        squawk_fmt::fmt_str(sql, Default::default()).unwrap(),
        expected
    );
}

#[test]
fn preserves_a_leading_bom() {
    let sql = "\u{feff}select   1;\n";
    let expected = "\u{feff}select 1;\n";

    let formatted = squawk_fmt::fmt_str(sql, Default::default()).unwrap();
    assert_no_dropped_tokens(sql, &formatted);
    assert_parses(&formatted);
    assert_eq!(formatted, expected);
    assert_eq!(
        squawk_fmt::fmt_str(&formatted, Default::default()).unwrap(),
        expected
    );
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

fn comment_spacing_error(formatted: &str) -> Option<(std::ops::Range<usize>, &'static str)> {
    let mut offset = 0;
    let mut tokens = tokenize(formatted)
        .map(|Token { kind, len }| {
            let start = offset;
            offset += len as usize;
            (kind, start..offset)
        })
        .filter(|(kind, _)| *kind != TokenKind::Whitespace)
        .peekable();

    let mut previous = None;
    while let Some((kind, span)) = tokens.next() {
        let left = previous.replace((kind, span.end));
        if !matches!(
            kind,
            TokenKind::LineComment | TokenKind::BlockComment { .. }
        ) {
            continue;
        }
        if let Some((previous, previous_end)) = left {
            let touching = previous_end == span.start;
            let must_touch = matches!(kind, TokenKind::BlockComment { .. })
                && matches!(
                    previous,
                    TokenKind::OpenParen | TokenKind::OpenBracket | TokenKind::OpenCurly
                );
            if must_touch && !touching {
                return Some((
                    span,
                    "block comment must touch the opening delimiter on its left",
                ));
            }
            if !must_touch && touching {
                return Some((span, "comment must have whitespace on its left"));
            }
        }
        if matches!(kind, TokenKind::BlockComment { .. })
            && let Some((next, next_span)) = tokens.peek()
        {
            let touching = span.end == next_span.start;
            let must_touch = matches!(
                next,
                TokenKind::CloseParen
                    | TokenKind::CloseBracket
                    | TokenKind::CloseCurly
                    | TokenKind::Comma
                    | TokenKind::Semi
                    | TokenKind::OpenParen
                    | TokenKind::OpenBracket
                    | TokenKind::OpenCurly
            );
            if must_touch && !touching {
                return Some((span, "block comment must touch the delimiter on its right"));
            }
            if !must_touch && touching {
                return Some((span, "block comment must have whitespace on its right"));
            }
        }
    }
    None
}

fn comment_spacing_diagnostic(formatted: &str) -> Option<String> {
    let (span, message) = comment_spacing_error(formatted)?;
    let snippet = Snippet::source(formatted)
        .fold(true)
        .annotation(AnnotationKind::Primary.span(span));
    let group = Level::ERROR.primary_title(message).element(snippet);
    Some(Renderer::plain().render(&[group]).to_string())
}

fn assert_comment_spacing(formatted: &str) {
    if let Some(diagnostic) = comment_spacing_diagnostic(formatted) {
        panic!("{diagnostic}");
    }
}

fn assert_no_extra_spaces(formatted: &str) {
    let mut offset = 0;
    for Token { kind, len } in tokenize(formatted) {
        let start = offset;
        offset += len as usize;
        if kind != TokenKind::Whitespace {
            continue;
        }
        let text = &formatted[start..offset];
        let at_line_start = start == 0 || formatted[..start].ends_with(['\n', '\r']);
        let at_eof = offset == formatted.len();
        if at_line_start || at_eof || text.contains(['\n', '\r']) || text == " " {
            continue;
        }
        let snippet = Snippet::source(formatted)
            .fold(true)
            .annotation(AnnotationKind::Primary.span(start..offset));
        let group = Level::ERROR
            .primary_title(format!(
                "expected a single space between tokens, found {text:?}"
            ))
            .element(snippet);
        panic!("{}", Renderer::plain().render(&[group]));
    }
}

#[cfg(test)]
mod comment_spacing_test {
    use insta::assert_snapshot;

    use crate::{assert_comment_spacing, comment_spacing_diagnostic};

    /// Block & line comments do not need whitespace at file boundaries.
    #[test]
    fn file_boundaries() {
        for sql in [
            "/* foo */",
            "-- foo",
            "/* foo */\nselect 1;\n/* bar */",
            "-- foo\nselect 1;\n-- bar",
        ] {
            assert_comment_spacing(sql);
        }
    }

    /// Trailing line comments must have whitespace on their left.
    #[test]
    fn trailing_line_comments() {
        assert_comment_spacing("select 1; -- comment");
        assert_snapshot!(comment_spacing_diagnostic("select 1;-- comment").unwrap(), @"
    error: comment must have whitespace on its left
      |
    1 | select 1;-- comment
      |          ^^^^^^^^^^
    ");
    }

    /// Block comments must be surrounded with whitespace.
    #[test]
    fn block_comments_surrounded_by_whitespace() {
        for sql in [
            "select 1 /*c*/ + 1;",
            "create function f(v accounts.id /* c */ % /* c */ type) returns int language sql return 1;",
            "table foo /* c */ *;",
            "select * from graph_table(g match (a)-[e] /* c */ - /* c */ > (b) columns (a.id));",
            "select 1 /* c */ ::int;",
            "select (array[1, 2, 3])[1 /* c */ : /* c */ 2];",
        ] {
            assert_comment_spacing(sql);
        }
        assert_snapshot!(comment_spacing_diagnostic("select 1/*c*/ + 1;").unwrap(), @"
    error: comment must have whitespace on its left
      |
    1 | select 1/*c*/ + 1;
      |         ^^^^^
    ");
        assert_snapshot!(comment_spacing_diagnostic("select 1 /*c*/+ 1;").unwrap(), @"
    error: block comment must have whitespace on its right
      |
    1 | select 1 /*c*/+ 1;
      |          ^^^^^
    ");
        assert_snapshot!(comment_spacing_diagnostic("select foo(x)/* c */;").unwrap(), @"
    error: comment must have whitespace on its left
      |
    1 | select foo(x)/* c */;
      |              ^^^^^^^
    ");
        assert_snapshot!(comment_spacing_diagnostic("select foo(x,/* c */ y);").unwrap(), @"
    error: comment must have whitespace on its left
      |
    1 | select foo(x,/* c */ y);
      |              ^^^^^^^
    ");
    }

    /// Block comments must have whitespace between them.
    #[test]
    fn between_block_comments() {
        assert_comment_spacing("select 1 /* foo */ /* bar */;");
        assert_snapshot!(comment_spacing_diagnostic("select 1 /* foo *//* bar */;").unwrap(), @"
    error: block comment must have whitespace on its right
      |
    1 | select 1 /* foo *//* bar */;
      |          ^^^^^^^^^
    ");
    }

    /// Block comments must touch `(`, `[`, `{` on their left.
    #[test]
    fn block_comments_touch_left_delimiters() {
        for sql in [
            "select foo(/* c */ x);",
            "select (array[1, 2, 3])[/* c */ 1];",
            "select * from graph_table(g match (a)->{/* c */ 1, 3}(b) columns (a.id));",
        ] {
            assert_comment_spacing(sql);
        }
        assert_snapshot!(comment_spacing_diagnostic("select foo( /* c */ x);").unwrap(), @"
    error: block comment must touch the opening delimiter on its left
      |
    1 | select foo( /* c */ x);
      |             ^^^^^^^
    ");
        assert_snapshot!(comment_spacing_diagnostic("select (array[1, 2, 3])[ /* c */ 1];").unwrap(), @"
    error: block comment must touch the opening delimiter on its left
      |
    1 | select (array[1, 2, 3])[ /* c */ 1];
      |                          ^^^^^^^
    ");
        assert_snapshot!(comment_spacing_diagnostic("select * from graph_table(g match (a)->{ /* c */ 1, 3}(b) columns (a.id));").unwrap(), @"
    error: block comment must touch the opening delimiter on its left
      |
    1 | select * from graph_table(g match (a)->{ /* c */ 1, 3}(b) columns (a.id));
      |                                          ^^^^^^^
    ");
    }

    /// Block comments must touch `)`, `]`, `}`, `,`, `;`, `(`, `[`, `{` on their right.
    #[test]
    fn block_comments_touch_right_delimiters() {
        for sql in [
            "select foo(x /* c */);",
            "select foo(x /* c */, y);",
            "select 1 /* c */;",
            "create function f /* c */(a int) returns int language sql return 1;",
            "select (array[1, 2, 3]) /* c */[1];",
            "select * from graph_table(g match (a)-> /* c */{1, 3}(b) columns (a.id));",
        ] {
            assert_comment_spacing(sql);
        }
        assert_snapshot!(comment_spacing_diagnostic("select foo(x /* c */ );").unwrap(), @"
    error: block comment must touch the delimiter on its right
      |
    1 | select foo(x /* c */ );
      |              ^^^^^^^
    ");
        assert_snapshot!(comment_spacing_diagnostic("select foo(x /* c */ , y);").unwrap(), @"
    error: block comment must touch the delimiter on its right
      |
    1 | select foo(x /* c */ , y);
      |              ^^^^^^^
    ");
        assert_snapshot!(comment_spacing_diagnostic("select 1 /* c */ ;").unwrap(), @"
    error: block comment must touch the delimiter on its right
      |
    1 | select 1 /* c */ ;
      |          ^^^^^^^
    ");
        assert_snapshot!(comment_spacing_diagnostic("select (array[1, 2, 3])[1 /* c */ ];").unwrap(), @"
    error: block comment must touch the delimiter on its right
      |
    1 | select (array[1, 2, 3])[1 /* c */ ];
      |                           ^^^^^^^
    ");
        assert_snapshot!(comment_spacing_diagnostic("select * from graph_table(g match (a)->{1, 3 /* c */ }(b) columns (a.id));").unwrap(), @"
    error: block comment must touch the delimiter on its right
      |
    1 | select * from graph_table(g match (a)->{1, 3 /* c */ }(b) columns (a.id));
      |                                              ^^^^^^^
    ");
        assert_snapshot!(comment_spacing_diagnostic("create function f /* c */ (a int) returns int language sql return 1;").unwrap(), @"
    error: block comment must touch the delimiter on its right
      |
    1 | create function f /* c */ (a int) returns int language sql return 1;
      |                   ^^^^^^^
    ");
        assert_snapshot!(comment_spacing_diagnostic("select (array[1, 2, 3]) /* c */ [1];").unwrap(), @"
    error: block comment must touch the delimiter on its right
      |
    1 | select (array[1, 2, 3]) /* c */ [1];
      |                         ^^^^^^^
    ");
        assert_snapshot!(comment_spacing_diagnostic("select * from graph_table(g match (a)-> /* c */ {1, 3}(b) columns (a.id));").unwrap(), @"
    error: block comment must touch the delimiter on its right
      |
    1 | select * from graph_table(g match (a)-> /* c */ {1, 3}(b) columns (a.id));
      |                                         ^^^^^^^
    ");
    }
}
