use crate::versions::Version;
use crate::violations::{RuleViolation, RuleViolationKind};
use crate::ViolationMessage;

use squawk_parser::ast::{
    AlterTableCmds, AlterTableDef, AlterTableType, ColumnDefConstraint, ConstrType, RawStmt, Stmt,
};

fn has_null_and_no_default_constraint(constraints: &[ColumnDefConstraint]) -> bool {
    let mut has_null = false;
    let mut has_default = false;
    for ColumnDefConstraint::Constraint(constraint) in constraints {
        if constraint.contype == ConstrType::NotNull {
            has_null = true;
        }
        if constraint.contype == ConstrType::Default {
            has_default = true;
        }
    }
    has_null && !has_default
}

#[must_use]
pub fn adding_not_nullable_field(
    tree: &[RawStmt],
    pg_version: Option<Version>,
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
                            if has_null_and_no_default_constraint(&column_def.constraints) {
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

    use crate::{check_sql, versions::Version, violations::RuleViolationKind, ViolationMessage};

    use insta::assert_debug_snapshot;

    #[test]
    fn set_null() {
        let sql = r#"
ALTER TABLE "core_recipe" ALTER COLUMN "foo" SET NOT NULL;
        "#;
        let res = check_sql(sql, &[RuleViolationKind::PreferRobustStmts], None).unwrap();
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].kind, RuleViolationKind::AddingNotNullableField);
        assert_eq!(
            res[0].messages,
            vec![
                ViolationMessage::Note(
                    "Setting a column NOT NULL blocks reads while the table is scanned.".into()
                ),
                ViolationMessage::Help("Use a check constraint instead.".into())
            ]
        );
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

        assert_debug_snapshot!(check_sql(
            bad_sql,
            &[RuleViolationKind::PreferRobustStmts],
            None
        ));

        let bad_sql = r#"
-- not sure how this would ever work, but might as well test it
ALTER TABLE "core_recipe" ADD COLUMN "foo" integer NOT NULL;
        "#;

        assert_debug_snapshot!(check_sql(
            bad_sql,
            &[RuleViolationKind::PreferRobustStmts],
            None
        ));
    }

    #[test]
    fn test_adding_field_that_is_not_nullable_in_version_11() {
        let ok_sql = r#"
BEGIN;
--
-- Add field foo to recipe
--
ALTER TABLE "core_recipe" ADD COLUMN "foo" integer NOT NULL;
COMMIT;
        "#;

        assert_debug_snapshot!(check_sql(
            ok_sql,
            &[RuleViolationKind::PreferRobustStmts],
            Some(Version::from_str("11.0.0").unwrap()),
        ));
    }

    #[test]
    fn allow_not_null_field_with_default() {
        let ok_sql = r#"
ALTER TABLE "foo_tbl" ADD COLUMN IF NOT EXISTS "bar_col" TEXT DEFAULT 'buzz' NOT NULL;
"#;
        assert_eq!(
            check_sql(ok_sql, &[RuleViolationKind::AddingFieldWithDefault], None),
            Ok(vec![])
        );
    }
}
