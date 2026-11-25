use squawk_diagnostics::Edit;
use squawk_rowan::TextSize;
use squawk_sema::file::InFile;
use squawk_syntax::ast::{self, AstNode};

use squawk_ide_db::Db;
use squawk_ide_db::offsets::token_from_offset;

use super::{ActionKind, CodeAction};

pub(super) fn rewrite_between_as_binary_expression(
    db: &dyn Db,
    position: InFile<TextSize>,
    actions: &mut Vec<CodeAction>,
) -> Option<()> {
    let token = token_from_offset(db, position)?;
    let between_expr = token.parent_ancestors().find_map(ast::BetweenExpr::cast)?;

    let target = between_expr.target()?;
    let start = between_expr.start()?;
    let end = between_expr.end()?;

    let is_not = between_expr.not_token().is_some();
    let is_symmetric = matches!(
        between_expr.between_symmetry(),
        Some(ast::BetweenSymmetry::Symmetric(_))
    );

    let target_text = operand_text(&target);

    let replacement = if is_symmetric {
        let start_text = start.syntax().text();
        let end_text = end.syntax().text();
        if is_not {
            format!(
                "({target_text} < least({start_text}, {end_text}) or {target_text} > greatest({start_text}, {end_text}))"
            )
        } else {
            format!(
                "({target_text} >= least({start_text}, {end_text}) and {target_text} <= greatest({start_text}, {end_text}))"
            )
        }
    } else {
        let start_text = operand_text(&start);
        let end_text = operand_text(&end);
        if is_not {
            format!("({target_text} < {start_text} or {target_text} > {end_text})")
        } else {
            format!("({target_text} >= {start_text} and {target_text} <= {end_text})")
        }
    };
    actions.push(CodeAction {
        title: "Rewrite as binary expression".to_owned(),
        edits: vec![Edit::replace(
            between_expr.syntax().text_range(),
            replacement,
        )],
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

fn operand_text(expr: &ast::Expr) -> String {
    let needs_parens = match expr {
        ast::Expr::BetweenExpr(_)
        | ast::Expr::BinExpr(_)
        | ast::Expr::PostfixExpr(_)
        | ast::Expr::PrefixExpr(_) => true,
        ast::Expr::ArrayExpr(_)
        | ast::Expr::CallExpr(_)
        | ast::Expr::CaseExpr(_)
        | ast::Expr::CastExpr(_)
        | ast::Expr::Collate(_)
        | ast::Expr::FieldExpr(_)
        | ast::Expr::IndexExpr(_)
        | ast::Expr::Literal(_)
        | ast::Expr::NameRef(_)
        | ast::Expr::ParenExpr(_)
        | ast::Expr::SliceExpr(_)
        | ast::Expr::TupleExpr(_) => false,
    };
    if needs_parens {
        format!("({})", expr.syntax().text())
    } else {
        expr.syntax().to_string()
    }
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::test_utils::{apply_code_action, code_action_not_applicable};

    use super::rewrite_between_as_binary_expression;

    #[test]
    fn rewrite_between_as_binary_expression_simple() {
        assert_snapshot!(apply_code_action(
            rewrite_between_as_binary_expression,
            "select 2 betw$0een 1 and 3;"
        ),
        @"select (2 >= 1 and 2 <= 3);"
        );
    }

    #[test]
    fn rewrite_not_between_as_binary_expression() {
        assert_snapshot!(apply_code_action(
            rewrite_between_as_binary_expression,
            "select 2 no$0t between 1 and 3;"
        ),
        @"select (2 < 1 or 2 > 3);"
        );
    }

    #[test]
    fn rewrite_between_symmetric_as_binary_expression() {
        assert_snapshot!(apply_code_action(
            rewrite_between_as_binary_expression,
            "select 2 between symme$0tric 3 and 1;"
        ),
        @"select (2 >= least(3, 1) and 2 <= greatest(3, 1));"
        );
    }

    #[test]
    fn rewrite_not_between_symmetric_as_binary_expression() {
        assert_snapshot!(apply_code_action(
            rewrite_between_as_binary_expression,
            "select 2 not between symme$0tric 3 and 1;"
        ),
        @"select (2 < least(3, 1) or 2 > greatest(3, 1));"
        );
    }

    #[test]
    fn rewrite_between_as_binary_expression_not_applicable() {
        assert!(code_action_not_applicable(
            rewrite_between_as_binary_expression,
            "select 1 +$0 2;"
        ));
    }

    #[test]
    fn rewrite_between_parenthesizes() {
        assert_snapshot!(apply_code_action(
            rewrite_between_as_binary_expression,
            r#"select t > 1 betw$0een 1 and 1;"#),
            @r#"select t > (1 >= 1 and 1 <= 1);"#
        );
        assert_snapshot!(apply_code_action(
            rewrite_between_as_binary_expression,
            r#"select not a betw$0een 1 and 2;"#),
            @r#"select not (a >= 1 and a <= 2);"#
        );
        assert_snapshot!(apply_code_action(
            rewrite_between_as_binary_expression,
            r#"select a betw$0een 1 and 2 is true;"#),
            @r#"select (a >= 1 and a <= 2) is true;"#
        );
        assert_snapshot!(apply_code_action(
            rewrite_between_as_binary_expression,
            r#"select a and b betw$0een 1 and 2 or c;"#),
            @r#"select a and (b >= 1 and b <= 2) or c;"#
        );
        assert_snapshot!(apply_code_action(
            rewrite_between_as_binary_expression,
            r#"select a betw$0een b = c and 2;"#),
            @r#"select (a >= (b = c) and a <= 2);"#
        );
        assert_snapshot!(apply_code_action(
            rewrite_between_as_binary_expression,
            r#"select (a betw$0een 1 and 2);"#),
            @r#"select ((a >= 1 and a <= 2));"#
        );
        assert_snapshot!(apply_code_action(
            rewrite_between_as_binary_expression,
            r#"select t > 1 not betw$0een 1 and 1;"#),
            @r#"select t > (1 < 1 or 1 > 1);"#
        );
    }
}
