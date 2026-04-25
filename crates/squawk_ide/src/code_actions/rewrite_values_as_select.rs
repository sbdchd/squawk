use itertools::Itertools;
use rowan::{TextRange, TextSize};
use salsa::Database as Db;
use squawk_syntax::ast::{self, AstNode};

use crate::{db::File, offsets::token_from_offset};

use super::{ActionKind, CodeAction};

pub(super) fn rewrite_values_as_select(
    db: &dyn Db,
    file: File,
    actions: &mut Vec<CodeAction>,
    offset: TextSize,
) -> Option<()> {
    let token = token_from_offset(db, file, offset)?;
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
        edits: vec![squawk_linter::Edit::replace(values_range, select_stmt)],
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::code_actions::test_utils::{apply_code_action, code_action_not_applicable};

    use super::rewrite_values_as_select;

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
}
