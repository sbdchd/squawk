use rowan::TextSize;
use salsa::Database as Db;
use squawk_linter::Edit;
use squawk_syntax::ast::{self, AstNode};

use crate::{file::InFile, offsets::token_from_offset};

use super::{ActionKind, CodeAction};

pub(super) fn rewrite_colon_eq_as_fat_arrow(
    db: &dyn Db,
    position: InFile<TextSize>,
    actions: &mut Vec<CodeAction>,
) -> Option<()> {
    let token = token_from_offset(db, position)?;
    let named_arg = token.parent_ancestors().find_map(ast::NamedArg::cast)?;
    let colon_eq = named_arg.colon_eq_token()?;

    actions.push(CodeAction {
        title: "Rewrite `:=` as `=>`".to_owned(),
        edits: vec![Edit::replace(colon_eq.text_range(), "=>".to_owned())],
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::code_actions::test_utils::{apply_code_action, code_action_not_applicable};

    use super::rewrite_colon_eq_as_fat_arrow;

    #[test]
    fn rewrites_colon_equals_as_fat_arrow() {
        assert_snapshot!(
            apply_code_action(rewrite_colon_eq_as_fat_arrow, "select f(name :$0= 1);"),
            @"select f(name => 1);"
        );
    }

    #[test]
    fn applies_when_cursor_is_on_argument_value() {
        assert_snapshot!(
            apply_code_action(
                rewrite_colon_eq_as_fat_arrow,
                "select f(name := lower($0value));",
            ),
            @"select f(name => lower(value));"
        );
    }

    #[test]
    fn not_applicable_to_fat_arrow() {
        assert!(code_action_not_applicable(
            rewrite_colon_eq_as_fat_arrow,
            "select f(name =$0> 1);"
        ));
    }
}
