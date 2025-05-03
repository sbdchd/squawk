use crate::rules::utils::tables_created_in_transaction;
use crate::versions::Version;
use crate::violations::{RuleViolation, RuleViolationKind};

use squawk_parser::ast::{RawStmt, Stmt};

#[must_use]
pub fn require_concurrent_index_creation(
    tree: &[RawStmt],
    _pg_version: Option<Version>,
    assume_in_transaction: bool,
) -> Vec<RuleViolation> {
    let tables_created = tables_created_in_transaction(tree, assume_in_transaction);
    let mut errs = vec![];
    for raw_stmt in tree {
        match &raw_stmt.stmt {
            Stmt::IndexStmt(stmt) => {
                let range = &stmt.relation;
                let tbl_name = &range.relname;
                if !stmt.concurrent && !tables_created.contains(tbl_name) {
                    errs.push(RuleViolation::new(
                        RuleViolationKind::RequireConcurrentIndexCreation,
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
            &RuleViolationKind::RequireConcurrentIndexCreation,
            None,
            false,
        )
        .unwrap()
    }

    fn lint_sql_assuming_in_transaction(sql: &str) -> Vec<RuleViolation> {
        check_sql_with_rule(
            sql,
            &RuleViolationKind::RequireConcurrentIndexCreation,
            None,
            true,
        )
        .unwrap()
    }

    #[test]
    fn ensure_ignored_when_new_table() {
        let sql = r#"
BEGIN;
CREATE TABLE "core_foo" (
"id" serial NOT NULL PRIMARY KEY,
"tenant_id" integer NULL
);
CREATE INDEX "core_foo_tenant_id_4d397ef9" ON "core_foo" ("tenant_id");
COMMIT;
    "#;

        assert_debug_snapshot!(lint_sql(sql));
    }

    #[test]
    fn ensure_ignored_when_new_table_with_assume_in_transaction() {
        let sql = r#"
CREATE TABLE "core_foo" (
"id" serial NOT NULL PRIMARY KEY,
"tenant_id" integer NULL
);
CREATE INDEX "core_foo_tenant_id_4d397ef9" ON "core_foo" ("tenant_id");
    "#;

        assert_debug_snapshot!(lint_sql_assuming_in_transaction(sql));
    }

    /// ```sql
    /// -- instead of
    /// CREATE INDEX "field_name_idx" ON "table_name" ("field_name");
    /// -- use CONCURRENTLY
    /// CREATE INDEX CONCURRENTLY "field_name_idx" ON "table_name" ("field_name");
    /// ```
    #[test]
    fn adding_index_non_concurrently() {
        let bad_sql = r#"
  -- instead of
  CREATE INDEX "field_name_idx" ON "table_name" ("field_name");
  "#;

        assert_debug_snapshot!(lint_sql(bad_sql));

        let ok_sql = r#"
  -- use CONCURRENTLY
  CREATE INDEX CONCURRENTLY "field_name_idx" ON "table_name" ("field_name");
  "#;
        assert_debug_snapshot!(lint_sql(ok_sql));
    }
}
