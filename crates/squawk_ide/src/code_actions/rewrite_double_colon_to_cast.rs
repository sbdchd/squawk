use rowan::TextSize;
use salsa::Database as Db;
use squawk_syntax::ast::{self, AstNode};

use crate::{db::File, offsets::token_from_offset};

use super::{ActionKind, CodeAction};

pub(super) fn rewrite_double_colon_to_cast(
    db: &dyn Db,
    file: File,
    actions: &mut Vec<CodeAction>,
    offset: TextSize,
) -> Option<()> {
    let token = token_from_offset(db, file, offset)?;
    let cast_expr = token.parent_ancestors().find_map(ast::CastExpr::cast)?;

    if cast_expr.cast_token().is_some() {
        return None;
    }

    let expr = cast_expr.expr()?;
    let ty = cast_expr.ty()?;

    let expr_text = expr.syntax().text();
    let type_text = ty.syntax().text();

    let replacement = format!("cast({} as {})", expr_text, type_text);

    actions.push(CodeAction {
        title: "Rewrite as cast function `cast()`".to_owned(),
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

    use super::rewrite_double_colon_to_cast;

    #[test]
    fn rewrite_double_colon_to_cast_simple() {
        assert_snapshot!(apply_code_action(
            rewrite_double_colon_to_cast,
            "select foo::te$0xt from t;"),
            @"select cast(foo as text) from t;"
        );
    }

    #[test]
    fn rewrite_double_colon_to_cast_on_column() {
        assert_snapshot!(apply_code_action(
            rewrite_double_colon_to_cast,
            "select col_na$0me::int from t;"),
            @"select cast(col_name as int) from t;"
        );
    }

    #[test]
    fn rewrite_double_colon_to_cast_on_type() {
        assert_snapshot!(apply_code_action(
            rewrite_double_colon_to_cast,
            "select x::bigi$0nt from t;"),
            @"select cast(x as bigint) from t;"
        );
    }

    #[test]
    fn rewrite_double_colon_to_cast_qualified_type() {
        assert_snapshot!(apply_code_action(
            rewrite_double_colon_to_cast,
            "select x::pg_cata$0log.text from t;"),
            @"select cast(x as pg_catalog.text) from t;"
        );
    }

    #[test]
    fn rewrite_double_colon_to_cast_expression() {
        assert_snapshot!(apply_code_action(
            rewrite_double_colon_to_cast,
            "select 1 + 2::bigi$0nt from t;"),
            @"select 1 + cast(2 as bigint) from t;"
        );
    }

    #[test]
    fn rewrite_type_literal_syntax_to_cast() {
        assert_snapshot!(apply_code_action(
            rewrite_double_colon_to_cast,
            "select in$0t '1';"),
            @"select cast('1' as int);"
        );
    }

    #[test]
    fn rewrite_qualified_type_literal_syntax_to_cast() {
        assert_snapshot!(apply_code_action(
            rewrite_double_colon_to_cast,
            "select pg_catalog.int$04 '1';"),
            @"select cast('1' as pg_catalog.int4);"
        );
    }

    #[test]
    fn rewrite_double_colon_to_cast_not_applicable_already_cast() {
        assert!(code_action_not_applicable(
            rewrite_double_colon_to_cast,
            "select ca$0st(foo as text) from t;"
        ));
    }

    #[test]
    fn rewrite_double_colon_to_cast_not_applicable_outside_cast() {
        assert!(code_action_not_applicable(
            rewrite_double_colon_to_cast,
            "select fo$0o from t;"
        ));
    }
}
