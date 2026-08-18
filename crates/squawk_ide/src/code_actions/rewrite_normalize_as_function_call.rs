use rowan::TextSize;
use salsa::Database as Db;
use squawk_linter::Edit;
use squawk_syntax::{
    SyntaxKind,
    ast::{self, AstNode},
    quote::quote_string_literal,
};

use crate::{file::InFile, offsets::token_from_offset};

use super::{ActionKind, CodeAction};

pub(super) fn rewrite_normalize_as_function_call(
    db: &dyn Db,
    position: InFile<TextSize>,
    actions: &mut Vec<CodeAction>,
) -> Option<()> {
    let token = token_from_offset(db, position)?;
    let (name, normal_form) = token.parent_ancestors().find_map(|node| {
        let call = ast::CallExpr::cast(node)?;
        let ast::Expr::NameRef(name) = call.expr()? else {
            return None;
        };
        if name.syntax().first_token()?.kind() != SyntaxKind::NORMALIZE_KW {
            return None;
        }

        let args = call.arg_list()?.args().collect::<Vec<_>>();
        let normal_form = match args.as_slice() {
            [arg] => {
                arg.expr()?;
                None
            }
            [arg, form] => {
                arg.expr()?;
                Some(normal_form(&form.expr()?)?)
            }
            _ => return None,
        };

        Some((name, normal_form))
    })?;

    let mut edits = vec![Edit::replace(
        name.syntax().text_range(),
        "pg_catalog.normalize",
    )];
    if let Some((range, form)) = normal_form {
        edits.push(Edit::replace(range, quote_string_literal(&form)));
    }

    actions.push(CodeAction {
        title: "Rewrite as function call `pg_catalog.normalize()`".to_owned(),
        edits,
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

fn normal_form(expr: &ast::Expr) -> Option<(rowan::TextRange, String)> {
    let ast::Expr::NameRef(name) = expr else {
        return None;
    };
    let token = name.syntax().first_token()?;
    if !matches!(
        token.kind(),
        SyntaxKind::NFC_KW | SyntaxKind::NFD_KW | SyntaxKind::NFKC_KW | SyntaxKind::NFKD_KW
    ) {
        return None;
    }
    Some((
        expr.syntax().text_range(),
        token.text().to_ascii_uppercase(),
    ))
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::code_actions::test_utils::{apply_code_action, code_action_not_applicable};

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
        assert!(code_action_not_applicable(
            rewrite_normalize_as_function_call,
            "select normalize(value, fo$0rm);"
        ));
    }
}
