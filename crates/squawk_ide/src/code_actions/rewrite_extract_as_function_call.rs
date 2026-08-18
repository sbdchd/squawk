use rowan::TextSize;
use salsa::Database as Db;
use squawk_linter::Edit;
use squawk_syntax::{
    ast::{self, AstNode, LitKind},
    quote::quote_string_literal,
};

use crate::{file::InFile, offsets::token_from_offset};

use super::{ActionKind, CodeAction};

pub(super) fn rewrite_extract_as_function_call(
    db: &dyn Db,
    position: InFile<TextSize>,
    actions: &mut Vec<CodeAction>,
) -> Option<()> {
    let token = token_from_offset(db, position)?;
    let extract = token.parent_ancestors().find_map(ast::ExtractFn::cast)?;
    let field = extract.extract_field()?;
    let expr = extract.expr()?;

    let field = extract_field_argument(&field)?;
    let replacement = format!("pg_catalog.extract({field}, {})", expr.syntax().text());

    actions.push(CodeAction {
        title: "Rewrite as function call `pg_catalog.extract()`".to_owned(),
        edits: vec![Edit::replace(extract.syntax().text_range(), replacement)],
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

fn extract_field_argument(field: &ast::ExtractField) -> Option<String> {
    match field {
        ast::ExtractField::ExtractFieldLiteral(field)
            if matches!(
                field.literal()?.kind()?,
                LitKind::String(_)
                    | LitKind::EscString(_)
                    | LitKind::NationalString(_)
                    | LitKind::UnicodeEscString(_)
                    | LitKind::DollarQuotedString(_)
            ) =>
        {
            Some(field.syntax().text().to_string())
        }
        ast::ExtractField::ExtractFieldName(name) => Some(quote_string_literal(&name.text())),
        ast::ExtractField::ExtractFieldLiteral(_) => None,
    }
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::code_actions::test_utils::{apply_code_action, code_action_not_applicable};

    use super::rewrite_extract_as_function_call;

    #[test]
    fn rewrites_extract_as_function_call() {
        assert_snapshot!(
            apply_code_action(
                rewrite_extract_as_function_call,
                "select EXTR$0ACT(YEAR FROM DATE '2026-08-17');",
            ),
            @"select pg_catalog.extract('year', DATE '2026-08-17');"
        );
    }

    #[test]
    fn applies_with_cursor_in_source_expression() {
        assert_snapshot!(
            apply_code_action(
                rewrite_extract_as_function_call,
                "select extract(epoch from current_time$0stamp);",
            ),
            @"select pg_catalog.extract('epoch', current_timestamp);"
        );
    }

    #[test]
    fn preserves_string_field_and_aggregate_clause() {
        assert_snapshot!(
            apply_code_action(
                rewrite_extract_as_function_call,
                "select extract('EPOCH' from ts$0) filter (where ok);",
            ),
            @"select pg_catalog.extract('EPOCH', ts) filter (where ok);"
        );
    }

    #[test]
    fn converts_quoted_field_to_string() {
        assert_snapshot!(
            apply_code_action(
                rewrite_extract_as_function_call,
                r#"select extract("time""zone" from ts$0);"#,
            ),
            @"select pg_catalog.extract('time\"zone', ts);"
        );
    }

    #[test]
    fn not_applicable_outside_extract() {
        assert!(code_action_not_applicable(
            rewrite_extract_as_function_call,
            "select date_part('year', ts$0);"
        ));
    }

    #[test]
    fn converts_unicode_identifier_with_custom_escape() {
        assert_snapshot!(
            apply_code_action(
                rewrite_extract_as_function_call,
                r#"select extract(U&"@0079ear" UESCAPE '@' from ts$0);"#,
            ),
            @"select pg_catalog.extract('year', ts);"
        );
    }

    #[test]
    fn does_not_reinterpret_bit_string_field_as_identifier() {
        assert!(code_action_not_applicable(
            rewrite_extract_as_function_call,
            "select extract(B'0101' from ts$0);"
        ));
    }
}
