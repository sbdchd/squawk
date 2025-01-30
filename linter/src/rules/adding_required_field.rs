use crate::versions::Version;
use crate::violations::{RuleViolation, RuleViolationKind};

use squawk_parser::ast::{
    AlterTableCmds, AlterTableDef, AlterTableType, ColumnDefConstraint, ConstrType, RawStmt, Stmt,
};
fn has_generated_constraint(constraints: &[ColumnDefConstraint]) -> bool {
    for ColumnDefConstraint::Constraint(constraint) in constraints {
        if constraint.contype == ConstrType::Generated {
            return true;
        }
    }
    false
}

fn has_not_null_and_no_default_constraint(constraints: &[ColumnDefConstraint]) -> bool {
    let mut has_not_null = false;
    let mut has_default = false;
    for ColumnDefConstraint::Constraint(constraint) in constraints {
        if constraint.contype == ConstrType::NotNull {
            has_not_null = true;
        }
        if constraint.contype == ConstrType::Default {
            has_default = true;
        }
    }
    has_not_null && !has_default
}

#[must_use]
pub fn adding_required_field(
    tree: &[RawStmt],
    _pg_version: Option<Version>,
    _assume_in_transaction: bool,
) -> Vec<RuleViolation> {
    let mut errs = vec![];

    for raw_stmt in tree {
        match &raw_stmt.stmt {
            Stmt::AlterTableStmt(stmt) => {
                for AlterTableCmds::AlterTableCmd(cmd) in &stmt.cmds {
                    if cmd.subtype == AlterTableType::AddColumn {
                        if let Some(AlterTableDef::ColumnDef(column_def)) = &cmd.def {
                            if has_generated_constraint(&column_def.constraints) {
                                continue;
                            }
                            if has_not_null_and_no_default_constraint(&column_def.constraints) {
                                errs.push(RuleViolation::new(
                                    RuleViolationKind::AddingRequiredField,
                                    raw_stmt.into(),
                                    None,
                                ));
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
    use crate::{
        check_sql_with_rule,
        violations::{RuleViolation, RuleViolationKind},
    };
    use insta::assert_debug_snapshot;

    fn lint_sql(sql: &str) -> Vec<RuleViolation> {
        check_sql_with_rule(sql, &RuleViolationKind::AddingRequiredField, None, false).unwrap()
    }

    #[test]
    fn nullable() {
        let ok_sql = r#"
ALTER TABLE "recipe" ADD COLUMN "public" boolean;
  "#;
        assert_debug_snapshot!(lint_sql(ok_sql));
    }

    #[test]
    fn not_null_with_default() {
        let ok_sql = r#"
ALTER TABLE "recipe" ADD COLUMN "public" boolean NOT NULL DEFAULT true;
  "#;
        assert_debug_snapshot!(lint_sql(ok_sql));
    }

    #[test]
    fn not_null_without_default() {
        let bad_sql = r#"
ALTER TABLE "recipe" ADD COLUMN "public" boolean NOT NULL;
  "#;
        assert_debug_snapshot!(lint_sql(bad_sql));
    }
    #[test]
    fn generated_stored_not_null() {
        let ok_sql = r"
 ALTER TABLE foo
    ADD COLUMN bar numeric GENERATED ALWAYS AS (bar + baz) STORED NOT NULL;
        ";
        assert_debug_snapshot!(lint_sql(ok_sql));
    }
    #[test]
    fn generated_stored() {
        let ok_sql = r"
 ALTER TABLE foo
    ADD COLUMN bar numeric GENERATED ALWAYS AS (bar + baz) STORED ;
        ";
        assert_debug_snapshot!(lint_sql(ok_sql));
    }
}
