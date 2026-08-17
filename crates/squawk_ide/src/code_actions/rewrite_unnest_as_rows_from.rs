use rowan::TextSize;
use salsa::Database as Db;
use squawk_linter::Edit;
use squawk_syntax::ast::{self, AstNode};

use crate::{file::InFile, offsets::token_from_offset};

use super::unnest::{unnest_call, unnest_shadowed};
use super::{ActionKind, CodeAction};

pub(super) fn rewrite_unnest_as_rows_from(
    db: &dyn Db,
    position: InFile<TextSize>,
    actions: &mut Vec<CodeAction>,
) -> Option<()> {
    let token = token_from_offset(db, position)?;
    let from_item = token
        .parent_ancestors()
        .find_map(ast::FunctionFromItem::cast)?;

    let call_expr = from_item.call_expr()?;
    let call = unnest_call(&call_expr)?;
    if call.args.len() < 2 {
        return None;
    }

    if from_item
        .alias()
        .and_then(|alias| alias.columns())
        .is_some_and(|columns| matches!(columns, ast::FromAliasColumns::ColumnDefList(_)))
    {
        return None;
    }

    let call_range = call_expr.syntax().text_range();
    if unnest_shadowed(db, position.file_id, call_range.start()) {
        return None;
    }

    let name = call.name.syntax().text();
    let calls = call
        .args
        .iter()
        .map(|arg| format!("{name}({})", arg.syntax().text()))
        .collect::<Vec<_>>()
        .join(", ");

    actions.push(CodeAction {
        title: "Rewrite as `rows from`".to_owned(),
        edits: vec![Edit::replace(call_range, format!("rows from ({calls})"))],
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::code_actions::test_utils::{apply_code_action, code_action_not_applicable};

    use super::rewrite_unnest_as_rows_from;

    #[test]
    fn rewrite_unnest_as_rows_from_simple() {
        assert_snapshot!(
            apply_code_action(
                rewrite_unnest_as_rows_from,
                "select * from unn$0est(a, b);"
            ),
            @"select * from rows from (unnest(a), unnest(b));"
        );
    }

    #[test]
    fn rewrite_unnest_as_rows_from_three_args() {
        assert_snapshot!(
            apply_code_action(
                rewrite_unnest_as_rows_from,
                "select * from unnest(array[1,2], array['a','b'], c$0);"
            ),
            @"select * from rows from (unnest(array[1,2]), unnest(array['a','b']), unnest(c));"
        );
    }

    #[test]
    fn rewrite_unnest_as_rows_from_keeps_alias() {
        assert_snapshot!(
            apply_code_action(
                rewrite_unnest_as_rows_from,
                "select * from unn$0est(a, b) as z(x, y);"
            ),
            @"select * from rows from (unnest(a), unnest(b)) as z(x, y);"
        );
    }

    #[test]
    fn rewrite_unnest_as_rows_from_keeps_with_ordinality() {
        assert_snapshot!(
            apply_code_action(
                rewrite_unnest_as_rows_from,
                "select * from unn$0est(a, b) with ordinality as z(x, y, n);"
            ),
            @"select * from rows from (unnest(a), unnest(b)) with ordinality as z(x, y, n);"
        );
    }

    #[test]
    fn rewrite_unnest_as_rows_from_keeps_lateral() {
        assert_snapshot!(
            apply_code_action(
                rewrite_unnest_as_rows_from,
                "select * from t, lateral unn$0est(t.a, t.b);"
            ),
            @"select * from t, lateral rows from (unnest(t.a), unnest(t.b));"
        );
    }

    #[test]
    fn rewrite_unnest_as_rows_from_preserves_case() {
        assert_snapshot!(
            apply_code_action(
                rewrite_unnest_as_rows_from,
                "select * from UNN$0EST(a, b);"
            ),
            @"select * from rows from (UNNEST(a), UNNEST(b));"
        );
    }

    #[test]
    fn rewrite_unnest_as_rows_from_in_join() {
        assert_snapshot!(
            apply_code_action(
                rewrite_unnest_as_rows_from,
                "select * from t join unn$0est(a, b) on true;"
            ),
            @"select * from t join rows from (unnest(a), unnest(b)) on true;"
        );
    }

    #[test]
    fn rewrite_unnest_as_rows_from_not_applicable_single_arg() {
        assert!(code_action_not_applicable(
            rewrite_unnest_as_rows_from,
            "select * from unn$0est(a);"
        ));
    }

    #[test]
    fn rewrite_unnest_as_rows_from_not_applicable_other_function() {
        assert!(code_action_not_applicable(
            rewrite_unnest_as_rows_from,
            "select * from generate_ser$0ies(1, 10);"
        ));
    }

    #[test]
    fn rewrite_unnest_as_rows_from_not_applicable_qualified() {
        assert!(code_action_not_applicable(
            rewrite_unnest_as_rows_from,
            "select * from pg_catalog.unn$0est(a, b);"
        ));
    }

    #[test]
    fn rewrite_unnest_as_rows_from_not_applicable_variadic() {
        assert!(code_action_not_applicable(
            rewrite_unnest_as_rows_from,
            "select * from unn$0est(variadic a, b);"
        ));
    }

    #[test]
    fn rewrite_unnest_as_rows_from_all() {
        assert_snapshot!(
            apply_code_action(
                rewrite_unnest_as_rows_from,
                "select * from unn$0est(all a, b);"
            ),
            @"select * from rows from (unnest(a), unnest(b));"
        );
    }

    #[test]
    fn rewrite_unnest_as_rows_from_not_applicable_distinct() {
        assert!(code_action_not_applicable(
            rewrite_unnest_as_rows_from,
            "select * from unn$0est(distinct a, b);"
        ));
    }

    #[test]
    fn rewrite_unnest_as_rows_from_not_applicable_star() {
        assert!(code_action_not_applicable(
            rewrite_unnest_as_rows_from,
            "select * from unn$0est(*);"
        ));
    }

    #[test]
    fn rewrite_unnest_as_rows_from_not_applicable_named_args() {
        assert!(code_action_not_applicable(
            rewrite_unnest_as_rows_from,
            "select * from unn$0est(x => a, y => b);"
        ));
    }

    #[test]
    fn rewrite_unnest_as_rows_from_not_applicable_order_by() {
        assert!(code_action_not_applicable(
            rewrite_unnest_as_rows_from,
            "select * from unn$0est(a, b order by 1);"
        ));
    }

    // the clause is part of the call expr we replace, so without the check it
    // would be dropped
    #[test]
    fn rewrite_unnest_as_rows_from_not_applicable_over_clause() {
        assert!(code_action_not_applicable(
            rewrite_unnest_as_rows_from,
            "select * from unn$0est(a, b) over ();"
        ));
    }

    #[test]
    fn rewrite_unnest_as_rows_from_not_applicable_filter_clause() {
        assert!(code_action_not_applicable(
            rewrite_unnest_as_rows_from,
            "select * from unn$0est(a, b) filter (where x);"
        ));
    }

    #[test]
    fn rewrite_unnest_as_rows_from_not_applicable_column_def_list() {
        assert!(code_action_not_applicable(
            rewrite_unnest_as_rows_from,
            "select * from unn$0est(a, b) as z(x int, y text);"
        ));
    }

    #[test]
    fn rewrite_unnest_as_rows_from_not_applicable_shadowed() {
        assert!(code_action_not_applicable(
            rewrite_unnest_as_rows_from,
            "
create function unnest(x text) returns setof text as $$ select 1 $$ language sql;
select * from unn$0est(a, b);"
        ));
    }

    #[test]
    fn rewrite_unnest_as_rows_from_other_schema_not_shadowed() {
        assert_snapshot!(
            apply_code_action(
                rewrite_unnest_as_rows_from,
                "
create function other.unnest(x text) returns setof text as $$ select 1 $$ language sql;
select * from unn$0est(a, b);"
            ),
            @"
create function other.unnest(x text) returns setof text as $$ select 1 $$ language sql;
select * from rows from (unnest(a), unnest(b));"
        );
    }

    #[test]
    fn rewrite_unnest_as_rows_from_not_applicable_shadowed_via_search_path() {
        assert!(code_action_not_applicable(
            rewrite_unnest_as_rows_from,
            "
create function other.unnest(x text) returns setof text as $$ select 1 $$ language sql;
set search_path to other, public, pg_catalog;
select * from unn$0est(a, b);"
        ));
    }

    #[test]
    fn rewrite_unnest_as_rows_from_unrelated_function_definition() {
        assert_snapshot!(
            apply_code_action(
                rewrite_unnest_as_rows_from,
                "
create function f(x text) returns setof text as $$ select 1 $$ language sql;
select * from unn$0est(a, b);"
            ),
            @"
create function f(x text) returns setof text as $$ select 1 $$ language sql;
select * from rows from (unnest(a), unnest(b));"
        );
    }

    #[test]
    fn rewrite_unnest_as_rows_from_not_applicable_in_target_list() {
        assert!(code_action_not_applicable(
            rewrite_unnest_as_rows_from,
            "select unn$0est(a, b);"
        ));
    }

    #[test]
    fn rewrite_unnest_as_rows_from_not_applicable_already_rows_from() {
        assert!(code_action_not_applicable(
            rewrite_unnest_as_rows_from,
            "select * from rows from (unn$0est(a), unnest(b));"
        ));
    }
}
