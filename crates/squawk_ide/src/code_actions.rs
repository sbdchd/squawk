use rowan::TextSize;
use squawk_linter::Edit;
use squawk_syntax::{
    SyntaxKind, SyntaxNode,
    ast::{self, AstNode},
};

use crate::{generated::keywords::RESERVED_KEYWORDS, offsets::token_from_offset};

#[derive(Debug, Clone)]
pub enum ActionKind {
    QuickFix,
    RefactorRewrite,
}

#[derive(Debug, Clone)]
pub struct CodeAction {
    pub title: String,
    pub edits: Vec<Edit>,
    pub kind: ActionKind,
}

pub fn code_actions(file: ast::SourceFile, offset: TextSize) -> Option<Vec<CodeAction>> {
    let mut actions = vec![];
    rewrite_as_regular_string(&mut actions, &file, offset);
    rewrite_as_dollar_quoted_string(&mut actions, &file, offset);
    remove_else_clause(&mut actions, &file, offset);
    rewrite_table_as_select(&mut actions, &file, offset);
    rewrite_select_as_table(&mut actions, &file, offset);
    quote_identifier(&mut actions, &file, offset);
    unquote_identifier(&mut actions, &file, offset);
    Some(actions)
}

fn rewrite_as_regular_string(
    actions: &mut Vec<CodeAction>,
    file: &ast::SourceFile,
    offset: TextSize,
) -> Option<()> {
    let dollar_string = file
        .syntax()
        .token_at_offset(offset)
        .find(|token| token.kind() == SyntaxKind::DOLLAR_QUOTED_STRING)?;

    let replacement = dollar_quoted_to_string(dollar_string.text())?;
    actions.push(CodeAction {
        title: "Rewrite as regular string".to_owned(),
        edits: vec![Edit::replace(dollar_string.text_range(), replacement)],
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

fn rewrite_as_dollar_quoted_string(
    actions: &mut Vec<CodeAction>,
    file: &ast::SourceFile,
    offset: TextSize,
) -> Option<()> {
    let string = file
        .syntax()
        .token_at_offset(offset)
        .find(|token| token.kind() == SyntaxKind::STRING)?;

    let replacement = string_to_dollar_quoted(string.text())?;
    actions.push(CodeAction {
        title: "Rewrite as dollar-quoted string".to_owned(),
        edits: vec![Edit::replace(string.text_range(), replacement)],
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

fn string_to_dollar_quoted(text: &str) -> Option<String> {
    let normalized = normalize_single_quoted_string(text)?;
    let delimiter = dollar_delimiter(&normalized)?;
    let boundary = format!("${}$", delimiter);
    Some(format!("{boundary}{normalized}{boundary}"))
}

fn dollar_quoted_to_string(text: &str) -> Option<String> {
    debug_assert!(text.starts_with('$'));
    let (delimiter, content) = split_dollar_quoted(text)?;
    let boundary = format!("${}$", delimiter);

    if !text.starts_with(&boundary) || !text.ends_with(&boundary) {
        return None;
    }

    // quotes are escaped by using two of them in Postgres
    let escaped = content.replace('\'', "''");
    Some(format!("'{}'", escaped))
}

fn split_dollar_quoted(text: &str) -> Option<(String, &str)> {
    debug_assert!(text.starts_with('$'));
    let second_dollar = text[1..].find('$')?;
    // the `foo` in `select $foo$bar$foo$`
    let delimiter = &text[1..=second_dollar];
    let boundary = format!("${}$", delimiter);

    if !text.ends_with(&boundary) {
        return None;
    }

    let start = boundary.len();
    let end = text.len().checked_sub(boundary.len())?;
    let content = text.get(start..end)?;
    Some((delimiter.to_owned(), content))
}

fn normalize_single_quoted_string(text: &str) -> Option<String> {
    let body = text.strip_prefix('\'')?.strip_suffix('\'')?;
    return Some(body.replace("''", "'"));
}

fn dollar_delimiter(content: &str) -> Option<String> {
    // We can't safely transform a trailing `$` i.e., `select 'foo $'` with an
    // empty delim, because we'll  `select $$foo $$$` which isn't valid.
    if !content.contains("$$") && !content.ends_with('$') {
        return Some("".to_owned());
    }

    let mut delim = "q".to_owned();
    // don't want to just loop forever
    for idx in 0..10 {
        if !content.contains(&format!("${}$", delim)) {
            return Some(delim);
        }
        delim.push_str(&idx.to_string());
    }
    None
}

fn remove_else_clause(
    actions: &mut Vec<CodeAction>,
    file: &ast::SourceFile,
    offset: TextSize,
) -> Option<()> {
    let else_token = file
        .syntax()
        .token_at_offset(offset)
        .find(|x| x.kind() == SyntaxKind::ELSE_KW)?;
    let parent = else_token.parent()?;
    let else_clause = ast::ElseClause::cast(parent)?;

    let mut edits = vec![];
    edits.push(Edit::delete(else_clause.syntax().text_range()));
    if let Some(token) = else_token.prev_token() {
        if token.kind() == SyntaxKind::WHITESPACE {
            edits.push(Edit::delete(token.text_range()));
        }
    }

    actions.push(CodeAction {
        title: "Remove `else` clause".to_owned(),
        edits,
        kind: ActionKind::RefactorRewrite,
    });
    Some(())
}

fn rewrite_table_as_select(
    actions: &mut Vec<CodeAction>,
    file: &ast::SourceFile,
    offset: TextSize,
) -> Option<()> {
    let token = token_from_offset(file, offset)?;
    let table = token.parent_ancestors().find_map(ast::Table::cast)?;

    let relation_name = table.relation_name()?;
    let table_name = relation_name.syntax().text();

    let replacement = format!("select * from {}", table_name);

    actions.push(CodeAction {
        title: "Rewrite as `select`".to_owned(),
        edits: vec![Edit::replace(table.syntax().text_range(), replacement)],
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

fn rewrite_select_as_table(
    actions: &mut Vec<CodeAction>,
    file: &ast::SourceFile,
    offset: TextSize,
) -> Option<()> {
    let token = token_from_offset(file, offset)?;
    let select = token.parent_ancestors().find_map(ast::Select::cast)?;

    if !can_transform_select_to_table(&select) {
        return None;
    }

    let from_clause = select.from_clause()?;
    let from_item = from_clause.from_items().next()?;

    let table_name = if let Some(name_ref) = from_item.name_ref() {
        name_ref.syntax().text().to_string()
    } else if let Some(field_expr) = from_item.field_expr() {
        field_expr.syntax().text().to_string()
    } else {
        return None;
    };

    let replacement = format!("table {}", table_name);

    actions.push(CodeAction {
        title: "Rewrite as `table`".to_owned(),
        edits: vec![Edit::replace(select.syntax().text_range(), replacement)],
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

/// Returns true if a `select` statement can be safely rewritten as a `table` statement.
///
/// We can only do this when there are no clauses besides the `select` and
/// `from` clause. Additionally, we can only have a table reference in the
/// `from` clause.
/// The `select`'s target list must only be a `*`.
fn can_transform_select_to_table(select: &ast::Select) -> bool {
    if select.with_clause().is_some()
        || select.where_clause().is_some()
        || select.group_by_clause().is_some()
        || select.having_clause().is_some()
        || select.window_clause().is_some()
        || select.order_by_clause().is_some()
        || select.limit_clause().is_some()
        || select.fetch_clause().is_some()
        || select.offset_clause().is_some()
        || select.filter_clause().is_some()
        || select.locking_clauses().next().is_some()
    {
        return false;
    }

    let Some(select_clause) = select.select_clause() else {
        return false;
    };

    if select_clause.distinct_clause().is_some() {
        return false;
    }

    let Some(target_list) = select_clause.target_list() else {
        return false;
    };

    let mut targets = target_list.targets();
    let Some(target) = targets.next() else {
        return false;
    };

    if targets.next().is_some() {
        return false;
    }

    // only want to support: `select *`
    if target.expr().is_some() || target.star_token().is_none() {
        return false;
    }

    let Some(from_clause) = select.from_clause() else {
        return false;
    };

    let mut from_items = from_clause.from_items();
    let Some(from_item) = from_items.next() else {
        return false;
    };

    // only can have one from item & no join exprs
    if from_items.next().is_some() || from_clause.join_exprs().next().is_some() {
        return false;
    }

    if from_item.alias().is_some()
        || from_item.tablesample_clause().is_some()
        || from_item.only_token().is_some()
        || from_item.lateral_token().is_some()
        || from_item.star_token().is_some()
        || from_item.call_expr().is_some()
        || from_item.paren_select().is_some()
        || from_item.json_table().is_some()
        || from_item.xml_table().is_some()
        || from_item.cast_expr().is_some()
    {
        return false;
    }

    // only want table refs
    from_item.name_ref().is_some() || from_item.field_expr().is_some()
}

fn quote_identifier(
    actions: &mut Vec<CodeAction>,
    file: &ast::SourceFile,
    offset: TextSize,
) -> Option<()> {
    let token = token_from_offset(file, offset)?;
    let parent = token.parent()?;

    let name_node = if let Some(name) = ast::Name::cast(parent.clone()) {
        name.syntax().clone()
    } else if let Some(name_ref) = ast::NameRef::cast(parent) {
        name_ref.syntax().clone()
    } else {
        return None;
    };

    let text = name_node.text().to_string();

    if text.starts_with('"') {
        return None;
    }

    let quoted = format!(r#""{}""#, text.to_lowercase());

    actions.push(CodeAction {
        title: "Quote identifier".to_owned(),
        edits: vec![Edit::replace(name_node.text_range(), quoted)],
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

fn unquote_identifier(
    actions: &mut Vec<CodeAction>,
    file: &ast::SourceFile,
    offset: TextSize,
) -> Option<()> {
    let token = token_from_offset(file, offset)?;
    let parent = token.parent()?;

    let name_node = if let Some(name) = ast::Name::cast(parent.clone()) {
        name.syntax().clone()
    } else if let Some(name_ref) = ast::NameRef::cast(parent) {
        name_ref.syntax().clone()
    } else {
        return None;
    };

    let unquoted = unquote(&name_node)?;

    actions.push(CodeAction {
        title: "Unquote identifier".to_owned(),
        edits: vec![Edit::replace(name_node.text_range(), unquoted)],
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

fn unquote(node: &SyntaxNode) -> Option<String> {
    let text = node.text().to_string();

    if !text.starts_with('"') || !text.ends_with('"') {
        return None;
    }

    let text = &text[1..text.len() - 1];

    if is_reserved_word(text) {
        return None;
    }

    if text.is_empty() {
        return None;
    }

    let mut chars = text.chars();

    // see: https://www.postgresql.org/docs/18/sql-syntax-lexical.html#SQL-SYNTAX-IDENTIFIERS
    match chars.next() {
        Some(c) if c.is_lowercase() || c == '_' => {}
        _ => return None,
    }

    for c in chars {
        if c.is_lowercase() || c.is_ascii_digit() || c == '_' || c == '$' {
            continue;
        }
        return None;
    }

    Some(text.to_string())
}

fn is_reserved_word(text: &str) -> bool {
    RESERVED_KEYWORDS
        .binary_search(&text.to_lowercase().as_str())
        .is_ok()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils::fixture;
    use insta::assert_snapshot;
    use rowan::TextSize;
    use squawk_syntax::ast;

    fn apply_code_action(
        f: impl Fn(&mut Vec<CodeAction>, &ast::SourceFile, TextSize) -> Option<()>,
        sql: &str,
    ) -> String {
        let (mut offset, sql) = fixture(sql);
        let parse = ast::SourceFile::parse(&sql);
        assert_eq!(parse.errors(), vec![]);
        let file: ast::SourceFile = parse.tree();

        offset = offset.checked_sub(1.into()).unwrap_or_default();

        let mut actions = vec![];
        f(&mut actions, &file, offset);

        assert!(
            !actions.is_empty(),
            "We should always have actions for `apply_code_action`. If you want to ensure there are no actions, use `code_action_not_applicable` instead."
        );

        let action = &actions[0];
        let mut result = sql.clone();

        let mut edits = action.edits.clone();
        edits.sort_by_key(|e| e.text_range.start());
        check_overlap(&edits);
        edits.reverse();

        for edit in edits {
            let start: usize = edit.text_range.start().into();
            let end: usize = edit.text_range.end().into();
            let replacement = edit.text.as_deref().unwrap_or("");
            result.replace_range(start..end, replacement);
        }

        let reparse = ast::SourceFile::parse(&result);
        assert_eq!(
            reparse.errors(),
            vec![],
            "Code actions shouldn't cause syntax errors"
        );

        result
    }

    // There's an invariant where the edits can't overlap.
    // For example, if we have an edit that deletes the full `else clause` and
    // another edit that deletes the `else` keyword and they overlap, then
    // vscode doesn't surface the code action.
    fn check_overlap(edits: &[Edit]) {
        for (edit_i, edit_j) in edits.iter().zip(edits.iter().skip(1)) {
            if let Some(intersection) = edit_i.text_range.intersect(edit_j.text_range) {
                assert!(
                    intersection.is_empty(),
                    "Edit ranges must not overlap: {:?} and {:?} intersect at {:?}",
                    edit_i.text_range,
                    edit_j.text_range,
                    intersection
                );
            }
        }
    }

    fn code_action_not_applicable(
        f: impl Fn(&mut Vec<CodeAction>, &ast::SourceFile, TextSize) -> Option<()>,
        sql: &str,
    ) -> bool {
        let (offset, sql) = fixture(sql);
        let parse = ast::SourceFile::parse(&sql);
        assert_eq!(parse.errors(), vec![]);
        let file: ast::SourceFile = parse.tree();

        let mut actions = vec![];
        f(&mut actions, &file, offset);
        actions.is_empty()
    }

    #[test]
    fn remove_else_clause_() {
        assert_snapshot!(apply_code_action(
            remove_else_clause,
            "select case x when true then 1 else$0 2 end;"),
            @"select case x when true then 1 end;"
        );
    }

    #[test]
    fn remove_else_clause_before_token() {
        assert_snapshot!(apply_code_action(
            remove_else_clause,
            "select case x when true then 1 e$0lse 2 end;"),
            @"select case x when true then 1 end;"
        );
    }

    #[test]
    fn remove_else_clause_not_applicable() {
        assert!(code_action_not_applicable(
            remove_else_clause,
            "select case x when true then 1 else 2 end$0;"
        ));
    }

    #[test]
    fn rewrite_string() {
        assert_snapshot!(apply_code_action(
            rewrite_as_dollar_quoted_string,
            "select 'fo$0o';"),
            @"select $$foo$$;"
        );
    }

    #[test]
    fn rewrite_string_with_single_quote() {
        assert_snapshot!(apply_code_action(
            rewrite_as_dollar_quoted_string,
            "select 'it''s$0 nice';"),
            @"select $$it's nice$$;"
        );
    }

    #[test]
    fn rewrite_string_with_dollar_signs() {
        assert_snapshot!(apply_code_action(
            rewrite_as_dollar_quoted_string,
            "select 'foo $$ ba$0r';"),
            @"select $q$foo $$ bar$q$;"
        );
    }

    #[test]
    fn rewrite_string_when_trailing_dollar() {
        assert_snapshot!(apply_code_action(
            rewrite_as_dollar_quoted_string,
            "select 'foo $'$0;"),
            @"select $q$foo $$q$;"
        );
    }

    #[test]
    fn rewrite_string_not_applicable() {
        assert!(code_action_not_applicable(
            rewrite_as_dollar_quoted_string,
            "select 1 + $0 2;"
        ));
    }

    #[test]
    fn rewrite_prefix_string_not_applicable() {
        assert!(code_action_not_applicable(
            rewrite_as_dollar_quoted_string,
            "select b'foo$0';"
        ));
    }

    #[test]
    fn rewrite_dollar_string() {
        assert_snapshot!(apply_code_action(
            rewrite_as_regular_string,
            "select $$fo$0o$$;"),
            @"select 'foo';"
        );
    }

    #[test]
    fn rewrite_dollar_string_with_tag() {
        assert_snapshot!(apply_code_action(
            rewrite_as_regular_string,
            "select $tag$fo$0o$tag$;"),
            @"select 'foo';"
        );
    }

    #[test]
    fn rewrite_dollar_string_with_quote() {
        assert_snapshot!(apply_code_action(
            rewrite_as_regular_string,
            "select $$it'$0s fine$$;"),
            @"select 'it''s fine';"
        );
    }

    #[test]
    fn rewrite_dollar_string_not_applicable() {
        assert!(code_action_not_applicable(
            rewrite_as_regular_string,
            "select 'foo$0';"
        ));
    }

    #[test]
    fn rewrite_table_as_select_simple() {
        assert_snapshot!(apply_code_action(
            rewrite_table_as_select,
            "tab$0le foo;"),
            @"select * from foo;"
        );
    }

    #[test]
    fn rewrite_table_as_select_qualified() {
        assert_snapshot!(apply_code_action(
            rewrite_table_as_select,
            "ta$0ble schema.foo;"),
            @"select * from schema.foo;"
        );
    }

    #[test]
    fn rewrite_table_as_select_after_keyword() {
        assert_snapshot!(apply_code_action(
            rewrite_table_as_select,
            "table$0 bar;"),
            @"select * from bar;"
        );
    }

    #[test]
    fn rewrite_table_as_select_on_table_name() {
        assert_snapshot!(apply_code_action(
            rewrite_table_as_select,
            "table fo$0o;"),
            @"select * from foo;"
        );
    }

    #[test]
    fn rewrite_table_as_select_not_applicable() {
        assert!(code_action_not_applicable(
            rewrite_table_as_select,
            "select * from foo$0;"
        ));
    }

    #[test]
    fn rewrite_select_as_table_simple() {
        assert_snapshot!(apply_code_action(
            rewrite_select_as_table,
            "sel$0ect * from foo;"),
            @"table foo;"
        );
    }

    #[test]
    fn rewrite_select_as_table_qualified() {
        assert_snapshot!(apply_code_action(
            rewrite_select_as_table,
            "select * from sch$0ema.foo;"),
            @"table schema.foo;"
        );
    }

    #[test]
    fn rewrite_select_as_table_on_star() {
        assert_snapshot!(apply_code_action(
            rewrite_select_as_table,
            "select $0* from bar;"),
            @"table bar;"
        );
    }

    #[test]
    fn rewrite_select_as_table_on_from() {
        assert_snapshot!(apply_code_action(
            rewrite_select_as_table,
            "select * fr$0om baz;"),
            @"table baz;"
        );
    }

    #[test]
    fn rewrite_select_as_table_not_applicable_with_where() {
        assert!(code_action_not_applicable(
            rewrite_select_as_table,
            "select * from foo$0 where x = 1;"
        ));
    }

    #[test]
    fn rewrite_select_as_table_not_applicable_with_order_by() {
        assert!(code_action_not_applicable(
            rewrite_select_as_table,
            "select * from foo$0 order by x;"
        ));
    }

    #[test]
    fn rewrite_select_as_table_not_applicable_with_limit() {
        assert!(code_action_not_applicable(
            rewrite_select_as_table,
            "select * from foo$0 limit 10;"
        ));
    }

    #[test]
    fn rewrite_select_as_table_not_applicable_with_distinct() {
        assert!(code_action_not_applicable(
            rewrite_select_as_table,
            "select distinct * from foo$0;"
        ));
    }

    #[test]
    fn rewrite_select_as_table_not_applicable_with_columns() {
        assert!(code_action_not_applicable(
            rewrite_select_as_table,
            "select id, name from foo$0;"
        ));
    }

    #[test]
    fn rewrite_select_as_table_not_applicable_with_join() {
        assert!(code_action_not_applicable(
            rewrite_select_as_table,
            "select * from foo$0 join bar on foo.id = bar.id;"
        ));
    }

    #[test]
    fn rewrite_select_as_table_not_applicable_with_alias() {
        assert!(code_action_not_applicable(
            rewrite_select_as_table,
            "select * from foo$0 f;"
        ));
    }

    #[test]
    fn rewrite_select_as_table_not_applicable_with_multiple_tables() {
        assert!(code_action_not_applicable(
            rewrite_select_as_table,
            "select * from foo$0, bar;"
        ));
    }

    #[test]
    fn rewrite_select_as_table_not_applicable_on_table() {
        assert!(code_action_not_applicable(
            rewrite_select_as_table,
            "table foo$0;"
        ));
    }

    #[test]
    fn quote_identifier_on_name_ref() {
        assert_snapshot!(apply_code_action(
            quote_identifier,
            "select x$0 from t;"),
            @r#"select "x" from t;"#
        );
    }

    #[test]
    fn quote_identifier_on_name() {
        assert_snapshot!(apply_code_action(
            quote_identifier,
            "create table T(X$0 int);"),
            @r#"create table T("x" int);"#
        );
    }

    #[test]
    fn quote_identifier_lowercases() {
        assert_snapshot!(apply_code_action(
            quote_identifier,
            "create table T(COL$0 int);"),
            @r#"create table T("col" int);"#
        );
    }

    #[test]
    fn quote_identifier_not_applicable_when_already_quoted() {
        assert!(code_action_not_applicable(
            quote_identifier,
            r#"select "x"$0 from t;"#
        ));
    }

    #[test]
    fn quote_identifier_not_applicable_on_select_keyword() {
        assert!(code_action_not_applicable(
            quote_identifier,
            "sel$0ect x from t;"
        ));
    }

    #[test]
    fn quote_identifier_on_keyword_column_name() {
        assert_snapshot!(apply_code_action(
            quote_identifier,
            "select te$0xt from t;"),
            @r#"select "text" from t;"#
        );
    }

    #[test]
    fn quote_identifier_example_select() {
        assert_snapshot!(apply_code_action(
            quote_identifier,
            "select x$0 from t;"),
            @r#"select "x" from t;"#
        );
    }

    #[test]
    fn quote_identifier_example_create_table() {
        assert_snapshot!(apply_code_action(
            quote_identifier,
            "create table T(X$0 int);"),
            @r#"create table T("x" int);"#
        );
    }

    #[test]
    fn unquote_identifier_simple() {
        assert_snapshot!(apply_code_action(
            unquote_identifier,
            r#"select "x"$0 from t;"#),
            @"select x from t;"
        );
    }

    #[test]
    fn unquote_identifier_with_underscore() {
        assert_snapshot!(apply_code_action(
            unquote_identifier,
            r#"select "user_id"$0 from t;"#),
            @"select user_id from t;"
        );
    }

    #[test]
    fn unquote_identifier_with_digits() {
        assert_snapshot!(apply_code_action(
            unquote_identifier,
            r#"select "x123"$0 from t;"#),
            @"select x123 from t;"
        );
    }

    #[test]
    fn unquote_identifier_with_dollar() {
        assert_snapshot!(apply_code_action(
            unquote_identifier,
            r#"select "my_table$1"$0 from t;"#),
            @"select my_table$1 from t;"
        );
    }

    #[test]
    fn unquote_identifier_starts_with_underscore() {
        assert_snapshot!(apply_code_action(
            unquote_identifier,
            r#"select "_col"$0 from t;"#),
            @"select _col from t;"
        );
    }

    #[test]
    fn unquote_identifier_starts_with_unicode() {
        assert_snapshot!(apply_code_action(
            unquote_identifier,
            r#"select "é"$0 from t;"#),
            @"select é from t;"
        );
    }

    #[test]
    fn unquote_identifier_not_applicable() {
        // upper case
        assert!(code_action_not_applicable(
            unquote_identifier,
            r#"select "X"$0 from t;"#
        ));
        // upper case
        assert!(code_action_not_applicable(
            unquote_identifier,
            r#"select "Foo"$0 from t;"#
        ));
        // dash
        assert!(code_action_not_applicable(
            unquote_identifier,
            r#"select "my-col"$0 from t;"#
        ));
        // leading digits
        assert!(code_action_not_applicable(
            unquote_identifier,
            r#"select "123"$0 from t;"#
        ));
        // space
        assert!(code_action_not_applicable(
            unquote_identifier,
            r#"select "foo bar"$0 from t;"#
        ));
        // quotes
        assert!(code_action_not_applicable(
            unquote_identifier,
            r#"select "foo""bar"$0 from t;"#
        ));
        // already unquoted
        assert!(code_action_not_applicable(
            unquote_identifier,
            "select x$0 from t;"
        ));
        // brackets
        assert!(code_action_not_applicable(
            unquote_identifier,
            r#"select "my[col]"$0 from t;"#
        ));
        // curly brackets
        assert!(code_action_not_applicable(
            unquote_identifier,
            r#"select "my{}"$0 from t;"#
        ));
        // reserved word
        assert!(code_action_not_applicable(
            unquote_identifier,
            r#"select "select"$0 from t;"#
        ));
    }

    #[test]
    fn unquote_identifier_on_name() {
        assert_snapshot!(apply_code_action(
            unquote_identifier,
            r#"create table T("x"$0 int);"#),
            @"create table T(x int);"
        );
    }
}
