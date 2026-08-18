use rowan::TextSize;
use salsa::Database as Db;
use squawk_linter::Edit;
use squawk_syntax::ast::{self, AstNode};

use crate::{file::InFile, offsets::token_from_offset};

use super::{ActionKind, CodeAction};

pub(super) fn rewrite_at_local_as_timezone(
    db: &dyn Db,
    position: InFile<TextSize>,
    actions: &mut Vec<CodeAction>,
) -> Option<()> {
    let token = token_from_offset(db, position)?;
    let postfix_expr = token
        .parent_ancestors()
        .filter_map(ast::PostfixExpr::cast)
        .find(|expr| matches!(expr.op(), Some(ast::PostfixOp::AtLocal(_))))?;

    let expr = postfix_expr.expr()?;
    let replacement = format!("timezone({})", expr.syntax().text());

    actions.push(CodeAction {
        title: "Rewrite `AT LOCAL` as `timezone`".to_owned(),
        edits: vec![Edit::replace(
            postfix_expr.syntax().text_range(),
            replacement,
        )],
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::code_actions::test_utils::{apply_code_action, code_action_not_applicable};

    use super::rewrite_at_local_as_timezone;

    #[test]
    fn rewrites_at_local_as_timezone() {
        assert_snapshot!(
            apply_code_action(
                rewrite_at_local_as_timezone,
                "select TIMESTAMPTZ '2026-08-17 09:30+00' AT $0LOCAL;",
            ),
            @"select timezone(TIMESTAMPTZ '2026-08-17 09:30+00');"
        );
    }

    #[test]
    fn rewrites_with_cursor_on_operand() {
        assert_snapshot!(
            apply_code_action(
                rewrite_at_local_as_timezone,
                "select (created_$0at + interval '1 hour') AT LOCAL from events;",
            ),
            @"select timezone((created_at + interval '1 hour')) from events;"
        );
    }

    #[test]
    fn rewrites_with_cursor_at_end_of_expression() {
        assert_snapshot!(
            apply_code_action(
                rewrite_at_local_as_timezone,
                "select created_at AT LOCAL$0;",
            ),
            @"select timezone(created_at);"
        );
    }

    #[test]
    fn not_applicable_to_at_time_zone() {
        assert!(code_action_not_applicable(
            rewrite_at_local_as_timezone,
            "select created_at AT TIME $0ZONE 'UTC';"
        ));
    }
}
