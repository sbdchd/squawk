use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

use squawk_syntax::quote::needs_quoting;

use crate::{Edit, Fix, Linter, Rule, Violation};

// via: https://github.com/postgres/postgres/blob/228a1f9542792c6533ef74c2e7aefad0da1d9a7a/src/include/pg_config_manual.h#L39C6-L39C6
const NAMEDATALEN: usize = 64;
const MAX_IDENT_BYTES: usize = NAMEDATALEN - 1;

pub(crate) fn identifier_too_long(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    for name in parse
        .tree()
        .syntax()
        .descendants()
        .filter_map(ast::AnyName::cast)
    {
        check_name(ctx, &name);
    }
}

fn check_name(ctx: &mut Linter, name_like: &impl ast::NameLike) {
    let ident = name_like.text();
    if ident.len() <= MAX_IDENT_BYTES {
        return;
    }

    let fix = truncate(name_like).map(|truncated| {
        Fix::new(
            format!("Rename to `{truncated}`"),
            vec![Edit::replace(name_like.syntax().text_range(), truncated)],
        )
    });

    ctx.report(
        Violation::for_node(
            Rule::IdentifierTooLong,
            format!("`{ident}` is too long and will be truncated to {MAX_IDENT_BYTES} bytes."),
            name_like.syntax(),
        )
        .fix(fix),
    );
}

fn truncate(name_like: &impl ast::NameLike) -> Option<String> {
    let raw = name_like.syntax().text().to_string();
    if has_escaped_quotes(&raw) {
        return None;
    }

    let ident = name_like.text();
    let truncated = &ident[..ident.floor_char_boundary(MAX_IDENT_BYTES)];

    Some(if raw.starts_with('"') || needs_quoting(truncated) {
        format!("\"{truncated}\"")
    } else {
        truncated.to_owned()
    })
}

fn has_escaped_quotes(text: &str) -> bool {
    text.strip_prefix('"')
        .and_then(|t| t.strip_suffix('"'))
        .is_some_and(|text| text.contains(r#""""#))
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::Rule;
    use crate::test_utils::{lint_errors, lint_ok};

    #[test]
    fn create_table_long_name_err() {
        let sql = r#"
create table table_very_long_very_long_very_long_very_long_very_long_very_long (column_very_long_very_long_very_long_very_long_very_long_very_long bigint);
        "#;
        assert_snapshot!(lint_errors(sql, Rule::IdentifierTooLong), @"
        warning[identifier-too-long]: `table_very_long_very_long_very_long_very_long_very_long_very_long` is too long and will be truncated to 63 bytes.
          ╭▸ 
        2 │ create table table_very_long_very_long_very_long_very_long_very_long_very_long (column_very_long_very_long_very_long_very_long_very_lon…
          │              ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
          ╭╴
        2 - create table table_very_long_very_long_very_long_very_long_very_long_very_long (column_very_long_very_long_very_long_very_long_very_long_very_long bigint);
        2 + create table table_very_long_very_long_very_long_very_long_very_long_very_lo (column_very_long_very_long_very_long_very_long_very_long_very_long bigint);
          ╰╴
        warning[identifier-too-long]: `column_very_long_very_long_very_long_very_long_very_long_very_long` is too long and will be truncated to 63 bytes.
          ╭▸ 
        2 │ …ng_very_long_very_long_very_long (column_very_long_very_long_very_long_very_long_very_long_very_long bigint);
          │                                    ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
          ╭╴
        2 - create table table_very_long_very_long_very_long_very_long_very_long_very_long (column_very_long_very_long_very_long_very_long_very_long_very_long bigint);
        2 + create table table_very_long_very_long_very_long_very_long_very_long_very_long (column_very_long_very_long_very_long_very_long_very_long_very_l bigint);
          ╰╴
        ");
    }

    #[test]
    fn create_table_ok() {
        let sql = r#"
create table short_name (col bigint);
        "#;
        lint_ok(sql, Rule::IdentifierTooLong);
    }

    #[test]
    fn create_index_long_name_err() {
        let sql = r#"
create index index_very_long_very_long_very_long_very_long_very_long_very_long on t (col);
        "#;
        assert_snapshot!(lint_errors(sql, Rule::IdentifierTooLong), @"
        warning[identifier-too-long]: `index_very_long_very_long_very_long_very_long_very_long_very_long` is too long and will be truncated to 63 bytes.
          ╭▸ 
        2 │ create index index_very_long_very_long_very_long_very_long_very_long_very_long on t (col);
          │              ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
          ╭╴
        2 - create index index_very_long_very_long_very_long_very_long_very_long_very_long on t (col);
        2 + create index index_very_long_very_long_very_long_very_long_very_long_very_lo on t (col);
          ╰╴
        ");
    }

    #[test]
    fn drop_table_long_name_err() {
        let sql = r#"
drop table table_very_long_very_long_very_long_very_long_very_long_very_long;
        "#;
        assert_snapshot!(lint_errors(sql, Rule::IdentifierTooLong), @"
        warning[identifier-too-long]: `table_very_long_very_long_very_long_very_long_very_long_very_long` is too long and will be truncated to 63 bytes.
          ╭▸ 
        2 │ drop table table_very_long_very_long_very_long_very_long_very_long_very_long;
          │            ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
          ╭╴
        2 - drop table table_very_long_very_long_very_long_very_long_very_long_very_long;
        2 + drop table table_very_long_very_long_very_long_very_long_very_long_very_lo;
          ╰╴
        ");
    }

    #[test]
    fn alter_table_add_column_long_name_err() {
        let sql = r#"
alter table t add column column_very_long_very_long_very_long_very_long_very_long_very_long bigint;
        "#;
        assert_snapshot!(lint_errors(sql, Rule::IdentifierTooLong), @"
        warning[identifier-too-long]: `column_very_long_very_long_very_long_very_long_very_long_very_long` is too long and will be truncated to 63 bytes.
          ╭▸ 
        2 │ alter table t add column column_very_long_very_long_very_long_very_long_very_long_very_long bigint;
          │                          ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
          ╭╴
        2 - alter table t add column column_very_long_very_long_very_long_very_long_very_long_very_long bigint;
        2 + alter table t add column column_very_long_very_long_very_long_very_long_very_long_very_l bigint;
          ╰╴
        ");
    }

    #[test]
    fn quoted_identifier_long_err() {
        let sql = r#"
create table "table_very_long_very_long_very_long_very_long_very_long_very_long" (id bigint);
        "#;
        assert_snapshot!(lint_errors(sql, Rule::IdentifierTooLong), @r#"
        warning[identifier-too-long]: `table_very_long_very_long_very_long_very_long_very_long_very_long` is too long and will be truncated to 63 bytes.
          ╭▸ 
        2 │ create table "table_very_long_very_long_very_long_very_long_very_long_very_long" (id bigint);
          │              ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
          ╭╴
        2 - create table "table_very_long_very_long_very_long_very_long_very_long_very_long" (id bigint);
        2 + create table "table_very_long_very_long_very_long_very_long_very_long_very_lo" (id bigint);
          ╰╴
        "#);
    }

    #[test]
    fn multibyte_emoji_err() {
        // postgres ends up slicing the emoji in two
        let sql = r#"create table "👨‍👩‍👧‍👦👨‍👩‍👧‍👦👨‍👩‍👧‍👦" (id bigint);"#;
        assert_snapshot!(lint_errors(sql, Rule::IdentifierTooLong), @r#"
        warning[identifier-too-long]: `👨👩👧👦👨👩👧👦👨👩👧👦` is too long and will be truncated to 63 bytes.
          ╭▸ 
        1 │ create table "👨👩👧👦👨👩👧👦👨👩👧👦" (id bigint);
          │              ━━━━━━━━━━━━━━━━━━━━━━━━━━
          ╭╴
        1 - create table "👨👩👧👦👨👩👧👦👨👩👧👦" (id bigint);
        1 + create table "👨👩👧👦👨👩👧👦👨👩" (id bigint);
          ╰╴
        "#);
    }

    #[test]
    fn quoted_identifier_with_escaped_quotes_message_unescapes_quotes() {
        let identifier = format!(r#""{}""b""#, "a".repeat(63));
        //                                     ^^ escaped quote
        let sql = format!("create table {identifier} (id bigint);");
        // quoting is complicated so we don't add a fix
        assert_snapshot!(lint_errors(&sql, Rule::IdentifierTooLong), @r#"
        warning[identifier-too-long]: `aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"b` is too long and will be truncated to 63 bytes.
          ╭▸ 
        1 │ create table "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa""b" (id bigint);
          ╰╴             ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
        "#);
    }

    #[test]
    fn exactly_63_bytes_ok() {
        // exactly 63 bytes -- should not trigger
        let sql = r#"
create table table_very_long_very_long_very_long_very_long_very_long_very_lo (id bigint);
-- ^ exactly 63 bytes
        "#;
        lint_ok(sql, Rule::IdentifierTooLong);
    }
}
