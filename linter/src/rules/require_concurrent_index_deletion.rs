use crate::violations::{RuleViolation, RuleViolationKind};
use squawk_parser::ast::{ObjectType, RawStmt, Stmt};

#[must_use]
pub fn require_concurrent_index_deletion(tree: &[RawStmt]) -> Vec<RuleViolation> {
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
    use crate::check_sql;
    use crate::violations::RuleViolationKind;

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
            RuleViolationKind::RequireConcurrentIndexDeletion
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
