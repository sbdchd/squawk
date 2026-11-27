use squawk_diagnostics::Edit;
use squawk_rowan::TextSize;
use squawk_sema::file::InFile;
use squawk_syntax::{
    ast::{self, AstNode},
    quote::quote_string_literal,
};

use squawk_ide_db::Db;
use squawk_ide_db::offsets::token_from_offset;

use super::{ActionKind, CodeAction};

pub(super) fn rewrite_normalize_as_function_call(
    db: &dyn Db,
    position: InFile<TextSize>,
    actions: &mut Vec<CodeAction>,
) -> Option<()> {
    let token = token_from_offset(db, position)?;
    let normalize_fn = token.parent_ancestors().find_map(ast::NormalizeFn::cast)?;
    normalize_fn.expr()?;
    let form = normalize_fn.unicode_normal_form();
    if normalize_fn.comma_token().is_some() && form.is_none() {
        return None;
    }

    let mut edits = vec![Edit::replace(
        normalize_fn.normalize_token()?.text_range(),
        "pg_catalog.normalize",
    )];
    if let Some(form) = form {
        let text = form.syntax().text().to_string().to_ascii_uppercase();
        edits.push(Edit::replace(
            form.syntax().text_range(),
            quote_string_literal(&text),
        ));
    }

    actions.push(CodeAction {
        title: "Rewrite as function call `pg_catalog.normalize()`".to_owned(),
        edits,
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::test_utils::{
        apply_code_action, code_action_not_applicable, code_action_not_applicable_with_errors,
    };

    use super::rewrite_normalize_as_function_call;

    #[test]
    fn rewrites_normalize_as_function_call() {
        assert_snapshot!(
            apply_code_action(
                rewrite_normalize_as_function_call,
                r"select NORM$0ALIZE(U&'\0061\0301', NFC);",
            ),
            @r"select pg_catalog.normalize(U&'\0061\0301', 'NFC');"
        );
    }

    #[test]
    fn rewrites_default_normal_form_with_cursor_in_argument() {
        assert_snapshot!(
            apply_code_action(
                rewrite_normalize_as_function_call,
                "select normalize(val$0ue);",
            ),
            @"select pg_catalog.normalize(value);"
        );
    }

    #[test]
    fn canonicalizes_normal_form_and_preserves_call_clauses() {
        assert_snapshot!(
            apply_code_action(
                rewrite_normalize_as_function_call,
                "select normalize(value, nf$0kd) FILTER (WHERE ok);",
            ),
            @"select pg_catalog.normalize(value, 'NFKD') FILTER (WHERE ok);"
        );
    }

    #[test]
    fn rewrites_innermost_normalize_call() {
        assert_snapshot!(
            apply_code_action(
                rewrite_normalize_as_function_call,
                "select normalize(normal$0ize(value, nfd), nfkc);",
            ),
            @"select normalize(pg_catalog.normalize(value, 'NFD'), nfkc);"
        );
    }

    #[test]
    fn not_applicable_to_already_qualified_call() {
        assert!(code_action_not_applicable(
            rewrite_normalize_as_function_call,
            "select pg_catalog.norm$0alize(value, 'NFC');"
        ));
    }

    #[test]
    fn not_applicable_to_non_normal_form_argument() {
        assert!(code_action_not_applicable_with_errors(
            rewrite_normalize_as_function_call,
            "select normalize(value, fo$0rm);"
        ));
    }
}
