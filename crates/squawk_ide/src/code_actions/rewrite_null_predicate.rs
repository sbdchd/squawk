use rowan::TextSize;
use salsa::Database as Db;
use squawk_linter::Edit;
use squawk_syntax::ast::{self, AstNode, PostfixOp};

use crate::{file::InFile, offsets::token_from_offset};

use super::{ActionKind, CodeAction};

pub(super) fn rewrite_null_predicate(
    db: &dyn Db,
    position: InFile<TextSize>,
    actions: &mut Vec<CodeAction>,
) -> Option<()> {
    let token = token_from_offset(db, position)?;
    let postfix_expr = token.parent_ancestors().find_map(ast::PostfixExpr::cast)?;

    let (op_token, replacement, title) = match postfix_expr.op()? {
        PostfixOp::IsNull(token) => (token, "is null", "Rewrite as `IS NULL`"),
        PostfixOp::NotNull(token) => (token, "is not null", "Rewrite as `IS NOT NULL`"),
        _ => return None,
    };

    actions.push(CodeAction {
        title: title.to_owned(),
        edits: vec![Edit::replace(op_token.text_range(), replacement.to_owned())],
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::code_actions::test_utils::{apply_code_action, code_action_not_applicable};

    use super::rewrite_null_predicate;

    #[test]
    fn rewrites_isnull() {
        assert_snapshot!(
            apply_code_action(rewrite_null_predicate, "select x is$0null from t;"),
            @"select x is null from t;"
        );
    }

    #[test]
    fn rewrites_notnull() {
        assert_snapshot!(
            apply_code_action(rewrite_null_predicate, "select x not$0null from t;"),
            @"select x is not null from t;"
        );
    }

    #[test]
    fn applies_when_cursor_is_on_the_value() {
        assert_snapshot!(
            apply_code_action(rewrite_null_predicate, "select val$0ue isnull from t;"),
            @"select value is null from t;"
        );
    }

    #[test]
    fn is_not_applicable_outside_null_predicate() {
        assert!(code_action_not_applicable(
            rewrite_null_predicate,
            "select val$0ue from t;"
        ));
    }
}
