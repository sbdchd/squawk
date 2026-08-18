use rowan::TextSize;
use salsa::Database as Db;
use squawk_linter::Edit;
use squawk_syntax::ast::{self, AstNode, OverlayArgs};

use crate::{file::InFile, offsets::token_from_offset};

use super::{ActionKind, CodeAction};

pub(super) fn rewrite_overlay_as_function_call(
    db: &dyn Db,
    position: InFile<TextSize>,
    actions: &mut Vec<CodeAction>,
) -> Option<()> {
    let token = token_from_offset(db, position)?;
    let overlay = token.parent_ancestors().find_map(ast::OverlayFn::cast)?;
    let args = match overlay.overlay_args()? {
        OverlayArgs::OverlayExprs(_) => return None,
        OverlayArgs::OverlayPlacing(args) => args,
    };

    let string = args.string()?;
    let placing = args.placing()?;
    let from = args.from()?;
    let mut arguments = format!(
        "{}, {}, {}",
        string.syntax().text(),
        placing.syntax().text(),
        from.syntax().text()
    );
    if let Some(for_) = args.for_() {
        arguments.push_str(&format!(", {}", for_.syntax().text()));
    }

    actions.push(CodeAction {
        title: "Rewrite as `overlay` function call".to_owned(),
        edits: vec![Edit::replace(
            overlay.syntax().text_range(),
            format!("overlay({arguments})"),
        )],
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::code_actions::test_utils::{apply_code_action, code_action_not_applicable};

    use super::rewrite_overlay_as_function_call;

    #[test]
    fn rewrites_overlay_as_function_call() {
        assert_snapshot!(
            apply_code_action(
                rewrite_overlay_as_function_call,
                "select OVER$0LAY('hello world' PLACING 'Y' FROM 1 FOR 5);",
            ),
            @"select overlay('hello world', 'Y', 1, 5);"
        );
    }

    #[test]
    fn rewrites_without_optional_for_argument() {
        assert_snapshot!(
            apply_code_action(
                rewrite_overlay_as_function_call,
                "select overlay(value PLACING replacement FROM pos$0);",
            ),
            @"select overlay(value, replacement, pos);"
        );
    }

    #[test]
    fn applies_with_cursor_in_source_expression() {
        assert_snapshot!(
            apply_code_action(
                rewrite_overlay_as_function_call,
                "select overlay(lower(val$0ue) placing 'x' from 2 for length(value));",
            ),
            @"select overlay(lower(value), 'x', 2, length(value));"
        );
    }

    #[test]
    fn rewrites_innermost_overlay_and_preserves_aggregate_clause() {
        assert_snapshot!(
            apply_code_action(
                rewrite_overlay_as_function_call,
                "select overlay(overlay(a placing b fr$0om 1) placing c from 2) filter (where ok);",
            ),
            @"select overlay(overlay(a, b, 1) placing c from 2) filter (where ok);"
        );
    }

    #[test]
    fn not_applicable_to_comma_separated_call() {
        assert!(code_action_not_applicable(
            rewrite_overlay_as_function_call,
            "select overlay(a, b, 1$0, 2);"
        ));
    }
}
