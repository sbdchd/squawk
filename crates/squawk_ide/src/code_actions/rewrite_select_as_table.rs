use rowan::TextSize;
use salsa::Database as Db;
use squawk_syntax::ast::{self, AstNode};

use crate::{db::File, offsets::token_from_offset};

use super::{ActionKind, CodeAction};

pub(super) fn rewrite_select_as_table(
    db: &dyn Db,
    file: File,
    actions: &mut Vec<CodeAction>,
    offset: TextSize,
) -> Option<()> {
    let token = token_from_offset(db, file, offset)?;
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
        edits: vec![squawk_linter::Edit::replace(
            select.syntax().text_range(),
            replacement,
        )],
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

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

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::code_actions::test_utils::{apply_code_action, code_action_not_applicable};

    use super::rewrite_select_as_table;

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
}
