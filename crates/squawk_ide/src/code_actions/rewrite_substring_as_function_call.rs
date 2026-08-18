use rowan::TextSize;
use salsa::Database as Db;
use squawk_linter::Edit;
use squawk_syntax::ast::{self, AstNode, SubstringArgs};

use crate::{file::InFile, offsets::token_from_offset};

use super::{ActionKind, CodeAction};

pub(super) fn rewrite_substring_as_function_call(
    db: &dyn Db,
    position: InFile<TextSize>,
    actions: &mut Vec<CodeAction>,
) -> Option<()> {
    let token = token_from_offset(db, position)?;
    let substring = token.parent_ancestors().find_map(ast::SubstringFn::cast)?;

    let arguments = match substring.substring_args()? {
        SubstringArgs::SubstringFromFor(args) => {
            let string = args.string()?;
            let start = args.start()?;
            match args.count() {
                Some(count) => format!(
                    "{}, {}, {}",
                    string.syntax().text(),
                    start.syntax().text(),
                    count.syntax().text()
                ),
                None => format!("{}, {}", string.syntax().text(), start.syntax().text()),
            }
        }
        SubstringArgs::SubstringForFrom(args) => {
            let string = args.string()?;
            let count = args.count()?;
            let start = args
                .start()
                .map(|start| start.syntax().text().to_string())
                .unwrap_or_else(|| "1".to_owned());
            format!(
                "{}, {start}, {}",
                string.syntax().text(),
                count.syntax().text()
            )
        }
        SubstringArgs::SubstringSimilarEscape(args) => format!(
            "{}, {}, {}",
            args.string()?.syntax().text(),
            args.pattern()?.syntax().text(),
            args.escape()?.syntax().text()
        ),
        SubstringArgs::SubstringExprs(_) => return None,
    };

    actions.push(CodeAction {
        title: "Rewrite as `substring` function call".to_owned(),
        edits: vec![Edit::replace(
            substring.syntax().text_range(),
            format!("substring({arguments})"),
        )],
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::code_actions::test_utils::{apply_code_action, code_action_not_applicable};

    use super::rewrite_substring_as_function_call;

    #[test]
    fn rewrites_substring_from_for_as_function_call() {
        assert_snapshot!(
            apply_code_action(
                rewrite_substring_as_function_call,
                "select SUBS$0TRING('hello world' FROM 3 FOR 5);",
            ),
            @"select substring('hello world', 3, 5);"
        );
    }

    #[test]
    fn rewrites_for_before_from_in_function_argument_order() {
        assert_snapshot!(
            apply_code_action(
                rewrite_substring_as_function_call,
                "select substring(value FOR leng$0th FROM start);",
            ),
            @"select substring(value, start, length);"
        );
    }

    #[test]
    fn rewrites_optional_from_only_form() {
        assert_snapshot!(
            apply_code_action(
                rewrite_substring_as_function_call,
                "select substring(lower(val$0ue) FROM start);",
            ),
            @"select substring(lower(value), start);"
        );
    }

    #[test]
    fn rewrites_optional_for_only_form() {
        assert_snapshot!(
            apply_code_action(
                rewrite_substring_as_function_call,
                "select substring(value FOR leng$0th);",
            ),
            @"select substring(value, 1, length);"
        );
    }

    #[test]
    fn rewrites_similar_escape_form() {
        assert_snapshot!(
            apply_code_action(
                rewrite_substring_as_function_call,
                r##"select SUBS$0TRING('hello world' SIMILAR '%#"o w#"%' ESCAPE '#');"##,
            ),
            @r##"select substring('hello world', '%#"o w#"%', '#');"##
        );
    }

    #[test]
    fn rewrites_similar_escape_expression_arguments() {
        assert_snapshot!(
            apply_code_action(
                rewrite_substring_as_function_call,
                "select substring(lower(value) similar ('%' || patt$0ern) escape coalesce(escape_char, '#'));",
            ),
            @"select substring(lower(value), ('%' || pattern), coalesce(escape_char, '#'));"
        );
    }

    #[test]
    fn rewrites_innermost_from_for_call_and_preserves_aggregate_clause() {
        assert_snapshot!(
            apply_code_action(
                rewrite_substring_as_function_call,
                "select substring(substring(value FR$0OM 2) FROM 3 FOR 4) FILTER (WHERE ok);",
            ),
            @"select substring(substring(value, 2) FROM 3 FOR 4) FILTER (WHERE ok);"
        );
    }

    #[test]
    fn rewrites_innermost_similar_escape_call_and_preserves_aggregate_clause() {
        assert_snapshot!(
            apply_code_action(
                rewrite_substring_as_function_call,
                "select substring(substring(value similar patt$0ern escape esc) similar outer_pattern escape outer_esc) filter (where ok);",
            ),
            @"select substring(substring(value, pattern, esc) similar outer_pattern escape outer_esc) filter (where ok);"
        );
    }

    #[test]
    fn not_applicable_to_comma_separated_call() {
        assert!(code_action_not_applicable(
            rewrite_substring_as_function_call,
            "select substring(value, 2$0, 3);"
        ));
    }
}
