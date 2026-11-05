use squawk_diagnostics::{Edit, Fix};
use squawk_syntax::ast::{self, AstNode};

use crate::{Linter, Rule, Violation, transactions::TransactionState};

fn concurrently_fix(drop_index: &ast::DropIndex) -> Option<Fix> {
    let index_token = drop_index.index_token()?;
    let at = index_token.text_range().end();
    let edit = Edit::insert(" concurrently", at);
    Some(Fix::new("Add `concurrently`", vec![edit]))
}

pub(crate) fn require_concurrent_index_deletion(ctx: &mut Linter, file: &ast::SourceFile) {
    let mut transaction = TransactionState::new(ctx.settings.assume_in_transaction);
    for stmt in file.stmts() {
        transaction.update(&stmt);
        if let ast::Stmt::DropIndex(drop_index) = stmt
            && drop_index.concurrently_token().is_none()
        {
            let violation = Violation::for_node(
                Rule::RequireConcurrentIndexDeletion,
                "a normal `drop index` acquires an `access exclusive` lock on the table, blocking other accesses until the index drop can complete".into(),
                drop_index.syntax(),
            );
            let violation = if transaction.inside_transaction() {
                violation.help("drop the index `concurrently` outside of a transaction")
            } else if drop_index.index_refs().nth(1).is_some() {
                violation.help("drop each index in its own `drop index concurrently` statement")
            } else if let Some(ast::DropBehavior::Cascade(_)) = drop_index.drop_behavior() {
                violation.help(
                    "drop the dependent objects first, then drop the index `concurrently` without `cascade`",
                )
            } else {
                violation
                    .help("drop the index `concurrently`")
                    .fix(concurrently_fix(&drop_index))
            };
            ctx.report(violation);
        }
    }
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::{
        Rule,
        test_utils::{fix_sql, lint_errors, lint_ok},
    };

    #[must_use]
    fn fix(sql: &str) -> String {
        fix_sql(sql, Rule::RequireConcurrentIndexDeletion)
    }

    #[test]
    fn fix_add_concurrently_simple() {
        let sql = "drop index i;";
        let result = fix(sql);
        assert_snapshot!(result, @"drop index concurrently i;");
    }

    #[test]
    fn fix_add_concurrently_if_exists() {
        let sql = r#"DROP INDEX IF EXISTS "field_name_idx";"#;
        let result = fix(sql);
        assert_snapshot!(result, @r#"DROP INDEX concurrently IF EXISTS "field_name_idx";"#);
    }

    #[test]
    fn multiple_indexes_err() {
        let sql = r#"
drop index a, b;
        "#;
        assert_snapshot!(lint_errors(sql, Rule::RequireConcurrentIndexDeletion));
    }

    #[test]
    fn cascade_err() {
        let sql = r#"
drop index c cascade;
        "#;
        assert_snapshot!(lint_errors(sql, Rule::RequireConcurrentIndexDeletion));
    }

    #[test]
    fn drop_index_missing_concurrently_err() {
        let sql = r#"
  -- instead of
  DROP INDEX IF EXISTS "field_name_idx";
        "#;
        assert_snapshot!(lint_errors(sql, Rule::RequireConcurrentIndexDeletion));
    }

    #[test]
    fn drop_index_concurrently_ok() {
        let sql = r#"
DROP INDEX CONCURRENTLY IF EXISTS "field_name_idx";
        "#;
        lint_ok(sql, Rule::RequireConcurrentIndexDeletion);
    }

    #[test]
    fn regression_false_positive_drop_type_ok() {
        let sql = r#"
DROP INDEX CONCURRENTLY IF EXISTS "field_name_idx";
        "#;
        lint_ok(sql, Rule::RequireConcurrentIndexDeletion);
    }

    #[test]
    fn regression_false_positive_drop_table_ok() {
        let sql = r#"
DROP TABLE IF EXISTS some_table;
        "#;
        lint_ok(sql, Rule::RequireConcurrentIndexDeletion);
    }

    #[test]
    fn regression_false_positive_drop_trigger_ok() {
        let sql = r#"
DROP TRIGGER IF EXISTS trigger on foo_table;
        "#;
        lint_ok(sql, Rule::RequireConcurrentIndexDeletion);
    }

    #[test]
    fn in_transaction_err() {
        let sql = r#"
begin;
drop index i;
commit;
        "#;
        assert_snapshot!(lint_errors(sql, Rule::RequireConcurrentIndexDeletion));
    }
}
