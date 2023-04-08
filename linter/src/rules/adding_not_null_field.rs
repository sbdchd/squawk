use crate::versions::Version;
use crate::violations::{RuleViolation, RuleViolationKind};
use crate::ViolationMessage;

use squawk_parser::ast::{
    AlterTableCmds, AlterTableDef, AlterTableType, ColumnDefConstraint, ConstrType, RawStmt, Stmt,
};

fn has_not_null_and_default_constraint(constraints: &[ColumnDefConstraint]) -> bool {
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
    has_not_null && has_default
}

#[must_use]
pub fn adding_not_nullable_field(
    tree: &[RawStmt],
    pg_version: Option<Version>,
    _assume_in_transaction: bool,
) -> Vec<RuleViolation> {
    let mut errs = vec![];
    if let Some(pg_version) = pg_version {
        let pg_11 = Version::new(11, Some(0), Some(0));
        if pg_version >= pg_11 {
            return errs;
        }
    }

    for raw_stmt in tree {
        match &raw_stmt.stmt {
            Stmt::AlterTableStmt(stmt) => {
                for AlterTableCmds::AlterTableCmd(cmd) in &stmt.cmds {
                    if cmd.subtype == AlterTableType::SetNotNull {
                        errs.push(RuleViolation::new(
                            RuleViolationKind::AddingNotNullableField,
                            raw_stmt.into(),
                            Some(vec![
                                ViolationMessage::Note("Setting a column NOT NULL blocks reads while the table is scanned.".into()),
                                ViolationMessage::Help("Use a check constraint instead.".into())
                            ]),
                        ));
                    }
                    if cmd.subtype == AlterTableType::AddColumn {
                        if let Some(AlterTableDef::ColumnDef(column_def)) = &cmd.def {
                            if has_not_null_and_default_constraint(&column_def.constraints) {
                                errs.push(RuleViolation::new(
                                    RuleViolationKind::AddingNotNullableField,
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
    use std::str::FromStr;

    use crate::{
        check_sql_with_rule,
        versions::Version,
        violations::{RuleViolation, RuleViolationKind},
    };

    fn lint_sql(sql: &str, pg_version: Option<Version>) -> Vec<RuleViolation> {
        check_sql_with_rule(
            sql,
            &RuleViolationKind::AddingNotNullableField,
            pg_version,
            false,
        )
        .unwrap()
    }

    use insta::assert_debug_snapshot;

    #[test]
    fn test_set_not_null() {
        let sql = r#"
ALTER TABLE "core_recipe" ALTER COLUMN "foo" SET NOT NULL;
        "#;
        assert_debug_snapshot!(lint_sql(sql, None));
    }

    #[test]
    fn test_adding_field_that_is_not_nullable() {
        let bad_sql = r#"
BEGIN;
--
-- Add field foo to recipe
--
ALTER TABLE "core_recipe" ADD COLUMN "foo" integer DEFAULT 10 NOT NULL;
ALTER TABLE "core_recipe" ALTER COLUMN "foo" DROP DEFAULT;
COMMIT;
        "#;
        assert_debug_snapshot!(lint_sql(bad_sql, None));
    }

    #[test]
    fn test_adding_field_that_is_not_nullable_without_default() {
        let ok_sql = r#"
-- This won't work if the table is populated, but that error is caught by adding_required_field.
ALTER TABLE "core_recipe" ADD COLUMN "foo" integer NOT NULL;
        "#;
        assert_debug_snapshot!(lint_sql(ok_sql, None));
    }

    #[test]
    fn test_adding_field_that_is_not_nullable_in_version_11() {
        let ok_sql = r#"
BEGIN;
--
-- Add field foo to recipe
--
ALTER TABLE "core_recipe" ADD COLUMN "foo" integer NOT NULL DEFAULT 10;
COMMIT;
        "#;
        assert_debug_snapshot!(lint_sql(ok_sql, Some(Version::from_str("11.0.0").unwrap()),));
    }
}
