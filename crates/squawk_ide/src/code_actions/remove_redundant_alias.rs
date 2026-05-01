use rowan::{TextRange, TextSize};
use salsa::Database as Db;
use squawk_syntax::ast::{self, AstNode};

use crate::{column_name::ColumnName, db::File, offsets::token_from_offset, symbols::Name};

use super::{ActionKind, CodeAction};

pub(super) fn remove_redundant_alias(
    db: &dyn Db,
    file: File,
    actions: &mut Vec<CodeAction>,
    offset: TextSize,
) -> Option<()> {
    let token = token_from_offset(db, file, offset)?;
    let target = token.parent_ancestors().find_map(ast::Target::cast)?;

    let as_name = target.as_name()?;
    let (inferred_column, _) = ColumnName::inferred_from_target(target.clone())?;
    let inferred_column_alias = inferred_column.to_string()?;

    let alias = as_name.name()?;

    if Name::from_node(&alias) != Name::from_string(inferred_column_alias) {
        return None;
    }

    let expr_end = target.expr()?.syntax().text_range().end();
    let alias_end = as_name.syntax().text_range().end();

    actions.push(CodeAction {
        title: "Remove redundant alias".to_owned(),
        edits: vec![squawk_linter::Edit::delete(TextRange::new(
            expr_end, alias_end,
        ))],
        kind: ActionKind::QuickFix,
    });

    Some(())
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::code_actions::test_utils::{apply_code_action, code_action_not_applicable};

    use super::remove_redundant_alias;

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
}
