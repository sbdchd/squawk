use crate::violations::{RuleViolation, RuleViolationKind, ViolationMessage};
use squawk_parser::ast::{
  AlterTableCmds, AlterTableDef, AlterTableType, ColumnDefConstraint, ConstrType, RawStmt, Stmt,
};

#[must_use]
pub fn adding_not_null_with_default(tree: &[RawStmt]) -> Vec<RuleViolation> {
  let mut errs = vec![];
  for raw_stmt in tree {
    match &raw_stmt.stmt {
      Stmt::AlterTableStmt(stmt) => {
        for AlterTableCmds::AlterTableCmd(cmd) in &stmt.cmds {
          if cmd.subtype == AlterTableType::AddColumn {
            if let Some(AlterTableDef::ColumnDef(column_def)) = &cmd.def {
              for ColumnDefConstraint::Constraint(constraint) in &column_def.constraints {
                if constraint.contype == ConstrType::NotNull {
                  errs.push(RuleViolation::new(
                    RuleViolationKind::AddingNotNullWithDefault,
                    raw_stmt.into(),
                    Some(vec![
                      ViolationMessage::Note(
                        "Adding a column with a not null default is only safe in PG versions > 11."
                          .into(),
                      ),
                      ViolationMessage::Help(
                        "For older versions first add the column, then change the default.".into(),
                      ),
                    ]),
                  ));
                }
              }
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
  use crate::check_sql;
  use insta::assert_debug_snapshot;

  #[test]
  fn do_not_allow_not_null_field_with_default() {
    let bad_sql = r#"
ALTER TABLE "foo_tbl" ADD COLUMN IF NOT EXISTS "bar_col" TEXT DEFAULT 'buzz' NOT NULL;
"#;
    assert_debug_snapshot!(check_sql(bad_sql, &["adding-field-with-default".into()]));
  }
}
