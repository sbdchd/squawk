use rowan::TextSize;
use salsa::Database as Db;
use squawk_syntax::ast::{self, AstNode};

use crate::{db::File, offsets::token_from_offset};

use super::{ActionKind, CodeAction};

pub(super) fn rewrite_cast_to_double_colon(
    db: &dyn Db,
    file: File,
    actions: &mut Vec<CodeAction>,
    offset: TextSize,
) -> Option<()> {
    let token = token_from_offset(db, file, offset)?;
    let cast_expr = token.parent_ancestors().find_map(ast::CastExpr::cast)?;

    if cast_expr.colon_colon().is_some() {
        return None;
    }

    let expr = cast_expr.expr()?;
    let ty = cast_expr.ty()?;

    let expr_text = expr.syntax().text();
    let type_text = ty.syntax().text();

    let replacement = format!("{}::{}", expr_text, type_text);

    actions.push(CodeAction {
        title: "Rewrite as cast operator `::`".to_owned(),
        edits: vec![squawk_linter::Edit::replace(
            cast_expr.syntax().text_range(),
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

    use super::rewrite_cast_to_double_colon;

    #[test]
    fn rewrite_cast_to_double_colon_simple() {
        assert_snapshot!(apply_code_action(
            rewrite_cast_to_double_colon,
            "select ca$0st(foo as text) from t;"),
            @"select foo::text from t;"
        );
    }

    #[test]
    fn rewrite_cast_to_double_colon_on_column() {
        assert_snapshot!(apply_code_action(
            rewrite_cast_to_double_colon,
            "select cast(col_na$0me as int) from t;"),
            @"select col_name::int from t;"
        );
    }

    #[test]
    fn rewrite_cast_to_double_colon_on_type() {
        assert_snapshot!(apply_code_action(
            rewrite_cast_to_double_colon,
            "select cast(x as bigi$0nt) from t;"),
            @"select x::bigint from t;"
        );
    }

    #[test]
    fn rewrite_cast_to_double_colon_qualified_type() {
        assert_snapshot!(apply_code_action(
            rewrite_cast_to_double_colon,
            "select cast(x as pg_cata$0log.text) from t;"),
            @"select x::pg_catalog.text from t;"
        );
    }

    #[test]
    fn rewrite_cast_to_double_colon_expression() {
        assert_snapshot!(apply_code_action(
            rewrite_cast_to_double_colon,
            "select ca$0st(1 + 2 as bigint) from t;"),
            @"select 1 + 2::bigint from t;"
        );
    }

    #[test]
    fn rewrite_cast_to_double_colon_type_first_syntax() {
        assert_snapshot!(apply_code_action(
            rewrite_cast_to_double_colon,
            "select in$0t '1';"),
            @"select '1'::int;"
        );
    }

    #[test]
    fn rewrite_cast_to_double_colon_type_first_qualified() {
        assert_snapshot!(apply_code_action(
            rewrite_cast_to_double_colon,
            "select pg_catalog.int$04 '1';"),
            @"select '1'::pg_catalog.int4;"
        );
    }

    #[test]
    fn rewrite_cast_to_double_colon_not_applicable_already_double_colon() {
        assert!(code_action_not_applicable(
            rewrite_cast_to_double_colon,
            "select foo::te$0xt from t;"
        ));
    }

    #[test]
    fn rewrite_cast_to_double_colon_not_applicable_outside_cast() {
        assert!(code_action_not_applicable(
            rewrite_cast_to_double_colon,
            "select fo$0o from t;"
        ));
    }
}
