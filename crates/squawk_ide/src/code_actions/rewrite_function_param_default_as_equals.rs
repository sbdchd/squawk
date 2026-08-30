use rowan::TextSize;
use salsa::Database as Db;
use squawk_linter::Edit;
use squawk_syntax::ast::{self, AstNode};

use crate::{file::InFile, offsets::token_from_offset};

use super::{ActionKind, CodeAction};

pub(super) fn rewrite_function_param_default_as_equals(
    db: &dyn Db,
    position: InFile<TextSize>,
    actions: &mut Vec<CodeAction>,
) -> Option<()> {
    let token = token_from_offset(db, position)?;
    let param_default = token.parent_ancestors().find_map(ast::ParamDefault::cast)?;
    param_default
        .syntax()
        .ancestors()
        .find_map(ast::CreateFunction::cast)?;
    let default_token = param_default.default_token()?;

    actions.push(CodeAction {
        title: "Rewrite `DEFAULT` as `=`".to_owned(),
        edits: vec![Edit::replace(default_token.text_range(), "=".to_owned())],
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::code_actions::test_utils::{apply_code_action, code_action_not_applicable};

    use super::rewrite_function_param_default_as_equals;

    #[test]
    fn rewrites_default_as_equals() {
        assert_snapshot!(
            apply_code_action(
                rewrite_function_param_default_as_equals,
                "create function f(a int def$0ault 1) returns int language sql as $$ select a $$;",
            ),
            @"create function f(a int = 1) returns int language sql as $$ select a $$;"
        );
    }

    #[test]
    fn applies_when_cursor_is_on_default_expression() {
        assert_snapshot!(
            apply_code_action(
                rewrite_function_param_default_as_equals,
                "create function f(a text DEFAULT lower($0'x')) returns text language sql as $$ select a $$;",
            ),
            @"create function f(a text = lower('x')) returns text language sql as $$ select a $$;"
        );
    }

    #[test]
    fn preserves_comments() {
        assert_snapshot!(
            apply_code_action(
                rewrite_function_param_default_as_equals,
                "create function f(a int DEFAULT$0 /* value */ 1) returns int language sql as $$ select a $$;",
            ),
            @"create function f(a int = /* value */ 1) returns int language sql as $$ select a $$;"
        );
    }

    #[test]
    fn not_applicable_to_equals() {
        assert!(code_action_not_applicable(
            rewrite_function_param_default_as_equals,
            "create function f(a int =$0 1) returns int language sql as $$ select a $$;"
        ));
    }

    #[test]
    fn not_applicable_to_column_default() {
        assert!(code_action_not_applicable(
            rewrite_function_param_default_as_equals,
            "create table t(a int DEF$0AULT 1);"
        ));
    }
}
