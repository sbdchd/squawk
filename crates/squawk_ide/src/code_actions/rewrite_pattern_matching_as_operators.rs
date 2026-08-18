use rowan::TextSize;
use salsa::Database as Db;
use squawk_linter::Edit;
use squawk_syntax::ast::{self, AstNode, BinOp};

use crate::{file::InFile, offsets::token_from_offset};

use super::{ActionKind, CodeAction};

pub(super) fn rewrite_pattern_matching_as_operators(
    db: &dyn Db,
    position: InFile<TextSize>,
    actions: &mut Vec<CodeAction>,
) -> Option<()> {
    let token = token_from_offset(db, position)?;

    let (expr, operator, operator_range, title, needs_similar_to_escape) = token
        .parent_ancestors()
        .filter_map(ast::BinExpr::cast)
        .find_map(|expr| {
            let (operator, operator_range, title, needs_similar_to_escape) = match expr.op()? {
                BinOp::Like(token) => ("~~", token.text_range(), "Rewrite `LIKE` as `~~`", false),
                BinOp::NotLike(op) => (
                    "!~~",
                    op.syntax().text_range(),
                    "Rewrite `NOT LIKE` as `!~~`",
                    false,
                ),
                BinOp::Ilike(token) => {
                    ("~~*", token.text_range(), "Rewrite `ILIKE` as `~~*`", false)
                }
                BinOp::NotIlike(op) => (
                    "!~~*",
                    op.syntax().text_range(),
                    "Rewrite `NOT ILIKE` as `!~~*`",
                    false,
                ),
                BinOp::SimilarTo(op) => (
                    "~",
                    op.syntax().text_range(),
                    "Rewrite `SIMILAR TO` as `~`",
                    true,
                ),
                BinOp::NotSimilarTo(op) => (
                    "!~",
                    op.syntax().text_range(),
                    "Rewrite `NOT SIMILAR TO` as `!~`",
                    true,
                ),
                _ => return None,
            };
            Some((
                expr,
                operator,
                operator_range,
                title,
                needs_similar_to_escape,
            ))
        })?;

    let rhs = expr.rhs()?;
    let explicit_escape = ast::BinExpr::cast(rhs.syntax().clone()).and_then(|escape_expr| {
        if !matches!(escape_expr.op(), Some(BinOp::Escape(_))) {
            return None;
        }
        Some((escape_expr.lhs()?, escape_expr.rhs()?))
    });

    let mut edits = vec![Edit::replace(operator_range, operator.to_owned())];
    if needs_similar_to_escape || explicit_escape.is_some() {
        let function = if needs_similar_to_escape {
            "similar_to_escape"
        } else {
            "like_escape"
        };
        let arguments = match explicit_escape {
            Some((pattern, escape)) => {
                format!("{}, {}", pattern.syntax().text(), escape.syntax().text())
            }
            None => rhs.syntax().text().to_string(),
        };
        edits.push(Edit::replace(
            rhs.syntax().text_range(),
            format!("pg_catalog.{function}({arguments})"),
        ));
    }

    actions.push(CodeAction {
        title: title.to_owned(),
        edits,
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::code_actions::test_utils::{apply_code_action, code_action_not_applicable};

    use super::rewrite_pattern_matching_as_operators;

    #[test]
    fn rewrites_like_as_operator() {
        assert_snapshot!(
            apply_code_action(rewrite_pattern_matching_as_operators, "select s $0LIKE p;"),
            @"select s ~~ p;"
        );
    }

    #[test]
    fn rewrites_not_like_as_operator() {
        assert_snapshot!(
            apply_code_action(rewrite_pattern_matching_as_operators, "select s NOT $0LIKE p;"),
            @"select s !~~ p;"
        );
    }

    #[test]
    fn rewrites_ilike_as_operator() {
        assert_snapshot!(
            apply_code_action(rewrite_pattern_matching_as_operators, "select s $0ILIKE p;"),
            @"select s ~~* p;"
        );
    }

    #[test]
    fn rewrites_not_ilike_as_operator() {
        assert_snapshot!(
            apply_code_action(rewrite_pattern_matching_as_operators, "select s NOT $0ILIKE p;"),
            @"select s !~~* p;"
        );
    }

    #[test]
    fn rewrites_similar_to_as_operator() {
        assert_snapshot!(
            apply_code_action(
                rewrite_pattern_matching_as_operators,
                "select s $0SIMILAR TO p;",
            ),
            @"select s ~ pg_catalog.similar_to_escape(p);"
        );
    }

    #[test]
    fn rewrites_not_similar_to_as_operator() {
        assert_snapshot!(
            apply_code_action(
                rewrite_pattern_matching_as_operators,
                "select s NOT $0SIMILAR TO p;",
            ),
            @"select s !~ pg_catalog.similar_to_escape(p);"
        );
    }

    #[test]
    fn rewrites_similar_to_with_escape() {
        assert_snapshot!(
            apply_code_action(
                rewrite_pattern_matching_as_operators,
                "select s SIMILAR TO p ESC$0APE e;",
            ),
            @"select s ~ pg_catalog.similar_to_escape(p, e);"
        );
    }

    #[test]
    fn rewrites_like_with_escape() {
        assert_snapshot!(
            apply_code_action(
                rewrite_pattern_matching_as_operators,
                "select s LIK$0E p ESCAPE e;",
            ),
            @"select s ~~ pg_catalog.like_escape(p, e);"
        );
    }

    #[test]
    fn applies_when_cursor_is_on_pattern() {
        assert_snapshot!(
            apply_code_action(
                rewrite_pattern_matching_as_operators,
                "select s NOT ILIKE lower($0pattern);",
            ),
            @"select s !~~* lower(pattern);"
        );
    }

    #[test]
    fn not_applicable_to_other_binary_expressions() {
        assert!(code_action_not_applicable(
            rewrite_pattern_matching_as_operators,
            "select left +$0 right;"
        ));
    }
}
