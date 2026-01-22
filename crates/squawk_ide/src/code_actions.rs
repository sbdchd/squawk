use itertools::Itertools;
use rowan::{TextRange, TextSize};
use squawk_linter::Edit;
use squawk_syntax::{
    SyntaxKind, SyntaxToken,
    ast::{self, AstNode},
};
use std::iter;

use crate::{
    binder,
    column_name::ColumnName,
    offsets::token_from_offset,
    quote::{quote_column_alias, unquote_ident},
    symbols::Name,
};

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
    rewrite_values_as_select(&mut actions, &file, offset);
    rewrite_select_as_values(&mut actions, &file, offset);
    add_schema(&mut actions, &file, offset);
    quote_identifier(&mut actions, &file, offset);
    unquote_identifier(&mut actions, &file, offset);
    add_explicit_alias(&mut actions, &file, offset);
    remove_redundant_alias(&mut actions, &file, offset);
    rewrite_cast_to_double_colon(&mut actions, &file, offset);
    rewrite_double_colon_to_cast(&mut actions, &file, offset);
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

    let unquoted = unquote_ident(&name_node)?;

    actions.push(CodeAction {
        title: "Unquote identifier".to_owned(),
        edits: vec![Edit::replace(name_node.text_range(), unquoted)],
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

// Postgres docs call these output names.
// Postgres' parser calls this a column label.
// Third-party docs call these aliases, so going with that.
fn add_explicit_alias(
    actions: &mut Vec<CodeAction>,
    file: &ast::SourceFile,
    offset: TextSize,
) -> Option<()> {
    let token = token_from_offset(file, offset)?;
    let target = token.parent_ancestors().find_map(ast::Target::cast)?;

    if target.as_name().is_some() {
        return None;
    }

    if let Some(ast::Expr::FieldExpr(field_expr)) = target.expr()
        && field_expr.star_token().is_some()
    {
        return None;
    }

    let alias = ColumnName::from_target(target.clone()).and_then(|c| c.0.to_string())?;

    let expr_end = target.expr().map(|e| e.syntax().text_range().end())?;

    let quoted_alias = quote_column_alias(&alias);
    // Postgres docs recommend either using `as` or quoting the name. I think
    // `as` looks a bit nicer.
    let replacement = format!(" as {}", quoted_alias);

    actions.push(CodeAction {
        title: "Add explicit alias".to_owned(),
        edits: vec![Edit::insert(replacement, expr_end)],
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

fn remove_redundant_alias(
    actions: &mut Vec<CodeAction>,
    file: &ast::SourceFile,
    offset: TextSize,
) -> Option<()> {
    let token = token_from_offset(file, offset)?;
    let target = token.parent_ancestors().find_map(ast::Target::cast)?;

    let as_name = target.as_name()?;
    let (inferred_column, _) = ColumnName::inferred_from_target(target.clone())?;
    let inferred_column_alias = inferred_column.to_string()?;

    let alias = as_name.name()?;

    if Name::from_node(&alias) != Name::from_string(inferred_column_alias) {
        return None;
    }

    // TODO:
    // This lets use remove any whitespace so we don't end up with:
    //   select x as x, b from t;
    // becoming
    //   select x , b from t;
    // but we probably want a better way to express this.
    // Maybe a "Remove preceding whitespace" style option for edits.
    let expr_end = target.expr()?.syntax().text_range().end();
    let alias_end = as_name.syntax().text_range().end();

    actions.push(CodeAction {
        title: "Remove redundant alias".to_owned(),
        edits: vec![Edit::delete(TextRange::new(expr_end, alias_end))],
        kind: ActionKind::QuickFix,
    });

    Some(())
}

fn add_schema(
    actions: &mut Vec<CodeAction>,
    file: &ast::SourceFile,
    offset: TextSize,
) -> Option<()> {
    let token = token_from_offset(file, offset)?;
    let range = token.parent_ancestors().find_map(|node| {
        if let Some(path) = ast::Path::cast(node.clone()) {
            if path.qualifier().is_some() {
                return None;
            }
            return Some(path.syntax().text_range());
        }
        if let Some(from_item) = ast::FromItem::cast(node.clone()) {
            let name_ref = from_item.name_ref()?;
            return Some(name_ref.syntax().text_range());
        }
        if let Some(call_expr) = ast::CallExpr::cast(node) {
            let ast::Expr::NameRef(name_ref) = call_expr.expr()? else {
                return None;
            };
            return Some(name_ref.syntax().text_range());
        }
        None
    })?;

    if !range.contains(offset) {
        return None;
    }

    let position = token.text_range().start();
    let binder = binder::bind(file);
    let schema = binder.search_path_at(position).first()?.to_string();
    let replacement = format!("{}.", schema);

    actions.push(CodeAction {
        title: "Add schema".to_owned(),
        edits: vec![Edit::insert(replacement, position)],
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

fn rewrite_cast_to_double_colon(
    actions: &mut Vec<CodeAction>,
    file: &ast::SourceFile,
    offset: TextSize,
) -> Option<()> {
    let token = token_from_offset(file, offset)?;
    let cast_expr = token.parent_ancestors().find_map(ast::CastExpr::cast)?;

    if cast_expr.colon_colon().is_some() {
        return None;
    }

    let expr = cast_expr.expr()?;
    let ty = cast_expr.ty()?;

    let expr_text = expr.syntax().text();
    let type_text = ty.syntax().text();

    let replacement = format!("{}::{}", expr_text, type_text);

    actions.push(CodeAction {
        title: "Rewrite as cast operator `::`".to_owned(),
        edits: vec![Edit::replace(cast_expr.syntax().text_range(), replacement)],
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

fn rewrite_double_colon_to_cast(
    actions: &mut Vec<CodeAction>,
    file: &ast::SourceFile,
    offset: TextSize,
) -> Option<()> {
    let token = token_from_offset(file, offset)?;
    let cast_expr = token.parent_ancestors().find_map(ast::CastExpr::cast)?;

    if cast_expr.cast_token().is_some() {
        return None;
    }

    let expr = cast_expr.expr()?;
    let ty = cast_expr.ty()?;

    let expr_text = expr.syntax().text();
    let type_text = ty.syntax().text();

    let replacement = format!("cast({} as {})", expr_text, type_text);

    actions.push(CodeAction {
        title: "Rewrite as cast function `cast()`".to_owned(),
        edits: vec![Edit::replace(cast_expr.syntax().text_range(), replacement)],
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

fn rewrite_values_as_select(
    actions: &mut Vec<CodeAction>,
    file: &ast::SourceFile,
    offset: TextSize,
) -> Option<()> {
    let token = token_from_offset(file, offset)?;
    let values = token.parent_ancestors().find_map(ast::Values::cast)?;

    let value_token_start = values.values_token().map(|x| x.text_range().start())?;
    let values_end = values.syntax().text_range().end();
    // `values` but we skip over the possibly preceeding CTE
    let values_range = TextRange::new(value_token_start, values_end);

    let mut rows = values.row_list()?.rows();

    let first_targets: Vec<_> = rows
        .next()?
        .exprs()
        .enumerate()
        .map(|(idx, expr)| format!("{} as column{}", expr.syntax().text(), idx + 1))
        .collect();

    if first_targets.is_empty() {
        return None;
    }

    let mut select_parts = vec![format!("select {}", first_targets.join(", "))];

    for row in rows {
        let row_targets = row
            .exprs()
            .map(|e| e.syntax().text().to_string())
            .join(", ");
        if row_targets.is_empty() {
            return None;
        }
        select_parts.push(format!("union all\nselect {}", row_targets));
    }

    let select_stmt = select_parts.join("\n");

    actions.push(CodeAction {
        title: "Rewrite as `select`".to_owned(),
        edits: vec![Edit::replace(values_range, select_stmt)],
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

fn is_values_row_column_name(target: &ast::Target, idx: usize) -> bool {
    let Some(as_name) = target.as_name() else {
        return false;
    };
    let Some(name) = as_name.name() else {
        return false;
    };
    let expected = format!("column{}", idx + 1);
    if Name::from_node(&name) != Name::from_string(expected) {
        return false;
    }
    true
}

enum SelectContext {
    Compound(ast::CompoundSelect),
    Single(ast::Select),
}

impl SelectContext {
    fn iter(&self) -> Option<Box<dyn Iterator<Item = ast::Select>>> {
        // Ideally we'd have something like Python's `yield` and `yield from`
        // but instead we have to do all of this to avoid creating some temp
        // vecs
        fn variant_iter(
            variant: ast::SelectVariant,
        ) -> Option<Box<dyn Iterator<Item = ast::Select>>> {
            match variant {
                ast::SelectVariant::Select(select) => Some(Box::new(iter::once(select))),
                ast::SelectVariant::CompoundSelect(compound) => compound_iter(&compound),
                ast::SelectVariant::ParenSelect(_)
                | ast::SelectVariant::SelectInto(_)
                | ast::SelectVariant::Table(_)
                | ast::SelectVariant::Values(_) => None,
            }
        }

        fn compound_iter(
            node: &ast::CompoundSelect,
        ) -> Option<Box<dyn Iterator<Item = ast::Select>>> {
            let lhs_iter = node
                .lhs()
                .map(variant_iter)
                .unwrap_or_else(|| Some(Box::new(iter::empty())))?;
            let rhs_iter = node
                .rhs()
                .map(variant_iter)
                .unwrap_or_else(|| Some(Box::new(iter::empty())))?;
            Some(Box::new(lhs_iter.chain(rhs_iter)))
        }

        match self {
            SelectContext::Compound(compound) => compound_iter(compound),
            SelectContext::Single(select) => Some(Box::new(iter::once(select.clone()))),
        }
    }
}

fn rewrite_select_as_values(
    actions: &mut Vec<CodeAction>,
    file: &ast::SourceFile,
    offset: TextSize,
) -> Option<()> {
    let token = token_from_offset(file, offset)?;

    let parent = find_select_parent(token)?;

    let mut selects = parent.iter()?.peekable();
    let select_token_start = selects
        .peek()?
        .select_clause()
        .and_then(|x| x.select_token())
        .map(|x| x.text_range().start())?;

    let mut rows = vec![];
    for (idx, select) in selects.enumerate() {
        let exprs: Vec<String> = select
            .select_clause()?
            .target_list()?
            .targets()
            .enumerate()
            .map(|(i, t)| {
                if idx != 0 || is_values_row_column_name(&t, i) {
                    t.expr().map(|expr| expr.syntax().text().to_string())
                } else {
                    None
                }
            })
            .collect::<Option<_>>()?;

        if exprs.is_empty() {
            return None;
        }

        rows.push(format!("({})", exprs.join(", ")));
    }

    let values_stmt = format!("values {}", rows.join(", "));

    let select_end = match &parent {
        SelectContext::Compound(compound) => compound.syntax().text_range().end(),
        SelectContext::Single(select) => select.syntax().text_range().end(),
    };
    let select_range = TextRange::new(select_token_start, select_end);

    actions.push(CodeAction {
        title: "Rewrite as `values`".to_owned(),
        edits: vec![Edit::replace(select_range, values_stmt)],
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

fn find_select_parent(token: SyntaxToken) -> Option<SelectContext> {
    let mut found_select = None;
    let mut found_compound = None;
    for node in token.parent_ancestors() {
        if let Some(compound_select) = ast::CompoundSelect::cast(node.clone()) {
            if compound_select.union_token().is_some() && compound_select.all_token().is_some() {
                found_compound = Some(SelectContext::Compound(compound_select));
            } else {
                break;
            }
        }
        if found_select.is_none()
            && let Some(select) = ast::Select::cast(node)
        {
            found_select = Some(SelectContext::Single(select));
        }
    }
    found_compound.or(found_select)
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
    fn add_schema_simple() {
        assert_snapshot!(apply_code_action(
            add_schema,
            "create table t$0(a text, b int);"),
            @"create table public.t(a text, b int);"
        );
    }

    #[test]
    fn add_schema_create_foreign_table() {
        assert_snapshot!(apply_code_action(
            add_schema,
            "create foreign table t$0(a text, b int) server foo;"),
            @"create foreign table public.t(a text, b int) server foo;"
        );
    }

    #[test]
    fn add_schema_create_function() {
        assert_snapshot!(apply_code_action(
            add_schema,
            "create function f$0() returns int8\n  as 'select 1'\n  language sql;"),
            @"create function public.f() returns int8
  as 'select 1'
  language sql;"
        );
    }

    #[test]
    fn add_schema_create_type() {
        assert_snapshot!(apply_code_action(
            add_schema,
            "create type t$0 as enum ();"),
            @"create type public.t as enum ();"
        );
    }

    #[test]
    fn add_schema_table_stmt() {
        assert_snapshot!(apply_code_action(
            add_schema,
            "table t$0;"),
            @"table public.t;"
        );
    }

    #[test]
    fn add_schema_select_from() {
        assert_snapshot!(apply_code_action(
            add_schema,
            "create table t(a text, b int);
        select t from t$0;"),
            @"create table t(a text, b int);
        select t from public.t;"
        );
    }

    #[test]
    fn add_schema_select_table_value() {
        // we can't insert the schema here because:
        // `select public.t from t` isn't valid
        assert!(code_action_not_applicable(
            add_schema,
            "create table t(a text, b int);
        select t$0 from t;"
        ));
    }

    #[test]
    fn add_schema_select_unqualified_column() {
        // not applicable since we don't have the table name set
        // we'll have another quick action to insert table names
        assert!(code_action_not_applicable(
            add_schema,
            "create table t(a text, b int);
        select a$0 from t;"
        ));
    }

    #[test]
    fn add_schema_select_qualified_column() {
        // not valid because we haven't specified the schema on the table name
        // `select public.t.c from t` isn't valid sql
        assert!(code_action_not_applicable(
            add_schema,
            "create table t(c text);
        select t$0.c from t;"
        ));
    }

    #[test]
    fn add_schema_with_search_path() {
        assert_snapshot!(
            apply_code_action(
                add_schema,
                "
set search_path to myschema;
create table t$0(a text, b int);"
            ),
            @"
set search_path to myschema;
create table myschema.t(a text, b int);"
        );
    }

    #[test]
    fn add_schema_not_applicable_with_schema() {
        assert!(code_action_not_applicable(
            add_schema,
            "create table myschema.t$0(a text, b int);"
        ));
    }

    #[test]
    fn add_schema_function_call() {
        assert_snapshot!(apply_code_action(
            add_schema,
            "
create function f() returns int8
  as 'select 1'
  language sql;

select f$0();"),
            @"
create function f() returns int8
  as 'select 1'
  language sql;

select public.f();"
        );
    }

    #[test]
    fn add_schema_function_call_not_applicable_with_schema() {
        assert!(code_action_not_applicable(
            add_schema,
            "
create function f() returns int8 as 'select 1' language sql;
select myschema.f$0();"
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

    #[test]
    fn add_explicit_alias_simple_column() {
        assert_snapshot!(apply_code_action(
            add_explicit_alias,
            "select col_na$0me from t;"),
            @"select col_name as col_name from t;"
        );
    }

    #[test]
    fn add_explicit_alias_quoted_identifier() {
        assert_snapshot!(apply_code_action(
            add_explicit_alias,
            r#"select "b"$0 from t;"#),
            @r#"select "b" as b from t;"#
        );
    }

    #[test]
    fn add_explicit_alias_field_expr() {
        assert_snapshot!(apply_code_action(
            add_explicit_alias,
            "select t.col$0umn from t;"),
            @"select t.column as column from t;"
        );
    }

    #[test]
    fn add_explicit_alias_function_call() {
        assert_snapshot!(apply_code_action(
            add_explicit_alias,
            "select cou$0nt(*) from t;"),
            @"select count(*) as count from t;"
        );
    }

    #[test]
    fn add_explicit_alias_cast_to_type() {
        assert_snapshot!(apply_code_action(
            add_explicit_alias,
            "select '1'::bigi$0nt from t;"),
            @"select '1'::bigint as int8 from t;"
        );
    }

    #[test]
    fn add_explicit_alias_cast_column() {
        assert_snapshot!(apply_code_action(
            add_explicit_alias,
            "select col_na$0me::text from t;"),
            @"select col_name::text as col_name from t;"
        );
    }

    #[test]
    fn add_explicit_alias_case_expr() {
        assert_snapshot!(apply_code_action(
            add_explicit_alias,
            "select ca$0se when true then 'a' end from t;"),
            @"select case when true then 'a' end as case from t;"
        );
    }

    #[test]
    fn add_explicit_alias_case_with_else() {
        assert_snapshot!(apply_code_action(
            add_explicit_alias,
            "select ca$0se when true then 'a' else now()::text end from t;"),
            @"select case when true then 'a' else now()::text end as now from t;"
        );
    }

    #[test]
    fn add_explicit_alias_array() {
        assert_snapshot!(apply_code_action(
            add_explicit_alias,
            "select arr$0ay[1, 2, 3] from t;"),
            @"select array[1, 2, 3] as array from t;"
        );
    }

    #[test]
    fn add_explicit_alias_not_applicable_already_has_alias() {
        assert!(code_action_not_applicable(
            add_explicit_alias,
            "select col_name$0 as foo from t;"
        ));
    }

    #[test]
    fn add_explicit_alias_unknown_column() {
        assert_snapshot!(apply_code_action(
            add_explicit_alias,
            "select 1 $0+ 2 from t;"),
            @r#"select 1 + 2 as "?column?" from t;"#
        );
    }

    #[test]
    fn add_explicit_alias_not_applicable_star() {
        assert!(code_action_not_applicable(
            add_explicit_alias,
            "select $0* from t;"
        ));
    }

    #[test]
    fn add_explicit_alias_not_applicable_qualified_star() {
        assert!(code_action_not_applicable(
            add_explicit_alias,
            "with t as (select 1 a) select t.*$0 from t;"
        ));
    }

    #[test]
    fn add_explicit_alias_literal() {
        assert_snapshot!(apply_code_action(
            add_explicit_alias,
            "select 'foo$0' from t;"),
            @r#"select 'foo' as "?column?" from t;"#
        );
    }

    #[test]
    fn remove_redundant_alias_simple() {
        assert_snapshot!(apply_code_action(
            remove_redundant_alias,
            "select col_name as col_na$0me from t;"),
            @"select col_name from t;"
        );
    }

    #[test]
    fn remove_redundant_alias_quoted() {
        assert_snapshot!(apply_code_action(
            remove_redundant_alias,
            r#"select "x"$0 as x from t;"#),
            @r#"select "x" from t;"#
        );
    }

    #[test]
    fn remove_redundant_alias_case_insensitive() {
        assert_snapshot!(apply_code_action(
            remove_redundant_alias,
            "select col_name$0 as COL_NAME from t;"),
            @"select col_name from t;"
        );
    }

    #[test]
    fn remove_redundant_alias_function() {
        assert_snapshot!(apply_code_action(
            remove_redundant_alias,
            "select count(*)$0 as count from t;"),
            @"select count(*) from t;"
        );
    }

    #[test]
    fn remove_redundant_alias_field_expr() {
        assert_snapshot!(apply_code_action(
            remove_redundant_alias,
            "select t.col$0umn as column from t;"),
            @"select t.column from t;"
        );
    }

    #[test]
    fn remove_redundant_alias_not_applicable_different_name() {
        assert!(code_action_not_applicable(
            remove_redundant_alias,
            "select col_name$0 as foo from t;"
        ));
    }

    #[test]
    fn remove_redundant_alias_not_applicable_no_alias() {
        assert!(code_action_not_applicable(
            remove_redundant_alias,
            "select col_name$0 from t;"
        ));
    }

    #[test]
    fn rewrite_cast_to_double_colon_simple() {
        assert_snapshot!(apply_code_action(
            rewrite_cast_to_double_colon,
            "select ca$0st(foo as text) from t;"),
            @"select foo::text from t;"
        );
    }

    #[test]
    fn rewrite_cast_to_double_colon_on_column() {
        assert_snapshot!(apply_code_action(
            rewrite_cast_to_double_colon,
            "select cast(col_na$0me as int) from t;"),
            @"select col_name::int from t;"
        );
    }

    #[test]
    fn rewrite_cast_to_double_colon_on_type() {
        assert_snapshot!(apply_code_action(
            rewrite_cast_to_double_colon,
            "select cast(x as bigi$0nt) from t;"),
            @"select x::bigint from t;"
        );
    }

    #[test]
    fn rewrite_cast_to_double_colon_qualified_type() {
        assert_snapshot!(apply_code_action(
            rewrite_cast_to_double_colon,
            "select cast(x as pg_cata$0log.text) from t;"),
            @"select x::pg_catalog.text from t;"
        );
    }

    #[test]
    fn rewrite_cast_to_double_colon_expression() {
        assert_snapshot!(apply_code_action(
            rewrite_cast_to_double_colon,
            "select ca$0st(1 + 2 as bigint) from t;"),
            @"select 1 + 2::bigint from t;"
        );
    }

    #[test]
    fn rewrite_cast_to_double_colon_type_first_syntax() {
        assert_snapshot!(apply_code_action(
            rewrite_cast_to_double_colon,
            "select in$0t '1';"),
            @"select '1'::int;"
        );
    }

    #[test]
    fn rewrite_cast_to_double_colon_type_first_qualified() {
        assert_snapshot!(apply_code_action(
            rewrite_cast_to_double_colon,
            "select pg_catalog.int$04 '1';"),
            @"select '1'::pg_catalog.int4;"
        );
    }

    #[test]
    fn rewrite_cast_to_double_colon_not_applicable_already_double_colon() {
        assert!(code_action_not_applicable(
            rewrite_cast_to_double_colon,
            "select foo::te$0xt from t;"
        ));
    }

    #[test]
    fn rewrite_cast_to_double_colon_not_applicable_outside_cast() {
        assert!(code_action_not_applicable(
            rewrite_cast_to_double_colon,
            "select fo$0o from t;"
        ));
    }

    #[test]
    fn rewrite_double_colon_to_cast_simple() {
        assert_snapshot!(apply_code_action(
            rewrite_double_colon_to_cast,
            "select foo::te$0xt from t;"),
            @"select cast(foo as text) from t;"
        );
    }

    #[test]
    fn rewrite_double_colon_to_cast_on_column() {
        assert_snapshot!(apply_code_action(
            rewrite_double_colon_to_cast,
            "select col_na$0me::int from t;"),
            @"select cast(col_name as int) from t;"
        );
    }

    #[test]
    fn rewrite_double_colon_to_cast_on_type() {
        assert_snapshot!(apply_code_action(
            rewrite_double_colon_to_cast,
            "select x::bigi$0nt from t;"),
            @"select cast(x as bigint) from t;"
        );
    }

    #[test]
    fn rewrite_double_colon_to_cast_qualified_type() {
        assert_snapshot!(apply_code_action(
            rewrite_double_colon_to_cast,
            "select x::pg_cata$0log.text from t;"),
            @"select cast(x as pg_catalog.text) from t;"
        );
    }

    #[test]
    fn rewrite_double_colon_to_cast_expression() {
        assert_snapshot!(apply_code_action(
            rewrite_double_colon_to_cast,
            "select 1 + 2::bigi$0nt from t;"),
            @"select 1 + cast(2 as bigint) from t;"
        );
    }

    #[test]
    fn rewrite_type_literal_syntax_to_cast() {
        assert_snapshot!(apply_code_action(
            rewrite_double_colon_to_cast,
            "select in$0t '1';"),
            @"select cast('1' as int);"
        );
    }

    #[test]
    fn rewrite_qualified_type_literal_syntax_to_cast() {
        assert_snapshot!(apply_code_action(
            rewrite_double_colon_to_cast,
            "select pg_catalog.int$04 '1';"),
            @"select cast('1' as pg_catalog.int4);"
        );
    }

    #[test]
    fn rewrite_double_colon_to_cast_not_applicable_already_cast() {
        assert!(code_action_not_applicable(
            rewrite_double_colon_to_cast,
            "select ca$0st(foo as text) from t;"
        ));
    }

    #[test]
    fn rewrite_double_colon_to_cast_not_applicable_outside_cast() {
        assert!(code_action_not_applicable(
            rewrite_double_colon_to_cast,
            "select fo$0o from t;"
        ));
    }

    #[test]
    fn rewrite_values_as_select_simple() {
        assert_snapshot!(
            apply_code_action(rewrite_values_as_select, "valu$0es (1, 'one'), (2, 'two');"),
            @r"
        select 1 as column1, 'one' as column2
        union all
        select 2, 'two';
        "
        );
    }

    #[test]
    fn rewrite_values_as_select_single_row() {
        assert_snapshot!(
            apply_code_action(rewrite_values_as_select, "val$0ues (1, 2, 3);"),
            @"select 1 as column1, 2 as column2, 3 as column3;"
        );
    }

    #[test]
    fn rewrite_values_as_select_single_column() {
        assert_snapshot!(
            apply_code_action(rewrite_values_as_select, "values$0 (1);"),
            @"select 1 as column1;"
        );
    }

    #[test]
    fn rewrite_values_as_select_multiple_rows() {
        assert_snapshot!(
            apply_code_action(rewrite_values_as_select, "values (1, 2), (3, 4), (5, 6$0);"),
            @r"
        select 1 as column1, 2 as column2
        union all
        select 3, 4
        union all
        select 5, 6;
        "
        );
    }

    #[test]
    fn rewrite_values_as_select_with_clause() {
        assert_snapshot!(
            apply_code_action(
                rewrite_values_as_select,
                "with cte as (select 1) val$0ues (1, 'one'), (2, 'two');"
            ),
            @r"
        with cte as (select 1) select 1 as column1, 'one' as column2
        union all
        select 2, 'two';
        "
        );
    }

    #[test]
    fn rewrite_values_as_select_complex_expressions() {
        assert_snapshot!(
            apply_code_action(
                rewrite_values_as_select,
                "values (1 + 2, 'test'::text$0, array[1,2]);"
            ),
            @"select 1 + 2 as column1, 'test'::text as column2, array[1,2] as column3;"
        );
    }

    #[test]
    fn rewrite_values_as_select_on_values_keyword() {
        assert_snapshot!(
            apply_code_action(rewrite_values_as_select, "val$0ues (1, 2);"),
            @"select 1 as column1, 2 as column2;"
        );
    }

    #[test]
    fn rewrite_values_as_select_on_row_content() {
        assert_snapshot!(
            apply_code_action(rewrite_values_as_select, "values (1$0, 2), (3, 4);"),
            @r"
        select 1 as column1, 2 as column2
        union all
        select 3, 4;
        "
        );
    }

    #[test]
    fn rewrite_values_as_select_not_applicable_on_select() {
        assert!(code_action_not_applicable(
            rewrite_values_as_select,
            "sel$0ect 1;"
        ));
    }

    #[test]
    fn rewrite_select_as_values_simple() {
        assert_snapshot!(
            apply_code_action(
                rewrite_select_as_values,
                "select 1 as column1, 'one' as column2 union all$0 select 2, 'two';"
            ),
            @"values (1, 'one'), (2, 'two');"
        );
    }

    #[test]
    fn rewrite_select_as_values_multiple_rows() {
        assert_snapshot!(
            apply_code_action(
                rewrite_select_as_values,
                "select 1 as column1, 2 as column2 union$0 all select 3, 4 union all select 5, 6;"
            ),
            @"values (1, 2), (3, 4), (5, 6);"
        );
    }

    #[test]
    fn rewrite_select_as_values_multiple_rows_cursor_on_second_union() {
        assert_snapshot!(
            apply_code_action(
                rewrite_select_as_values,
                "select 1 as column1, 2 as column2 union all select 3, 4 union$0 all select 5, 6;"
            ),
            @"values (1, 2), (3, 4), (5, 6);"
        );
    }

    #[test]
    fn rewrite_select_as_values_single_column() {
        assert_snapshot!(
            apply_code_action(
                rewrite_select_as_values,
                "select 1 as column1$0 union all select 2;"
            ),
            @"values (1), (2);"
        );
    }

    #[test]
    fn rewrite_select_as_values_with_clause() {
        assert_snapshot!(
            apply_code_action(
                rewrite_select_as_values,
                "with cte as (select 1) select 1 as column1, 'one' as column2 uni$0on all select 2, 'two';"
            ),
            @"with cte as (select 1) values (1, 'one'), (2, 'two');"
        );
    }

    #[test]
    fn rewrite_select_as_values_complex_expressions() {
        assert_snapshot!(
            apply_code_action(
                rewrite_select_as_values,
                "select 1 + 2 as column1, 'test'::text as column2$0 union all select 3 * 4, array[1,2]::text;"
            ),
            @"values (1 + 2, 'test'::text), (3 * 4, array[1,2]::text);"
        );
    }

    #[test]
    fn rewrite_select_as_values_single_select() {
        assert_snapshot!(
            apply_code_action(
                rewrite_select_as_values,
                "select 1 as column1, 2 as column2$0;"
            ),
            @"values (1, 2);"
        );
    }

    #[test]
    fn rewrite_select_as_values_single_select_with_clause() {
        assert_snapshot!(
            apply_code_action(
                rewrite_select_as_values,
                "with cte as (select 1) select 1 as column1$0, 'test' as column2;"
            ),
            @"with cte as (select 1) values (1, 'test');"
        );
    }

    #[test]
    fn rewrite_select_as_values_not_applicable_union_without_all() {
        assert!(code_action_not_applicable(
            rewrite_select_as_values,
            "select 1 as column1 union$0 select 2;"
        ));
    }

    #[test]
    fn rewrite_select_as_values_not_applicable_wrong_column_names() {
        assert!(code_action_not_applicable(
            rewrite_select_as_values,
            "select 1 as foo, 2 as bar union all$0 select 3, 4;"
        ));
    }

    #[test]
    fn rewrite_select_as_values_not_applicable_missing_aliases() {
        assert!(code_action_not_applicable(
            rewrite_select_as_values,
            "select 1, 2 union all$0 select 3, 4;"
        ));
    }

    #[test]
    fn rewrite_select_as_values_case_insensitive_column_names() {
        assert_snapshot!(
            apply_code_action(
                rewrite_select_as_values,
                "select 1 as COLUMN1, 2 as CoLuMn2 union all$0 select 3, 4;"
            ),
            @"values (1, 2), (3, 4);"
        );
    }

    #[test]
    fn rewrite_select_as_values_not_applicable_with_values() {
        assert!(code_action_not_applicable(
            rewrite_select_as_values,
            "select 1 as column1, 2 as column2 union all$0 values (3, 4);"
        ));
    }

    #[test]
    fn rewrite_select_as_values_not_applicable_with_table() {
        assert!(code_action_not_applicable(
            rewrite_select_as_values,
            "select 1 as column1, 2 as column2 union all$0 table foo;"
        ));
    }

    #[test]
    fn rewrite_select_as_values_not_applicable_intersect() {
        assert!(code_action_not_applicable(
            rewrite_select_as_values,
            "select 1 as column1, 2 as column2 inter$0sect select 3, 4;"
        ));
    }

    #[test]
    fn rewrite_select_as_values_not_applicable_except() {
        assert!(code_action_not_applicable(
            rewrite_select_as_values,
            "select 1 as column1, 2 as column2 exc$0ept select 3, 4;"
        ));
    }
}
