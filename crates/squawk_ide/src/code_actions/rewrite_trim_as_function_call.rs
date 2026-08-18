use rowan::TextSize;
use salsa::Database as Db;
use squawk_linter::Edit;
use squawk_syntax::ast::{self, AstNode, TrimArgs, TrimSide};

use crate::{file::InFile, offsets::token_from_offset};

use super::{ActionKind, CodeAction};

pub(super) fn rewrite_trim_as_function_call(
    db: &dyn Db,
    position: InFile<TextSize>,
    actions: &mut Vec<CodeAction>,
) -> Option<()> {
    let token = token_from_offset(db, position)?;
    let trim = token.parent_ancestors().find_map(ast::TrimFn::cast)?;

    let (function, is_trailing) = match trim.trim_side()? {
        TrimSide::TrimBoth(_) => ("btrim", false),
        TrimSide::TrimLeading(_) => ("ltrim", false),
        TrimSide::TrimTrailing(_) => ("rtrim", true),
    };

    let expressions = match trim.trim_args()? {
        TrimArgs::TrimExprFrom(args) => {
            let mut expressions = args.exprs().collect::<Vec<_>>();
            if expressions.len() < 2 || (!is_trailing && expressions.len() != 2) {
                return None;
            }
            expressions.rotate_left(1);
            expressions
        }
        TrimArgs::TrimFrom(args) => args.exprs().collect(),
        TrimArgs::TrimExprs(args) if is_trailing => args.exprs().collect(),
        TrimArgs::TrimExprs(_) => return None,
    };
    if expressions.is_empty() {
        return None;
    }

    let arguments = expressions
        .iter()
        .map(|expr| expr.syntax().text().to_string())
        .collect::<Vec<_>>()
        .join(", ");

    actions.push(CodeAction {
        title: format!("Rewrite as `{function}` function call"),
        edits: vec![Edit::replace(
            trim.syntax().text_range(),
            format!("{function}({arguments})"),
        )],
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::code_actions::test_utils::{apply_code_action, code_action_not_applicable};

    use super::rewrite_trim_as_function_call;

    #[test]
    fn rewrites_trim_both_as_btrim() {
        assert_snapshot!(
            apply_code_action(
                rewrite_trim_as_function_call,
                "select TRIM($0BOTH 'x' FROM 'xxhixx');",
            ),
            @"select btrim('xxhixx', 'x');"
        );
    }

    #[test]
    fn rewrites_trim_both_without_characters() {
        assert_snapshot!(
            apply_code_action(
                rewrite_trim_as_function_call,
                "select trim(BOTH FROM lower(val$0ue));",
            ),
            @"select btrim(lower(value));"
        );
    }

    #[test]
    fn rewrites_trim_leading_as_ltrim() {
        assert_snapshot!(
            apply_code_action(
                rewrite_trim_as_function_call,
                "select TRIM($0LEADING 'x' FROM 'xxhixx');",
            ),
            @"select ltrim('xxhixx', 'x');"
        );
    }

    #[test]
    fn rewrites_trim_leading_without_characters() {
        assert_snapshot!(
            apply_code_action(
                rewrite_trim_as_function_call,
                "select trim(leading from val$0ue);",
            ),
            @"select ltrim(value);"
        );
    }

    #[test]
    fn rewrites_from_first_function_style_form() {
        assert_snapshot!(
            apply_code_action(
                rewrite_trim_as_function_call,
                "select trim(leading from val$0ue, characters);",
            ),
            @"select ltrim(value, characters);"
        );
    }

    #[test]
    fn rewrites_trim_trailing_as_rtrim() {
        assert_snapshot!(
            apply_code_action(
                rewrite_trim_as_function_call,
                "select TRIM($0TRAILING 'x' FROM 'xxhixx');",
            ),
            @"select rtrim('xxhixx', 'x');"
        );
    }

    #[test]
    fn rewrites_trim_trailing_without_characters() {
        assert_snapshot!(
            apply_code_action(
                rewrite_trim_as_function_call,
                "select trim(trailing from lower(val$0ue));",
            ),
            @"select rtrim(lower(value));"
        );
    }

    #[test]
    fn rewrites_comma_separated_trailing_variant() {
        assert_snapshot!(
            apply_code_action(
                rewrite_trim_as_function_call,
                "select trim(trailing val$0ue, characters);",
            ),
            @"select rtrim(value, characters);"
        );
    }

    #[test]
    fn rewrites_innermost_call_and_preserves_aggregate_clause() {
        assert_snapshot!(
            apply_code_action(
                rewrite_trim_as_function_call,
                "select trim(both 'x' from trim(leading fr$0om value)) filter (where ok);",
            ),
            @"select trim(both 'x' from ltrim(value)) filter (where ok);"
        );
    }

    #[test]
    fn not_applicable_without_trim_side() {
        assert!(code_action_not_applicable(
            rewrite_trim_as_function_call,
            "select trim(val$0ue);"
        ));
    }

    #[test]
    fn not_applicable_to_nonstandard_both_or_leading_forms() {
        assert!(code_action_not_applicable(
            rewrite_trim_as_function_call,
            "select trim(both value$0, 'x');"
        ));
        assert!(code_action_not_applicable(
            rewrite_trim_as_function_call,
            "select trim(leading val$0ue);"
        ));
    }
}
