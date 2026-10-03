use camino::Utf8Path;
use dir_test::{Fixture, dir_test};
use insta::{assert_snapshot, with_settings};
use squawk_fmt::validation::assert_valid_format;

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

    assert_valid_format(content, &formatted, Default::default());
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
            assert_valid_format(&sql, &formatted, Default::default());
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

    let formatted = squawk_fmt::fmt_str(sql, options).unwrap();
    assert_valid_format(sql, &formatted, options);
    assert_snapshot!(formatted, @"
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

    let formatted = squawk_fmt::fmt_str(sql, options).unwrap();
    assert_valid_format(sql, &formatted, options);
    assert_snapshot!(formatted, @"
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
    assert_valid_format(sql, &formatted, Default::default());
    assert_eq!(formatted, expected);
}

#[test]
fn removes_trailing_whitespace_from_comments() {
    let sql = "select 1; -- ok   \n/* a  \n * comment\t\n */\nselect 2;\n";
    let expected = "select 1; -- ok\n/* a\n * comment\n */\nselect 2;\n";

    let formatted = squawk_fmt::fmt_str(sql, Default::default()).unwrap();
    assert_valid_format(sql, &formatted, Default::default());
    assert_eq!(formatted, expected);
}

#[test]
fn preserves_a_leading_bom() {
    let sql = "\u{feff}select   1;\n";
    let expected = "\u{feff}select 1;\n";

    let formatted = squawk_fmt::fmt_str(sql, Default::default()).unwrap();
    assert_valid_format(sql, &formatted, Default::default());
    assert_eq!(formatted, expected);
}

#[test]
fn removes_leading_and_trailing_whitespace() {
    for (sql, expected) in [
        ("\n\n\n\n    select 1;\n\n\n", "select 1;\n"),
        ("select 1;  ", "select 1;"),
        ("select 1; -- c\n\n\n", "select 1; -- c\n"),
        ("select 1;\n\n/* c */\n\n", "select 1;\n\n/* c */\n"),
        ("  /* c */ select 1;", "/* c */ select 1;"),
        ("  -- c\nselect 1;", "-- c\nselect 1;"),
        ("\n\n  select 1;", "select 1;"),
        ("\u{feff}  /* c */ select 1;", "\u{feff}/* c */ select 1;"),
    ] {
        let formatted = squawk_fmt::fmt_str(sql, Default::default()).unwrap();
        assert_valid_format(sql, &formatted, Default::default());
        assert_eq!(formatted, expected);
    }
}

#[cfg(test)]
mod comment_spacing_test {
    use insta::assert_snapshot;

    use squawk_fmt::validation::validate_format;

    fn assert_valid(sql: &str) {
        let formatted = squawk_fmt::fmt_str(sql, Default::default()).unwrap();
        validate_format(sql, &formatted, Default::default()).unwrap();
    }

    fn validation_diagnostic(formatted: &str) -> String {
        validate_format(formatted, formatted, Default::default())
            .unwrap_err()
            .to_string()
    }

    /// Block & line comments do not need whitespace at file boundaries.
    #[test]
    fn file_boundaries() {
        assert_valid(
            r"/* foo */
select 1;
/* bar */",
        );
        assert_valid(
            r"-- foo
select 1;
-- bar",
        );
    }

    /// Block and line comments are fine by themselves.
    #[test]
    fn own_line() {
        assert_valid(
            r"
-- foo
select 1;
/* foo */
select 1;",
        );
    }

    #[test]
    fn delimiters_across_line_breaks() {
        assert_valid(
            r"create table t (
  /* a */ a int,
  b int /* b */
);",
        );
    }

    /// Trailing line comments must have whitespace on their left.
    #[test]
    fn trailing_line_comments() {
        assert_valid("select 1; -- comment");
        assert_snapshot!(validation_diagnostic("select 1;-- comment"), @"
    error: comment must have whitespace on its left
      |
    1 | select 1;-- comment
      |          ^^^^^^^^^^
    ");
    }

    /// Block comments must be surrounded with whitespace.
    #[test]
    fn block_comments_surrounded_by_whitespace() {
        assert_valid(
            r"
select 1 /*c*/ + 1;
create function f(v accounts.id /* c */ % /* c */ type) returns int language sql return 1;
table foo /* c */ *;
select * from graph_table(g match (a)-[e] /* c */ - /* c */ > (b) columns (a.id));
select 1 /* c */ ::int;
select (array[1, 2, 3])[1 /* c */ : /* c */ 2];
create function f /* c */ (a int) returns int language sql return 1;
select (array[1, 2, 3]) /* c */ [1];
select * from graph_table(g match (a)-> /* c */ {1, 3}(b) columns (a.id));
",
        );
        assert_snapshot!(validation_diagnostic("select 1/*c*/ + 1;"), @"
    error: comment must have whitespace on its left
      |
    1 | select 1/*c*/ + 1;
      |         ^^^^^
    ");
        assert_snapshot!(validation_diagnostic("select 1 /*c*/+ 1;"), @"
    error: block comment must have whitespace on its right
      |
    1 | select 1 /*c*/+ 1;
      |          ^^^^^
    ");
        assert_snapshot!(validation_diagnostic("select foo(x)/* c */;"), @"
    error: comment must have whitespace on its left
      |
    1 | select foo(x)/* c */;
      |              ^^^^^^^
    ");
        assert_snapshot!(validation_diagnostic("select foo(x,/* c */ y);"), @"
    error: comment must have whitespace on its left
      |
    1 | select foo(x,/* c */ y);
      |              ^^^^^^^
    ");
        assert_snapshot!(validation_diagnostic("create function f /* c */(a int) returns int language sql return 1;"), @"
    error: block comment must have whitespace on its right
      |
    1 | create function f /* c */(a int) returns int language sql return 1;
      |                   ^^^^^^^
    ");
        assert_snapshot!(validation_diagnostic("select (array[1, 2, 3]) /* c */[1];"), @"
    error: block comment must have whitespace on its right
      |
    1 | select (array[1, 2, 3]) /* c */[1];
      |                         ^^^^^^^
    ");
        assert_snapshot!(validation_diagnostic("select * from graph_table(g match (a)-> /* c */{1, 3}(b) columns (a.id));"), @"
    error: block comment must have whitespace on its right
      |
    1 | select * from graph_table(g match (a)-> /* c */{1, 3}(b) columns (a.id));
      |                                         ^^^^^^^
    ");
    }

    /// Block comments must have whitespace between them.
    #[test]
    fn between_block_comments() {
        assert_valid("select 1 /* foo */ /* bar */;");
        assert_snapshot!(validation_diagnostic("select 1 /* foo *//* bar */;"), @"
    error: block comment must have whitespace on its right
      |
    1 | select 1 /* foo *//* bar */;
      |          ^^^^^^^^^
    ");
    }

    /// Block comments must touch `(`, `[`, `{` on their left.
    #[test]
    fn block_comments_touch_left_delimiters() {
        assert_valid(
            r"
select foo(/* c */ x);
select (array[1, 2, 3])[/* c */ 1];
select * from graph_table(g match (a)->{/* c */ 1, 3}(b) columns (a.id));
",
        );
        assert_snapshot!(validation_diagnostic("select foo( /* c */ x);"), @"
    error: block comment must touch the opening delimiter on its left
      |
    1 | select foo( /* c */ x);
      |             ^^^^^^^
    ");
        assert_snapshot!(validation_diagnostic("select (array[1, 2, 3])[ /* c */ 1];"), @"
    error: block comment must touch the opening delimiter on its left
      |
    1 | select (array[1, 2, 3])[ /* c */ 1];
      |                          ^^^^^^^
    ");
        assert_snapshot!(validation_diagnostic("select * from graph_table(g match (a)->{ /* c */ 1, 3}(b) columns (a.id));"), @"
    error: block comment must touch the opening delimiter on its left
      |
    1 | select * from graph_table(g match (a)->{ /* c */ 1, 3}(b) columns (a.id));
      |                                          ^^^^^^^
    ");
    }

    /// Comments only share a line with `(`, `[`, `{` when the contents don't wrap.
    #[test]
    fn opening_delimiter_comments_when_wrapping() {
        assert_valid(
            r"
select foo(/* c */ x);
select foo(
  -- c
  x
);
select foo(
  /* c */
  x
);
select foo(
  /* c */ x,
  y
);
",
        );
        assert_snapshot!(validation_diagnostic("select foo(-- c\n  x);"), @"
        error: comment after an opening delimiter must be on its own line when the contents wrap
          |
        1 | select foo(-- c
          |            ^^^^
        ");
        assert_snapshot!(validation_diagnostic("select foo( -- c\n  x\n);"), @"
        error: comment after an opening delimiter must be on its own line when the contents wrap
          |
        1 | select foo( -- c
          |             ^^^^
        ");
        assert_snapshot!(validation_diagnostic("select foo(/* c */\n  x\n);"), @"
        error: comment after an opening delimiter must be on its own line when the contents wrap
          |
        1 | select foo(/* c */
          |            ^^^^^^^
        ");
    }

    /// Block comments must touch `)`, `]`, `}`, `,`, `;` on their right.
    #[test]
    fn block_comments_touch_right_delimiters() {
        assert_valid(
            r"
select foo(x /* c */);
select foo(x /* c */, y);
select 1 /* c */;
",
        );
        assert_snapshot!(validation_diagnostic("select foo(x /* c */ );"), @"
    error: block comment must touch the delimiter on its right
      |
    1 | select foo(x /* c */ );
      |              ^^^^^^^
    ");
        assert_snapshot!(validation_diagnostic("select foo(x /* c */ , y);"), @"
    error: block comment must touch the delimiter on its right
      |
    1 | select foo(x /* c */ , y);
      |              ^^^^^^^
    ");
        assert_snapshot!(validation_diagnostic("select 1 /* c */ ;"), @"
    error: block comment must touch the delimiter on its right
      |
    1 | select 1 /* c */ ;
      |          ^^^^^^^
    ");
        assert_snapshot!(validation_diagnostic("select (array[1, 2, 3])[1 /* c */ ];"), @"
    error: block comment must touch the delimiter on its right
      |
    1 | select (array[1, 2, 3])[1 /* c */ ];
      |                           ^^^^^^^
    ");
        assert_snapshot!(validation_diagnostic("select * from graph_table(g match (a)->{1, 3 /* c */ }(b) columns (a.id));"), @"
    error: block comment must touch the delimiter on its right
      |
    1 | select * from graph_table(g match (a)->{1, 3 /* c */ }(b) columns (a.id));
      |                                              ^^^^^^^
    ");
    }
}
