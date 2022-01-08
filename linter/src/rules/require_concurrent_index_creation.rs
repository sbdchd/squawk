use crate::rules::utils::tables_created_in_transaction;
use crate::violations::{RuleViolation, RuleViolationKind};
use squawk_parser::ast::{ObjectType, RawStmt, Stmt};

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
            Stmt::DropStmt(stmt) if !stmt.concurrent && stmt.remove_type == ObjectType::Index => {
                errs.push(RuleViolation::new(
                    RuleViolationKind::RequireConcurrentIndexCreation,
                    raw_stmt.into(),
                    None,
                ))
            }
            _ => continue,
        }
    }
    errs
}

#[cfg(test)]
mod test_rules {
    use crate::check_sql;
    use crate::violations::RuleViolationKind;
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

        assert_debug_snapshot!(check_sql(bad_sql, &["prefer-robust-stmts".into()]));

        let ok_sql = r#"
  -- use CONCURRENTLY
  CREATE INDEX CONCURRENTLY "field_name_idx" ON "table_name" ("field_name");
  "#;
        assert_debug_snapshot!(check_sql(ok_sql, &["prefer-robust-stmts".into()]));
    }

    #[test]
    fn test_drop_index_concurrently() {
        let bad_sql = r#"
  -- instead of
  DROP INDEX IF EXISTS "field_name_idx";
  "#;
        let res = check_sql(bad_sql, &[]).unwrap();
        assert_eq!(res.len(), 1);
        assert_eq!(
            res[0].kind,
            RuleViolationKind::RequireConcurrentIndexCreation
        );

        let ok_sql = r#"
  DROP INDEX CONCURRENTLY IF EXISTS "field_name_idx";
  "#;
        assert_eq!(check_sql(ok_sql, &[]), Ok(vec![]));
    }

    #[test]
    fn regression_false_positive_drop_type() {
        let sql = r#"
  DROP TYPE IF EXISTS foo;
  "#;
        assert_eq!(check_sql(sql, &[]), Ok(vec![]));
    }

    #[test]
    fn regression_false_positive_drop_table() {
        let sql = r#"
  DROP TABLE IF EXISTS some_table;
  "#;
        assert_eq!(check_sql(sql, &[]), Ok(vec![]));
    }

    #[test]
    fn regression_false_positive_drop_trigger() {
        let sql = r#"
  DROP TRIGGER IF EXISTS trigger on foo_table;
  "#;
        assert_eq!(check_sql(sql, &[]), Ok(vec![]));
    }
}
