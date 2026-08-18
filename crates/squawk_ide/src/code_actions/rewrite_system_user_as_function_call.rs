use rowan::TextSize;
use salsa::Database as Db;
use squawk_linter::Edit;
use squawk_syntax::{
    SyntaxKind,
    ast::{self, AstNode},
};

use crate::{file::InFile, offsets::token_from_offset};

use super::{ActionKind, CodeAction};

pub(super) fn rewrite_system_user_as_function_call(
    db: &dyn Db,
    position: InFile<TextSize>,
    actions: &mut Vec<CodeAction>,
) -> Option<()> {
    let token = token_from_offset(db, position)?;
    if token.kind() != SyntaxKind::SYSTEM_USER_KW {
        return None;
    }

    let name = token.parent().and_then(ast::NameRef::cast)?;
    let parent = name.syntax().parent()?;

    if ast::CallExpr::can_cast(parent.kind()) || ast::FieldExpr::can_cast(parent.kind()) {
        return None;
    }

    actions.push(CodeAction {
        title: "Rewrite as function call `pg_catalog.system_user()`".to_owned(),
        edits: vec![Edit::replace(
            name.syntax().text_range(),
            "pg_catalog.system_user()",
        )],
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::code_actions::test_utils::{apply_code_action, code_action_not_applicable};

    use super::rewrite_system_user_as_function_call;

    #[test]
    fn rewrites_system_user_as_function_call() {
        assert_snapshot!(
            apply_code_action(
                rewrite_system_user_as_function_call,
                "select SYSTEM_$0USER;",
            ),
            @"select pg_catalog.system_user();"
        );
    }

    #[test]
    fn rewrites_at_expression_boundary() {
        assert_snapshot!(
            apply_code_action(
                rewrite_system_user_as_function_call,
                "select (SYSTEM_USER$0)::text;",
            ),
            @"select (pg_catalog.system_user())::text;"
        );
    }

    #[test]
    fn not_applicable_to_function_call() {
        assert!(code_action_not_applicable(
            rewrite_system_user_as_function_call,
            "select system_$0user();"
        ));
    }

    #[test]
    fn not_applicable_to_qualified_reference() {
        assert!(code_action_not_applicable(
            rewrite_system_user_as_function_call,
            "select account.system_$0user from accounts;"
        ));
    }
}
