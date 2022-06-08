use crate::rules::utils::tables_created_in_transaction;
use crate::violations::{RuleViolation, RuleViolationKind};
use squawk_parser::ast::{RawStmt, Stmt};

#[must_use]
pub fn require_concurrent_index_creation(tree: &[RawStmt]) -> Vec<RuleViolation> {
    let tables_created = tables_created_in_transaction(tree);
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
    use crate::{check_sql, violations::RuleViolationKind};
    use insta::assert_debug_snapshot;

    /// ```sql
    /// -- instead of
    /// CREATE INDEX "field_name_idx" ON "table_name" ("field_name");
    /// -- use CONCURRENTLY
    /// CREATE INDEX CONCURRENTLY "field_name_idx" ON "table_name" ("field_name");
    /// ```
    #[test]
    fn test_adding_index_non_concurrently() {
        let bad_sql = r#"
  -- instead of
  CREATE INDEX "field_name_idx" ON "table_name" ("field_name");
  "#;

        assert_debug_snapshot!(check_sql(bad_sql, &[RuleViolationKind::PreferRobustStmts]));

        let ok_sql = r#"
  -- use CONCURRENTLY
  CREATE INDEX CONCURRENTLY "field_name_idx" ON "table_name" ("field_name");
  "#;
        assert_debug_snapshot!(check_sql(ok_sql, &[RuleViolationKind::PreferRobustStmts]));
    }
}
