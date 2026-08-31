use rowan::{TextRange, TextSize};
use salsa::Database as Db;
use squawk_linter::Edit;
use squawk_syntax::{
    SyntaxKind,
    ast::{self, AstNode},
};

use crate::{file::InFile, offsets::token_from_offset};

use super::{ActionKind, CodeAction};

pub(super) fn remove_function_param_in(
    db: &dyn Db,
    position: InFile<TextSize>,
    actions: &mut Vec<CodeAction>,
) -> Option<()> {
    let token = token_from_offset(db, position)?;
    let param = token.parent_ancestors().find_map(ast::Param::cast)?;
    param
        .syntax()
        .ancestors()
        .find_map(ast::CreateFunction::cast)?;
    let ast::ParamMode::ParamIn(mode) = param.mode()? else {
        return None;
    };
    let in_token = mode.in_token()?;
    let delete_range = in_token
        .next_token()
        .filter(|token| token.kind() == SyntaxKind::WHITESPACE)
        .map(|token| TextRange::new(in_token.text_range().start(), token.text_range().end()))
        .unwrap_or_else(|| in_token.text_range());

    actions.push(CodeAction {
        title: "Remove redundant `IN`".to_owned(),
        edits: vec![Edit::delete(delete_range)],
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::code_actions::test_utils::{apply_code_action, code_action_not_applicable};

    use super::remove_function_param_in;

    #[test]
    fn removes_in() {
        assert_snapshot!(
            apply_code_action(
                remove_function_param_in,
                "create function f(i$0n value int) returns int language sql as $$ select value $$;",
            ),
            @"create function f(value int) returns int language sql as $$ select value $$;"
        );
    }

    #[test]
    fn preserves_comments() {
        assert_snapshot!(
            apply_code_action(
                remove_function_param_in,
                "create function f(i$0n /* before value */ value int) returns int language sql as $$ select value $$;",
            ),
            @"create function f(/* before value */ value int) returns int language sql as $$ select value $$;"
        );
    }

    #[test]
    fn applies_when_mode_follows_name() {
        assert_snapshot!(
            apply_code_action(
                remove_function_param_in,
                "create function f(value i$0n int) returns int language sql as $$ select value $$;",
            ),
            @"create function f(value int) returns int language sql as $$ select value $$;"
        );
    }

    #[test]
    fn not_applicable_to_out() {
        assert!(code_action_not_applicable(
            remove_function_param_in,
            "create function f(o$0ut value int) returns int language sql as $$ select value $$;"
        ));
    }

    #[test]
    fn not_applicable_to_inout() {
        assert!(code_action_not_applicable(
            remove_function_param_in,
            "create function f(ino$0ut value int) returns int language sql as $$ select value $$;"
        ));
    }

    #[test]
    fn not_applicable_to_procedure_param() {
        assert!(code_action_not_applicable(
            remove_function_param_in,
            "create procedure p(i$0n value int) language sql as $$ select value $$;"
        ));
    }
}
