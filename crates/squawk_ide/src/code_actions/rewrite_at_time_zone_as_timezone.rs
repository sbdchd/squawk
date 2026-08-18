use rowan::TextSize;
use salsa::Database as Db;
use squawk_linter::Edit;
use squawk_syntax::ast::{self, AstNode};

use crate::{file::InFile, offsets::token_from_offset};

use super::{ActionKind, CodeAction};

pub(super) fn rewrite_at_time_zone_as_timezone(
    db: &dyn Db,
    position: InFile<TextSize>,
    actions: &mut Vec<CodeAction>,
) -> Option<()> {
    let token = token_from_offset(db, position)?;
    let bin_expr = token
        .parent_ancestors()
        .filter_map(ast::BinExpr::cast)
        .find(|expr| matches!(expr.op(), Some(ast::BinOp::AtTimeZone(_))))?;

    let value = bin_expr.lhs()?;
    let zone = bin_expr.rhs()?;
    let replacement = format!(
        "timezone({}, {})",
        zone.syntax().text(),
        value.syntax().text()
    );

    actions.push(CodeAction {
        title: "Rewrite as timezone function `timezone()`".to_owned(),
        edits: vec![Edit::replace(bin_expr.syntax().text_range(), replacement)],
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::code_actions::test_utils::{apply_code_action, code_action_not_applicable};

    use super::rewrite_at_time_zone_as_timezone;

    #[test]
    fn rewrites_at_time_zone_as_timezone() {
        assert_snapshot!(
            apply_code_action(
                rewrite_at_time_zone_as_timezone,
                "select TIMESTAMP '2026-08-17 09:30' AT TIME $0ZONE 'America/New_York';",
            ),
            @"select timezone('America/New_York', TIMESTAMP '2026-08-17 09:30');"
        );
    }

    #[test]
    fn applies_from_the_value_and_preserves_surrounding_expression() {
        assert_snapshot!(
            apply_code_action(
                rewrite_at_time_zone_as_timezone,
                "select 1 + created$0_at AT TIME ZONE zone_name;",
            ),
            @"select 1 + timezone(zone_name, created_at);"
        );
    }

    #[test]
    fn rewrites_expression_zone() {
        assert_snapshot!(
            apply_code_action(
                rewrite_at_time_zone_as_timezone,
                "select ts AT TIME ZONE coale$0sce(zone_name, 'UTC');",
            ),
            @"select timezone(coalesce(zone_name, 'UTC'), ts);"
        );
    }

    #[test]
    fn rewrites_nearest_nested_at_time_zone() {
        assert_snapshot!(
            apply_code_action(
                rewrite_at_time_zone_as_timezone,
                "select (ts AT TIME ZONE z$01) AT TIME ZONE z2;",
            ),
            @"select (timezone(z1, ts)) AT TIME ZONE z2;"
        );
    }

    #[test]
    fn not_applicable_outside_at_time_zone() {
        assert!(code_action_not_applicable(
            rewrite_at_time_zone_as_timezone,
            "select created_at +$0 interval '1 hour';"
        ));
    }
}
