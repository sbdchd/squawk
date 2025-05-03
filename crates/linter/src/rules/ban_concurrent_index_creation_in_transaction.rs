use crate::versions::Version;
use crate::violations::{RuleViolation, RuleViolationKind};

use squawk_parser::ast::{RawStmt, Stmt, TransactionStmtKind};

#[must_use]
pub fn ban_concurrent_index_creation_in_transaction(
    tree: &[RawStmt],
    _pg_version: Option<Version>,
    assume_in_transaction: bool,
) -> Vec<RuleViolation> {
    let mut in_transaction = assume_in_transaction;
    let mut errs = vec![];
    for raw_stmt in tree {
        match &raw_stmt.stmt {
            Stmt::TransactionStmt(stmt) => {
                if (stmt.kind == TransactionStmtKind::Begin
                    || stmt.kind == TransactionStmtKind::Start)
                    && !in_transaction
                {
                    in_transaction = true;
                }
                if stmt.kind == TransactionStmtKind::Commit {
                    in_transaction = false;
                }
            }
            Stmt::IndexStmt(stmt) => {
                if stmt.concurrent && in_transaction {
                    if assume_in_transaction && tree.len() == 1 {
                        // Migration tools should not require the transaction here so this is usually safe
                        continue;
                    }
                    errs.push(RuleViolation::new(
                        RuleViolationKind::BanConcurrentIndexCreationInTransaction,
                        raw_stmt.into(),
                        None,
                    ));
                }
            }
            _ => continue,
        }
    }
    errs
}

#[cfg(test)]
mod test_rules {
    use insta::assert_debug_snapshot;

    use crate::{
        check_sql_with_rule,
        violations::{RuleViolation, RuleViolationKind},
    };

    fn lint_sql(sql: &str) -> Vec<RuleViolation> {
        check_sql_with_rule(
            sql,
            &RuleViolationKind::BanConcurrentIndexCreationInTransaction,
            None,
            false,
        )
        .unwrap()
    }

    fn lint_sql_assuming_in_transaction(sql: &str) -> Vec<RuleViolation> {
        check_sql_with_rule(
            sql,
            &RuleViolationKind::BanConcurrentIndexCreationInTransaction,
            None,
            true,
        )
        .unwrap()
    }

    #[test]
    fn adding_index_concurrently_in_transaction() {
        let bad_sql = r#"
  -- instead of
  BEGIN;
  CREATE INDEX CONCURRENTLY "field_name_idx" ON "table_name" ("field_name");
  COMMIT;
  "#;

        assert_debug_snapshot!(lint_sql(bad_sql));

        let ok_sql = r#"
  -- run outside a transaction
  CREATE INDEX CONCURRENTLY "field_name_idx" ON "table_name" ("field_name");
  "#;
        assert_debug_snapshot!(lint_sql(ok_sql));
    }

    #[test]
    fn adding_index_concurrently_in_transaction_with_assume_in_transaction() {
        let bad_sql = r#"
  -- instead of
  CREATE UNIQUE INDEX CONCURRENTLY "field_name_idx" ON "table_name" ("field_name");
  ALTER TABLE "table_name" ADD CONSTRAINT "field_name_id" UNIQUE USING INDEX "field_name_idx";
  "#;

        assert_debug_snapshot!(lint_sql_assuming_in_transaction(bad_sql));

        let ok_sql = r#"
  -- run index creation in a standalone migration
  CREATE UNIQUE INDEX CONCURRENTLY "field_name_idx" ON "table_name" ("field_name");
  "#;
        assert_debug_snapshot!(lint_sql_assuming_in_transaction(ok_sql));
    }

    #[test]
    fn adding_index_concurrently_in_transaction_with_assume_in_transaction_but_outside() {
        let ok_sql = r#"
  -- the following will work too
  COMMIT;
  CREATE UNIQUE INDEX CONCURRENTLY "field_name_idx" ON "table_name" ("field_name");
  BEGIN;
  ALTER TABLE "table_name" ADD CONSTRAINT "field_name_id" UNIQUE USING INDEX "field_name_idx";
  "#;

        assert_debug_snapshot!(lint_sql_assuming_in_transaction(ok_sql));
    }
}
