use crate::{
    versions::Version,
    violations::{RuleViolation, RuleViolationKind},
};

use squawk_parser::ast::{ObjectType, RawStmt, Stmt};

#[must_use]
pub fn require_concurrent_index_deletion(
    tree: &[RawStmt],
    _pg_version: Option<Version>,
    _assume_in_transaction: bool,
) -> Vec<RuleViolation> {
    let mut errs = vec![];
    for raw_stmt in tree {
        match &raw_stmt.stmt {
            Stmt::DropStmt(stmt) if !stmt.concurrent && stmt.remove_type == ObjectType::Index => {
                errs.push(RuleViolation::new(
                    RuleViolationKind::RequireConcurrentIndexDeletion,
                    raw_stmt.into(),
                    None,
                ));
            }
            _ => continue,
        }
    }
    errs
}

#[cfg(test)]
mod test_rules {
    use crate::{
        check_sql_with_rule,
        violations::{RuleViolation, RuleViolationKind},
    };
    fn lint_sql(sql: &str) -> Vec<RuleViolation> {
        check_sql_with_rule(
            sql,
            &RuleViolationKind::RequireConcurrentIndexDeletion,
            None,
            false,
        )
        .unwrap()
    }

    #[test]
    fn drop_index_concurrently() {
        let bad_sql = r#"
  -- instead of
  DROP INDEX IF EXISTS "field_name_idx";
  "#;
        let res = lint_sql(bad_sql);
        assert_eq!(res.len(), 1);
        assert_eq!(
            res[0].kind,
            RuleViolationKind::RequireConcurrentIndexDeletion
        );

        let ok_sql = r#"
  DROP INDEX CONCURRENTLY IF EXISTS "field_name_idx";
  "#;
        assert_eq!(lint_sql(ok_sql), vec![]);
    }

    #[test]
    fn regression_false_positive_drop_type() {
        let sql = r"
  DROP TYPE IF EXISTS foo;
  ";
        assert_eq!(lint_sql(sql), vec![]);
    }

    #[test]
    fn regression_false_positive_drop_table() {
        let sql = r"
  DROP TABLE IF EXISTS some_table;
  ";
        assert_eq!(lint_sql(sql), vec![]);
    }

    #[test]
    fn regression_false_positive_drop_trigger() {
        let sql = r"
  DROP TRIGGER IF EXISTS trigger on foo_table;
  ";
        assert_eq!(lint_sql(sql), vec![]);
    }
}
