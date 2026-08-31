use rowan::TextSize;
use salsa::Database as Db;
use squawk_linter::Edit;
use squawk_syntax::ast::{self, AstNode};

use crate::{file::InFile, offsets::token_from_offset};

use super::{ActionKind, CodeAction};

pub(super) fn rewrite_function_param_in_out_as_inout(
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
    let ast::ParamMode::ParamInOut(mode) = param.mode()? else {
        return None;
    };
    mode.in_token()?;
    mode.out_token()?;

    actions.push(CodeAction {
        title: "Rewrite `IN OUT` as `INOUT`".to_owned(),
        edits: vec![Edit::replace(
            mode.syntax().text_range(),
            "inout".to_owned(),
        )],
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::code_actions::test_utils::{apply_code_action, code_action_not_applicable};

    use super::rewrite_function_param_in_out_as_inout;

    #[test]
    fn rewrites_in_out_as_inout() {
        assert_snapshot!(
            apply_code_action(
                rewrite_function_param_in_out_as_inout,
                "create function f(in $0out value int) returns int language sql as $$ select value $$;",
            ),
            @"create function f(inout value int) returns int language sql as $$ select value $$;"
        );
    }

    #[test]
    fn applies_when_mode_follows_name() {
        assert_snapshot!(
            apply_code_action(
                rewrite_function_param_in_out_as_inout,
                "create function f(value in o$0ut int) returns int language sql as $$ select value $$;",
            ),
            @"create function f(value inout int) returns int language sql as $$ select value $$;"
        );
    }

    #[test]
    fn not_applicable_to_inout() {
        assert!(code_action_not_applicable(
            rewrite_function_param_in_out_as_inout,
            "create function f(ino$0ut value int) returns int language sql as $$ select value $$;"
        ));
    }

    #[test]
    fn not_applicable_to_procedure_param() {
        assert!(code_action_not_applicable(
            rewrite_function_param_in_out_as_inout,
            "create procedure p(in o$0ut value int) language sql as $$ select value $$;"
        ));
    }
}
