use rowan::TextSize;
use salsa::Database as Db;
use squawk_syntax::ast::{self, AstNode};
use squawk_syntax::quote::quote_column_alias;

use crate::{column_name::ColumnName, db::File, offsets::token_from_offset};

use super::{ActionKind, CodeAction};

pub(super) fn add_explicit_alias(
    db: &dyn Db,
    file: File,
    actions: &mut Vec<CodeAction>,
    offset: TextSize,
) -> Option<()> {
    let token = token_from_offset(db, file, offset)?;
    let target = token.parent_ancestors().find_map(ast::Target::cast)?;

    if target.as_name().is_some() {
        return None;
    }

    if let Some(ast::Expr::FieldExpr(field_expr)) = target.expr()
        && field_expr.star_token().is_some()
    {
        return None;
    }

    let alias = ColumnName::from_target(target.clone()).and_then(|c| c.0.to_string())?;

    let expr_end = target.expr().map(|e| e.syntax().text_range().end())?;

    let quoted_alias = quote_column_alias(&alias);
    let replacement = format!(" as {}", quoted_alias);

    actions.push(CodeAction {
        title: "Add explicit alias".to_owned(),
        edits: vec![squawk_linter::Edit::insert(replacement, expr_end)],
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::code_actions::test_utils::{apply_code_action, code_action_not_applicable};

    use super::add_explicit_alias;

    #[test]
    fn add_explicit_alias_simple_column() {
        assert_snapshot!(apply_code_action(
            add_explicit_alias,
            "select col_na$0me from t;"),
            @"select col_name as col_name from t;"
        );
    }

    #[test]
    fn add_explicit_alias_quoted_identifier() {
        assert_snapshot!(apply_code_action(
            add_explicit_alias,
            r#"select "b"$0 from t;"#),
            @r#"select "b" as b from t;"#
        );
    }

    #[test]
    fn add_explicit_alias_field_expr() {
        assert_snapshot!(apply_code_action(
            add_explicit_alias,
            "select t.col$0umn from t;"),
            @"select t.column as column from t;"
        );
    }

    #[test]
    fn add_explicit_alias_function_call() {
        assert_snapshot!(apply_code_action(
            add_explicit_alias,
            "select cou$0nt(*) from t;"),
            @"select count(*) as count from t;"
        );
    }

    #[test]
    fn add_explicit_alias_cast_to_type() {
        assert_snapshot!(apply_code_action(
            add_explicit_alias,
            "select '1'::bigi$0nt from t;"),
            @"select '1'::bigint as int8 from t;"
        );
    }

    #[test]
    fn add_explicit_alias_cast_column() {
        assert_snapshot!(apply_code_action(
            add_explicit_alias,
            "select col_na$0me::text from t;"),
            @"select col_name::text as col_name from t;"
        );
    }

    #[test]
    fn add_explicit_alias_case_expr() {
        assert_snapshot!(apply_code_action(
            add_explicit_alias,
            "select ca$0se when true then 'a' end from t;"),
            @"select case when true then 'a' end as case from t;"
        );
    }

    #[test]
    fn add_explicit_alias_case_with_else() {
        assert_snapshot!(apply_code_action(
            add_explicit_alias,
            "select ca$0se when true then 'a' else now()::text end from t;"),
            @"select case when true then 'a' else now()::text end as now from t;"
        );
    }

    #[test]
    fn add_explicit_alias_array() {
        assert_snapshot!(apply_code_action(
            add_explicit_alias,
            "select arr$0ay[1, 2, 3] from t;"),
            @"select array[1, 2, 3] as array from t;"
        );
    }

    #[test]
    fn add_explicit_alias_not_applicable_already_has_alias() {
        assert!(code_action_not_applicable(
            add_explicit_alias,
            "select col_name$0 as foo from t;"
        ));
    }

    #[test]
    fn add_explicit_alias_unknown_column() {
        assert_snapshot!(apply_code_action(
            add_explicit_alias,
            "select 1 $0+ 2 from t;"),
            @r#"select 1 + 2 as "?column?" from t;"#
        );
    }

    #[test]
    fn add_explicit_alias_not_applicable_star() {
        assert!(code_action_not_applicable(
            add_explicit_alias,
            "select $0* from t;"
        ));
    }

    #[test]
    fn add_explicit_alias_not_applicable_qualified_star() {
        assert!(code_action_not_applicable(
            add_explicit_alias,
            "with t as (select 1 a) select t.*$0 from t;"
        ));
    }

    #[test]
    fn add_explicit_alias_literal() {
        assert_snapshot!(apply_code_action(
            add_explicit_alias,
            "select 'foo$0' from t;"),
            @r#"select 'foo' as "?column?" from t;"#
        );
    }
}
