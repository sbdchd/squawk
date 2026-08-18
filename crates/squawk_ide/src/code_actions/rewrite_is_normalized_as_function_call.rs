use rowan::TextSize;
use salsa::Database as Db;
use squawk_linter::Edit;
use squawk_syntax::{
    ast::{self, AstNode, PostfixOp},
    quote::quote_string_literal,
};

use crate::{file::InFile, offsets::token_from_offset};

use super::{ActionKind, CodeAction};

pub(super) fn rewrite_is_normalized_as_function_call(
    db: &dyn Db,
    position: InFile<TextSize>,
    actions: &mut Vec<CodeAction>,
) -> Option<()> {
    let token = token_from_offset(db, position)?;
    let postfix_expr = token.parent_ancestors().find_map(ast::PostfixExpr::cast)?;

    let (normal_form, negated) = match postfix_expr.op()? {
        PostfixOp::IsNormalized(op) => (op.unicode_normal_form(), false),
        PostfixOp::IsNotNormalized(op) => (op.unicode_normal_form(), true),
        _ => return None,
    };

    let expr = postfix_expr.expr()?;
    let expr_text = expr.syntax().text();
    let normal_form = normal_form.map(|form| form.syntax().text().to_string().to_ascii_uppercase());

    let arguments = match normal_form {
        Some(form) => format!("{expr_text}, {}", quote_string_literal(&form)),
        None => expr_text.to_string(),
    };
    let negation = if negated { "not " } else { "" };
    let replacement = format!("{negation}is_normalized({arguments})");

    actions.push(CodeAction {
        title: "Rewrite as `is_normalized` function call".to_owned(),
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

    use super::rewrite_is_normalized_as_function_call;

    #[test]
    fn rewrites_is_normalized_as_function_call() {
        assert_snapshot!(
            apply_code_action(
                rewrite_is_normalized_as_function_call,
                r"select U&'\0061\0301' IS NFC $0NORMALIZED;",
            ),
            @r"select is_normalized(U&'\0061\0301', 'NFC');"
        );
    }

    #[test]
    fn rewrites_default_normal_form() {
        assert_snapshot!(
            apply_code_action(
                rewrite_is_normalized_as_function_call,
                "select ('a' || 'b') $0IS NORMALIZED;",
            ),
            @"select is_normalized(('a' || 'b'));"
        );
    }

    #[test]
    fn rewrites_and_canonicalizes_optional_normal_form() {
        assert_snapshot!(
            apply_code_action(
                rewrite_is_normalized_as_function_call,
                "select value IS nfkd NOR$0MALIZED from t;",
            ),
            @"select is_normalized(value, 'NFKD') from t;"
        );
    }

    #[test]
    fn rewrites_is_not_normalized() {
        assert_snapshot!(
            apply_code_action(
                rewrite_is_normalized_as_function_call,
                "select value IS $0NOT NFD NORMALIZED from t;",
            ),
            @"select not is_normalized(value, 'NFD') from t;"
        );
    }

    #[test]
    fn applies_when_cursor_is_on_the_value() {
        assert_snapshot!(
            apply_code_action(
                rewrite_is_normalized_as_function_call,
                "select val$0ue IS NFC NORMALIZED from t;",
            ),
            @"select is_normalized(value, 'NFC') from t;"
        );
    }

    #[test]
    fn is_not_applicable_outside_is_normalized_expression() {
        assert!(code_action_not_applicable(
            rewrite_is_normalized_as_function_call,
            "select value$0 from t;"
        ));
    }
}
