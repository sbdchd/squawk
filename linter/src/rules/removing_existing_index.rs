use crate::violations::{RuleViolation, RuleViolationKind};
use crate::ViolationMessage;
use squawk_parser::ast::{AlterTableCmds, AlterTableType, ObjectType, RawStmt, Stmt};

#[must_use]
pub fn removing_existing_index(tree: &[RawStmt]) -> Vec<RuleViolation> {
  let mut errs = vec![];
  for raw_stmt in tree {
    match &raw_stmt.stmt {
      Stmt::DropStmt(stmt) if stmt.remove_type == ObjectType::Index => {
        errs.push(RuleViolation::new(
          RuleViolationKind::RemovingExistingIndex,
          raw_stmt.into(),
          Some(vec![
            ViolationMessage::Note(
              "Removing an existing index can lead to downtime if the index is heavily used."
                .into(),
            ),
            ViolationMessage::Help(
              "Ensure that query patterns do not rely on this index before removing.".into(),
            ),
          ]),
        ))
      }
      Stmt::AlterTableStmt(stmt) => {
        for AlterTableCmds::AlterTableCmd(cmd) in &stmt.cmds {
          if cmd.subtype == AlterTableType::DropConstraint {
            if String::from(cmd.name.as_ref().unwrap()).contains("pkey") {
              errs.push(RuleViolation::new(
                RuleViolationKind::RemovingExistingIndex,
                raw_stmt.into(),
                Some(vec![
                  ViolationMessage::Note(
                    "Removing an existing index can lead to downtime if the index is heavily used."
                      .into(),
                  ),
                  ViolationMessage::Help(
                    "Ensure that query patterns do not rely on this index before removing.".into(),
                  ),
                ]),
              ))
            }
          }
        }
      }
      _ => continue,
    }
  }
  errs
}

#[cfg(test)]
mod test_rules {
  use crate::{check_sql, violations::RuleViolationKind, ViolationMessage};

  #[test]
  fn drop_index() {
    let sql = r#"
BEGIN;
DROP INDEX CONCURRENTLY IF EXISTS core_recipe_index;
COMMIT;
        "#;
    let res = check_sql(sql, &[]).unwrap();
    assert_eq!(res.len(), 1);
    assert_eq!(res[0].kind, RuleViolationKind::RemovingExistingIndex);
    assert_eq!(
      res[0].messages,
      vec![
        ViolationMessage::Note(
          "Removing an existing index can lead to downtime if the index is heavily used.".into(),
        ),
        ViolationMessage::Help(
          "Ensure that query patterns do not rely on this index before removing.".into(),
        ),
      ]
    )
  }

  #[test]
  fn remove_pk_constraint() {
    let sql = r#"
BEGIN;
ALTER TABLE core_recipe DROP CONSTRAINT IF EXISTS core_recipe_pkey;
COMMIT;
        "#;
    let res = check_sql(sql, &[]).unwrap();
    assert_eq!(res.len(), 1);
    assert_eq!(res[0].kind, RuleViolationKind::RemovingExistingIndex);
    assert_eq!(
      res[0].messages,
      vec![
        ViolationMessage::Note(
          "Removing an existing index can lead to downtime if the index is heavily used.".into(),
        ),
        ViolationMessage::Help(
          "Ensure that query patterns do not rely on this index before removing.".into(),
        ),
      ]
    )
  }

  #[test]
  fn remove_non_pk_constraint() {
    let sql = r#"
ALTER TABLE core_recipe DROP CONSTRAINT IF EXISTS core_recipe_not_null;
        "#;
    let res = check_sql(sql, &[]).unwrap();
    assert_eq!(res.len(), 0);
  }
}
