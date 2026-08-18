use rowan::TextSize;
use salsa::Database as Db;
use squawk_linter::Edit;
use squawk_syntax::ast::{self, AstNode};

use crate::{file::InFile, offsets::token_from_offset};

use super::{ActionKind, CodeAction};

pub(super) fn rewrite_collation_for_as_function_call(
    db: &dyn Db,
    position: InFile<TextSize>,
    actions: &mut Vec<CodeAction>,
) -> Option<()> {
    let token = token_from_offset(db, position)?;
    let collation_for = token
        .parent_ancestors()
        .find_map(ast::CollationForFn::cast)?;
    let expr = collation_for.expr()?;

    actions.push(CodeAction {
        title: "Rewrite as `pg_collation_for` function call".to_owned(),
        edits: vec![Edit::replace(
            collation_for.syntax().text_range(),
            format!("pg_collation_for({})", expr.syntax().text()),
        )],
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::code_actions::test_utils::{apply_code_action, code_action_not_applicable};

    use super::rewrite_collation_for_as_function_call;

    #[test]
    fn rewrites_collation_for_as_function_call() {
        assert_snapshot!(
            apply_code_action(
                rewrite_collation_for_as_function_call,
                "select COLLATION $0FOR ('x'::text);",
            ),
            @"select pg_collation_for('x'::text);"
        );
    }

    #[test]
    fn rewrites_with_cursor_in_expression() {
        assert_snapshot!(
            apply_code_action(
                rewrite_collation_for_as_function_call,
                "select COLLATION FOR (lower('x'$0));",
            ),
            @"select pg_collation_for(lower('x'));"
        );
    }

    #[test]
    fn rewrites_innermost_collation_for() {
        assert_snapshot!(
            apply_code_action(
                rewrite_collation_for_as_function_call,
                "select COLLATION FOR (COLLATION F$0OR (x));",
            ),
            @"select COLLATION FOR (pg_collation_for(x));"
        );
    }

    #[test]
    fn not_applicable_outside_collation_for() {
        assert!(code_action_not_applicable(
            rewrite_collation_for_as_function_call,
            "select collati$0on from t;"
        ));
    }
}
