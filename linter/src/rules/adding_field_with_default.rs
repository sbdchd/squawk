use crate::violations::{ok_non_null_pg_version_req, RuleViolation, RuleViolationKind};
use ::semver::Version;
use serde_json::{json, Value};
use squawk_parser::ast::{
    AlterTableCmds, AlterTableDef, ColumnDefConstraint, ConstrType, RawStmt, Stmt,
};

#[must_use]
pub fn adding_field_with_default(tree: &[RawStmt], pg_version: &Version) -> Vec<RuleViolation> {
    let mut errs = vec![];
    for raw_stmt in tree {
        match &raw_stmt.stmt {
            Stmt::AlterTableStmt(stmt) => {
                for AlterTableCmds::AlterTableCmd(cmd) in &stmt.cmds {
                    match &cmd.def {
                        Some(AlterTableDef::ColumnDef(def)) => {
                            for ColumnDefConstraint::Constraint(constraint) in &def.constraints {
                                if constraint.contype == ConstrType::Default {
                                    if ok_non_null_pg_version_req().matches(pg_version)
                                        && constraint.raw_expr.is_some()
                                        && constraint.raw_expr.as_ref().unwrap_or(&json!({}))
                                            ["A_Const"]
                                            != Value::Null
                                    {
                                        continue;
                                    }
                                    errs.push(RuleViolation::new(
                                        RuleViolationKind::AddingFieldWithDefault,
                                        raw_stmt.into(),
                                        None,
                                    ));
                                }
                            }
                        }
                        _ => continue,
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
        check_sql,
        violations::{default_pg_version, RuleViolationKind},
    };
    use ::semver::Version;
    use insta::assert_debug_snapshot;

    ///
    /// ```sql
    /// -- instead of
    /// ALTER TABLE "core_recipe" ADD COLUMN "foo" integer DEFAULT 10;
    /// -- use
    /// ALTER TABLE "core_recipe" ADD COLUMN "foo" integer;
    /// ALTER TABLE "core_recipe" ALTER COLUMN "foo" SET DEFAULT 10;
    /// -- backfill
    /// -- remove nullability
    /// ```
    #[test]
    fn test_adding_field_with_default() {
        let bad_sql = r#"
-- instead of
ALTER TABLE "core_recipe" ADD COLUMN "foo" integer DEFAULT 10;
"#;

        let ok_sql = r#"
-- use
ALTER TABLE "core_recipe" ADD COLUMN "foo" integer;
ALTER TABLE "core_recipe" ALTER COLUMN "foo" SET DEFAULT 10;
-- backfill
-- remove nullability
        "#;

        assert_debug_snapshot!(check_sql(
            bad_sql,
            &[RuleViolationKind::PreferRobustStmts],
            &default_pg_version()
        ));
        assert_debug_snapshot!(check_sql(
            ok_sql,
            &[RuleViolationKind::PreferRobustStmts],
            &default_pg_version()
        ));
    }

    #[test]

    fn test_adding_field_with_default_in_version_11() {
        let bad_sql = r#"
-- VOLATILE
ALTER TABLE "core_recipe" ADD COLUMN "foo" integer DEFAULT uuid();
"#;
        let ok_sql = r#"
-- NON-VOLATILE
ALTER TABLE "core_recipe" ADD COLUMN "foo" integer DEFAULT 10;
"#;

        assert_debug_snapshot!(check_sql(
            bad_sql,
            &[RuleViolationKind::PreferRobustStmts],
            &Version::parse("11.0.0").unwrap(),
        ));
        assert_debug_snapshot!(check_sql(
            ok_sql,
            &[RuleViolationKind::PreferRobustStmts],
            &Version::parse("11.0.0").unwrap(),
        ));
    }
}
