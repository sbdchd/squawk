use rowan::{TextRange, TextSize};
use salsa::Database as Db;
use squawk_linter::Edit;
use squawk_syntax::ast::{self, AstNode};

use crate::{file::InFile, offsets::token_from_offset};

use super::unnest::{unnest_call, unnest_shadowed};
use super::{ActionKind, CodeAction};

pub(super) fn rewrite_rows_from_as_unnest(
    db: &dyn Db,
    position: InFile<TextSize>,
    actions: &mut Vec<CodeAction>,
) -> Option<()> {
    let token = token_from_offset(db, position)?;
    let rows_from = token.parent_ancestors().find_map(ast::RowsFromItem::cast)?;

    if rows_from
        .alias()
        .and_then(|alias| alias.columns())
        .is_some_and(|columns| matches!(columns, ast::FromAliasColumns::ColumnDefList(_)))
    {
        return None;
    }

    let mut calls = vec![];
    for rows_from_arg in rows_from.rows_from_args() {
        if rows_from_arg.column_def_list().is_some() {
            return None;
        }
        calls.push(unnest_call(&rows_from_arg.call_expr()?)?);
    }
    let (first, rest) = calls.split_first()?;

    let rows_from_range = TextRange::new(
        rows_from.rows_token()?.text_range().start(),
        rows_from.r_paren_token()?.text_range().end(),
    );

    if !rest.is_empty() && unnest_shadowed(db, position.file_id, rows_from_range.start()) {
        return None;
    }

    let args = calls
        .iter()
        .flat_map(|call| &call.args)
        .map(|arg| arg.syntax().text().to_string())
        .collect::<Vec<_>>()
        .join(", ");

    actions.push(CodeAction {
        title: "Rewrite as `unnest`".to_owned(),
        edits: vec![Edit::replace(
            rows_from_range,
            format!("{}({args})", first.name.syntax().text()),
        )],
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::code_actions::test_utils::{apply_code_action, code_action_not_applicable};

    use super::rewrite_rows_from_as_unnest;

    #[test]
    fn rewrite_rows_from_as_unnest_simple() {
        assert_snapshot!(
            apply_code_action(
                rewrite_rows_from_as_unnest,
                "select * from rows f$0rom (unnest(a), unnest(b));"
            ),
            @"select * from unnest(a, b);"
        );
    }

    #[test]
    fn rewrite_rows_from_as_unnest_single_item() {
        assert_snapshot!(
            apply_code_action(
                rewrite_rows_from_as_unnest,
                "select * from rows f$0rom (unnest(a));"
            ),
            @"select * from unnest(a);"
        );
    }

    #[test]
    fn rewrite_rows_from_as_unnest_multi_arg_item() {
        assert_snapshot!(
            apply_code_action(
                rewrite_rows_from_as_unnest,
                "select * from rows f$0rom (unnest(a, b), unnest(c));"
            ),
            @"select * from unnest(a, b, c);"
        );
    }

    #[test]
    fn rewrite_rows_from_as_unnest_keeps_alias() {
        assert_snapshot!(
            apply_code_action(
                rewrite_rows_from_as_unnest,
                "select * from rows f$0rom (unnest(a), unnest(b)) as z(x, y);"
            ),
            @"select * from unnest(a, b) as z(x, y);"
        );
    }

    #[test]
    fn rewrite_rows_from_as_unnest_keeps_with_ordinality() {
        assert_snapshot!(
            apply_code_action(
                rewrite_rows_from_as_unnest,
                "select * from rows f$0rom (unnest(a), unnest(b)) with ordinality as z(x, y, n);"
            ),
            @"select * from unnest(a, b) with ordinality as z(x, y, n);"
        );
    }

    #[test]
    fn rewrite_rows_from_as_unnest_keeps_lateral() {
        assert_snapshot!(
            apply_code_action(
                rewrite_rows_from_as_unnest,
                "select * from t, lateral rows f$0rom (unnest(t.a), unnest(t.b));"
            ),
            @"select * from t, lateral unnest(t.a, t.b);"
        );
    }

    #[test]
    fn rewrite_rows_from_as_unnest_array_literals() {
        assert_snapshot!(
            apply_code_action(
                rewrite_rows_from_as_unnest,
                "select * from rows from (unnest(array[1,2]$0), unnest(array['a','b']));"
            ),
            @"select * from unnest(array[1,2], array['a','b']);"
        );
    }

    #[test]
    fn rewrite_rows_from_as_unnest_preserves_case() {
        assert_snapshot!(
            apply_code_action(
                rewrite_rows_from_as_unnest,
                "select * from ROWS F$0ROM (UNNEST(a), UNNEST(b));"
            ),
            @"select * from UNNEST(a, b);"
        );
    }

    #[test]
    fn rewrite_rows_from_as_unnest_not_applicable_mixed_functions() {
        assert!(code_action_not_applicable(
            rewrite_rows_from_as_unnest,
            "select * from rows f$0rom (unnest(a), generate_series(1, 10));"
        ));
    }

    #[test]
    fn rewrite_rows_from_as_unnest_not_applicable_qualified() {
        assert!(code_action_not_applicable(
            rewrite_rows_from_as_unnest,
            "select * from rows f$0rom (pg_catalog.unnest(a), unnest(b));"
        ));
    }

    #[test]
    fn rewrite_rows_from_as_unnest_not_applicable_variadic() {
        assert!(code_action_not_applicable(
            rewrite_rows_from_as_unnest,
            "select * from rows f$0rom (unnest(variadic a), unnest(b));"
        ));
    }

    #[test]
    fn rewrite_rows_from_as_unnest_not_applicable_column_def_list() {
        assert!(code_action_not_applicable(
            rewrite_rows_from_as_unnest,
            "select * from rows f$0rom (unnest(a), unnest(b)) as z(x int, y text);"
        ));
    }

    #[test]
    fn rewrite_rows_from_as_unnest_not_applicable_per_item_column_def_list() {
        assert!(code_action_not_applicable(
            rewrite_rows_from_as_unnest,
            "select * from rows f$0rom (unnest(a) as (x int), unnest(b) as (y text));"
        ));
    }

    // postgres ignores the `all` and merges the call like any other
    #[test]
    fn rewrite_rows_from_as_unnest_all() {
        assert_snapshot!(
            apply_code_action(
                rewrite_rows_from_as_unnest,
                "select * from rows f$0rom (unnest(all a), unnest(b));"
            ),
            @"select * from unnest(a, b);"
        );
    }

    #[test]
    fn rewrite_rows_from_as_unnest_not_applicable_distinct() {
        assert!(code_action_not_applicable(
            rewrite_rows_from_as_unnest,
            "select * from rows f$0rom (unnest(distinct a), unnest(b));"
        ));
    }

    #[test]
    fn rewrite_rows_from_as_unnest_not_applicable_star() {
        assert!(code_action_not_applicable(
            rewrite_rows_from_as_unnest,
            "select * from rows f$0rom (unnest(*), unnest(b));"
        ));
    }

    #[test]
    fn rewrite_rows_from_as_unnest_not_applicable_named_args() {
        assert!(code_action_not_applicable(
            rewrite_rows_from_as_unnest,
            "select * from rows f$0rom (unnest(x => a), unnest(b));"
        ));
    }

    #[test]
    fn rewrite_rows_from_as_unnest_not_applicable_order_by() {
        assert!(code_action_not_applicable(
            rewrite_rows_from_as_unnest,
            "select * from rows f$0rom (unnest(a, b order by 1), unnest(c));"
        ));
    }

    #[test]
    fn rewrite_rows_from_as_unnest_not_applicable_over_clause() {
        assert!(code_action_not_applicable(
            rewrite_rows_from_as_unnest,
            "select * from rows f$0rom (unnest(a) over (), unnest(b));"
        ));
    }

    #[test]
    fn rewrite_rows_from_as_unnest_not_applicable_filter_clause() {
        assert!(code_action_not_applicable(
            rewrite_rows_from_as_unnest,
            "select * from rows f$0rom (unnest(a) filter (where x), unnest(b));"
        ));
    }

    #[test]
    fn rewrite_rows_from_as_unnest_not_applicable_per_item_column_def_list_with_alias() {
        assert!(code_action_not_applicable(
            rewrite_rows_from_as_unnest,
            "select * from rows f$0rom (unnest(a) as (x int)) as z(q);"
        ));
    }

    #[test]
    fn rewrite_rows_from_as_unnest_not_applicable_shadowed() {
        assert!(code_action_not_applicable(
            rewrite_rows_from_as_unnest,
            "
create function unnest(x text) returns setof text as $$ select 1 $$ language sql;
select * from rows f$0rom (unnest(a), unnest(b));"
        ));
    }

    #[test]
    fn rewrite_rows_from_as_unnest_other_schema_not_shadowed() {
        assert_snapshot!(
            apply_code_action(
                rewrite_rows_from_as_unnest,
                "
create function other.unnest(x text) returns setof text as $$ select 1 $$ language sql;
select * from rows f$0rom (unnest(a), unnest(b));"
            ),
            @"
create function other.unnest(x text) returns setof text as $$ select 1 $$ language sql;
select * from unnest(a, b);"
        );
    }

    #[test]
    fn rewrite_rows_from_as_unnest_shadowed_single_arg() {
        assert_snapshot!(
            apply_code_action(
                rewrite_rows_from_as_unnest,
                "
create function unnest(x text) returns setof text as $$ select 1 $$ language sql;
select * from rows f$0rom (unnest(a));"
            ),
            @"
create function unnest(x text) returns setof text as $$ select 1 $$ language sql;
select * from unnest(a);"
        );
    }

    #[test]
    fn rewrite_rows_from_as_unnest_shadowed_multi_arg_call() {
        assert_snapshot!(
            apply_code_action(
                rewrite_rows_from_as_unnest,
                "
create function unnest(x text) returns setof text as $$ select 1 $$ language sql;
select * from rows f$0rom (unnest(a, b));"
            ),
            @"
create function unnest(x text) returns setof text as $$ select 1 $$ language sql;
select * from unnest(a, b);"
        );
    }

    #[test]
    fn rewrite_rows_from_as_unnest_not_applicable_plain_function_from_item() {
        assert!(code_action_not_applicable(
            rewrite_rows_from_as_unnest,
            "select * from unn$0est(a, b);"
        ));
    }
}
