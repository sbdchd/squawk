use rowan::TextSize;
use salsa::Database as Db;
use squawk_linter::Edit;
use squawk_syntax::ast::{self, AstNode};

use crate::{file::InFile, offsets::token_from_offset};

use super::{ActionKind, CodeAction};

pub(super) fn rewrite_position_as_function_call(
    db: &dyn Db,
    position: InFile<TextSize>,
    actions: &mut Vec<CodeAction>,
) -> Option<()> {
    let token = token_from_offset(db, position)?;
    let position_fn = token.parent_ancestors().find_map(ast::PositionFn::cast)?;

    let needle = position_fn.pos()?;
    let haystack = position_fn.string()?;

    actions.push(CodeAction {
        title: "Rewrite as function call `pg_catalog.position()`".to_owned(),
        edits: vec![Edit::replace(
            position_fn.syntax().text_range(),
            format!(
                "pg_catalog.position({}, {})",
                haystack.syntax().text(),
                needle.syntax().text()
            ),
        )],
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::code_actions::test_utils::{apply_code_action, code_action_not_applicable};

    use super::rewrite_position_as_function_call;

    #[test]
    fn rewrites_position_as_function_call() {
        assert_snapshot!(
            apply_code_action(
                rewrite_position_as_function_call,
                "select POSI$0TION('world' IN 'hello world');",
            ),
            @"select pg_catalog.position('hello world', 'world');"
        );
    }

    #[test]
    fn applies_with_cursor_in_an_operand() {
        assert_snapshot!(
            apply_code_action(
                rewrite_position_as_function_call,
                "select position(needle in coalesce(hay$0stack, ''));",
            ),
            @"select pg_catalog.position(coalesce(haystack, ''), needle);"
        );
    }

    #[test]
    fn rewrites_innermost_position_call() {
        assert_snapshot!(
            apply_code_action(
                rewrite_position_as_function_call,
                "select position(position(a i$0n b)::text in c);",
            ),
            @"select position(pg_catalog.position(b, a)::text in c);"
        );
    }

    #[test]
    fn not_applicable_outside_position_call() {
        assert!(code_action_not_applicable(
            rewrite_position_as_function_call,
            "select strpos(haystack, nee$0dle);"
        ));
    }
}
