use rowan::{TextRange, TextSize};
use salsa::Database as Db;
use squawk_linter::Edit;
use squawk_syntax::{
    SyntaxKind, SyntaxNode,
    ast::{self, AstNode},
};

use crate::{file::InFile, offsets::token_from_offset};

use super::{ActionKind, CodeAction};

pub(super) fn rewrite_overlaps_as_function_call(
    db: &dyn Db,
    position: InFile<TextSize>,
    actions: &mut Vec<CodeAction>,
) -> Option<()> {
    let token = token_from_offset(db, position)?;
    let bin_expr = token
        .parent_ancestors()
        .filter_map(ast::BinExpr::cast)
        .find(|expr| matches!(expr.op(), Some(ast::BinOp::Overlaps(_))))?;

    let lhs = pair_tuple_ranges(bin_expr.lhs()?)?;
    let rhs = pair_tuple_ranges(bin_expr.rhs()?)?;

    actions.push(CodeAction {
        title: "Rewrite as `overlaps` function call".to_owned(),
        edits: vec![
            Edit::replace(
                TextRange::new(lhs.outer.start(), lhs.l_paren.start()),
                "overlaps",
            ),
            Edit::replace(TextRange::new(lhs.r_paren.start(), rhs.l_paren.end()), ", "),
        ],
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

struct TupleRanges {
    outer: TextRange,
    l_paren: TextRange,
    r_paren: TextRange,
}

fn pair_tuple_ranges(expr: ast::Expr) -> Option<TupleRanges> {
    let ast::Expr::TupleExpr(tuple) = expr else {
        return None;
    };

    if tuple.exprs().count() != 2 {
        return None;
    }

    tuple_ranges(tuple.syntax())
}

fn tuple_ranges(tuple: &SyntaxNode) -> Option<TupleRanges> {
    let l_paren = tuple
        .children_with_tokens()
        .find(|element| element.kind() == SyntaxKind::L_PAREN)?;
    let r_paren = tuple
        .children_with_tokens()
        .find(|element| element.kind() == SyntaxKind::R_PAREN)?;

    Some(TupleRanges {
        outer: tuple.text_range(),
        l_paren: l_paren.text_range(),
        r_paren: r_paren.text_range(),
    })
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::code_actions::test_utils::{
        apply_code_action, code_action_not_applicable, code_action_not_applicable_with_errors,
    };

    use super::rewrite_overlaps_as_function_call;

    #[test]
    fn rewrites_overlaps_as_function_call() {
        assert_snapshot!(
            apply_code_action(
                rewrite_overlaps_as_function_call,
                "select (TIMESTAMP '2026-01-01', TIMESTAMP '2026-06-01') OVER$0LAPS (TIMESTAMP '2026-03-01', TIMESTAMP '2026-09-01');",
            ),
            @"select overlaps(TIMESTAMP '2026-01-01', TIMESTAMP '2026-06-01', TIMESTAMP '2026-03-01', TIMESTAMP '2026-09-01');"
        );
    }

    #[test]
    fn rewrites_with_cursor_in_an_operand() {
        assert_snapshot!(
            apply_code_action(
                rewrite_overlaps_as_function_call,
                "select (started_$0at, ended_at) overlaps (other_start, other_end);",
            ),
            @"select overlaps(started_at, ended_at, other_start, other_end);"
        );
    }

    #[test]
    fn rewrites_explicit_row_syntax() {
        assert_snapshot!(
            apply_code_action(
                rewrite_overlaps_as_function_call,
                "select ROW(started_at, duration) overlaps R$0OW(other_start, other_duration);",
            ),
            @"select overlaps(started_at, duration, other_start, other_duration);"
        );
    }

    #[test]
    fn rewrites_innermost_overlaps_expression() {
        assert_snapshot!(
            apply_code_action(
                rewrite_overlaps_as_function_call,
                "select ((a, b) over$0laps (c, d), e) overlaps (f, g);",
            ),
            @"select (overlaps(a, b, c, d), e) overlaps (f, g);"
        );
    }

    #[test]
    fn not_applicable_to_non_pair_tuples() {
        assert!(code_action_not_applicable_with_errors(
            rewrite_overlaps_as_function_call,
            "select (a, b, c) over$0laps (d, e);"
        ));
    }

    #[test]
    fn not_applicable_outside_overlaps() {
        assert!(code_action_not_applicable(
            rewrite_overlaps_as_function_call,
            "select overlaps(a, b, c,$0 d);"
        ));
    }
}
