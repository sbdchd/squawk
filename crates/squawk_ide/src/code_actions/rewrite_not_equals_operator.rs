use rowan::TextSize;
use salsa::Database as Db;
use squawk_syntax::ast::{self, AstNode};

use crate::{db::File, offsets::token_from_offset};

use super::{ActionKind, CodeAction};

pub(super) fn rewrite_not_equals_operator(
    db: &dyn Db,
    file: File,
    actions: &mut Vec<CodeAction>,
    offset: TextSize,
) -> Option<()> {
    let token = token_from_offset(db, file, offset)?;
    let bin_expr = token.parent_ancestors().find_map(ast::BinExpr::cast)?;

    let (op_token, replacement, title) = match bin_expr.op()? {
        ast::BinOp::Neq(token) => (token, "<>", "Rewrite `!=` as `<>`"),
        ast::BinOp::Neqb(token) => (token, "!=", "Rewrite `<>` as `!=`"),
        _ => return None,
    };

    actions.push(CodeAction {
        title: title.to_owned(),
        edits: vec![squawk_linter::Edit::replace(
            op_token.text_range(),
            replacement.to_owned(),
        )],
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::code_actions::test_utils::{apply_code_action, code_action_not_applicable};

    use super::rewrite_not_equals_operator;

    #[test]
    fn rewrite_not_equals_bang_to_angle() {
        assert_snapshot!(
            apply_code_action(rewrite_not_equals_operator, "select 1 !$0= 2;"),
            @"select 1 <> 2;"
        );
    }

    #[test]
    fn rewrite_not_equals_angle_to_bang() {
        assert_snapshot!(
            apply_code_action(rewrite_not_equals_operator, "select 1 <$0> 2;"),
            @"select 1 != 2;"
        );
    }

    #[test]
    fn rewrite_not_equals_cursor_on_operand() {
        assert_snapshot!(
            apply_code_action(rewrite_not_equals_operator, "select a$0 != b from t;"),
            @"select a <> b from t;"
        );
    }

    #[test]
    fn rewrite_not_equals_not_applicable_other_op() {
        assert!(code_action_not_applicable(
            rewrite_not_equals_operator,
            "select 1 =$0 2;"
        ));
    }
}
