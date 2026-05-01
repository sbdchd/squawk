use rowan::TextSize;
use salsa::Database as Db;
use squawk_syntax::ast::AstNode;

use crate::{db::File, offsets::token_from_offset};

use super::{ActionKind, CodeAction};

pub(super) fn rewrite_between_as_binary_expression(
    db: &dyn Db,
    file: File,
    actions: &mut Vec<CodeAction>,
    offset: TextSize,
) -> Option<()> {
    use squawk_syntax::ast;

    let token = token_from_offset(db, file, offset)?;
    let between_expr = token.parent_ancestors().find_map(ast::BetweenExpr::cast)?;

    let target = between_expr.target()?;
    let start = between_expr.start()?;
    let end = between_expr.end()?;

    let is_not = between_expr.not_token().is_some();
    let is_symmetric = between_expr.symmetric_token().is_some();

    let target_text = target.syntax().text();
    let start_text = start.syntax().text();
    let end_text = end.syntax().text();

    let replacement = match (is_not, is_symmetric) {
        (false, false) => {
            format!("{target_text} >= {start_text} and {target_text} <= {end_text}")
        }
        (true, false) => {
            format!("({target_text} < {start_text} or {target_text} > {end_text})")
        }
        (false, true) => format!(
            "{target_text} >= least({start_text}, {end_text}) and {target_text} <= greatest({start_text}, {end_text})"
        ),
        (true, true) => format!(
            "({target_text} < least({start_text}, {end_text}) or {target_text} > greatest({start_text}, {end_text}))"
        ),
    };

    actions.push(CodeAction {
        title: "Rewrite as binary expression".to_owned(),
        edits: vec![squawk_linter::Edit::replace(
            between_expr.syntax().text_range(),
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

    use super::rewrite_between_as_binary_expression;

    #[test]
    fn rewrite_between_as_binary_expression_simple() {
        assert_snapshot!(apply_code_action(
            rewrite_between_as_binary_expression,
            "select 2 betw$0een 1 and 3;"
        ),
        @"select 2 >= 1 and 2 <= 3;"
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
        @"select 2 >= least(3, 1) and 2 <= greatest(3, 1);"
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
}
