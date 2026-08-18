use rowan::TextSize;
use salsa::Database as Db;
use squawk_linter::Edit;
use squawk_syntax::ast::{self, AstNode, BinOp};

use crate::{file::InFile, offsets::token_from_offset};

use super::{ActionKind, CodeAction};

pub(super) fn rewrite_in_as_expression(
    db: &dyn Db,
    position: InFile<TextSize>,
    actions: &mut Vec<CodeAction>,
) -> Option<()> {
    let token = token_from_offset(db, position)?;
    let (in_expr, quantifier, comparison) = token.parent_ancestors().find_map(|node| {
        let expr = ast::BinExpr::cast(node)?;
        match expr.op()? {
            BinOp::In(_) => Some((expr, "ANY", "=")),
            BinOp::NotIn(_) => Some((expr, "ALL", "!=")),
            _ => None,
        }
    })?;

    let lhs = in_expr.lhs()?;
    let ast::Expr::TupleExpr(tuple) = in_expr.rhs()? else {
        return None;
    };
    let items = tuple
        .exprs()
        .map(|expr| expr.syntax().text().to_string())
        .collect::<Vec<_>>()
        .join(", ");
    let replacement = format!(
        "{} {comparison} {quantifier} (ARRAY[{items}])",
        lhs.syntax().text()
    );

    actions.push(CodeAction {
        title: "Rewrite `IN` as an expression".to_owned(),
        edits: vec![Edit::replace(in_expr.syntax().text_range(), replacement)],
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::code_actions::test_utils::{apply_code_action, code_action_not_applicable};

    use super::rewrite_in_as_expression;

    #[test]
    fn rewrites_in_tuple_as_any_array() {
        assert_snapshot!(
            apply_code_action(rewrite_in_as_expression, "select x $0IN (1, y, 'three');"),
            @"select x = ANY (ARRAY[1, y, 'three']);"
        );
    }

    #[test]
    fn rewrites_not_in_tuple_as_all_array() {
        assert_snapshot!(
            apply_code_action(rewrite_in_as_expression, "select x NOT $0IN (1, y, 3);"),
            @"select x != ALL (ARRAY[1, y, 3]);"
        );
    }

    #[test]
    fn rewrites_with_cursor_in_rhs() {
        assert_snapshot!(
            apply_code_action(
                rewrite_in_as_expression,
                "select x IN (-1, 'two'::$0text, (3 + 4));"
            ),
            @"select x = ANY (ARRAY[-1, 'two'::text, (3 + 4)]);"
        );
    }

    #[test]
    fn does_not_rewrite_single_item_in() {
        assert!(code_action_not_applicable(
            rewrite_in_as_expression,
            "select x $0IN (1);"
        ));
    }

    #[test]
    fn does_not_rewrite_in_subquery() {
        assert!(code_action_not_applicable(
            rewrite_in_as_expression,
            "select x $0IN (select y from t);"
        ));
    }
}
