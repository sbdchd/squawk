use rowan::{TextRange, TextSize};
use salsa::Database as Db;
use squawk_linter::Edit;
use squawk_syntax::{
    SyntaxKind,
    ast::{self, AstNode},
};

use crate::{file::InFile, offsets::token_from_offset};

use super::{ActionKind, CodeAction};

pub(super) fn remove_routine_param_in(
    db: &dyn Db,
    position: InFile<TextSize>,
    actions: &mut Vec<CodeAction>,
) -> Option<()> {
    let token = token_from_offset(db, position)?;
    let param = token.parent_ancestors().find_map(ast::Param::cast)?;
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

    use super::remove_routine_param_in;

    #[test]
    fn removes_in() {
        assert_snapshot!(
            apply_code_action(
                remove_routine_param_in,
                "create function f(i$0n value int) returns int language sql as $$ select value $$;",
            ),
            @"create function f(value int) returns int language sql as $$ select value $$;"
        );
    }

    #[test]
    fn preserves_comments() {
        assert_snapshot!(
            apply_code_action(
                remove_routine_param_in,
                "create function f(i$0n /* before value */ value int) returns int language sql as $$ select value $$;",
            ),
            @"create function f(/* before value */ value int) returns int language sql as $$ select value $$;"
        );
    }

    #[test]
    fn applies_when_mode_follows_name() {
        assert_snapshot!(
            apply_code_action(
                remove_routine_param_in,
                "create function f(value i$0n int) returns int language sql as $$ select value $$;",
            ),
            @"create function f(value int) returns int language sql as $$ select value $$;"
        );
    }

    #[test]
    fn applies_to_create_aggregate_param() {
        assert_snapshot!(
            apply_code_action(
                remove_routine_param_in,
                "create aggregate a(i$0n value int) (sfunc = f, stype = int);",
            ),
            @"create aggregate a(value int) (sfunc = f, stype = int);"
        );
    }

    #[test]
    fn applies_to_aggregate_signature() {
        assert_snapshot!(
            apply_code_action(remove_routine_param_in, "drop aggregate a(i$0n int);"),
            @"drop aggregate a(int);"
        );
    }

    #[test]
    fn applies_to_function_signature() {
        assert_snapshot!(
            apply_code_action(remove_routine_param_in, "drop function f(i$0n int);"),
            @"drop function f(int);"
        );
    }

    #[test]
    fn applies_to_procedure_signature() {
        assert_snapshot!(
            apply_code_action(remove_routine_param_in, "drop procedure p(i$0n int);"),
            @"drop procedure p(int);"
        );
    }

    #[test]
    fn applies_to_routine_signature() {
        assert_snapshot!(
            apply_code_action(remove_routine_param_in, "drop routine r(i$0n int);"),
            @"drop routine r(int);"
        );
    }

    #[test]
    fn not_applicable_to_out() {
        assert!(code_action_not_applicable(
            remove_routine_param_in,
            "create function f(o$0ut value int) returns int language sql as $$ select value $$;"
        ));
    }

    #[test]
    fn not_applicable_to_inout() {
        assert!(code_action_not_applicable(
            remove_routine_param_in,
            "create function f(ino$0ut value int) returns int language sql as $$ select value $$;"
        ));
    }

    #[test]
    fn applies_to_procedure_param() {
        assert_snapshot!(
            apply_code_action(
                remove_routine_param_in,
                "create procedure p(i$0n value int) language sql as $$ select value $$;",
            ),
            @"create procedure p(value int) language sql as $$ select value $$;"
        );
    }
}
