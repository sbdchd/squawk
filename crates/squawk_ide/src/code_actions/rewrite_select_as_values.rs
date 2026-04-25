use rowan::{TextRange, TextSize};
use salsa::Database as Db;
use squawk_syntax::{
    SyntaxToken,
    ast::{self, AstNode},
};
use std::iter;

use crate::{db::File, offsets::token_from_offset, symbols::Name};

use super::{ActionKind, CodeAction};

pub(super) fn rewrite_select_as_values(
    db: &dyn Db,
    file: File,
    actions: &mut Vec<CodeAction>,
    offset: TextSize,
) -> Option<()> {
    let token = token_from_offset(db, file, offset)?;

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
        edits: vec![squawk_linter::Edit::replace(select_range, values_stmt)],
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
    use insta::assert_snapshot;

    use crate::code_actions::test_utils::{apply_code_action, code_action_not_applicable};

    use super::rewrite_select_as_values;

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
