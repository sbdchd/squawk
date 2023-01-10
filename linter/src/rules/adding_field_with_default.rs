use crate::{
    versions::Version,
    violations::{RuleViolation, RuleViolationKind},
};

use serde_json::{json, Value};
use squawk_parser::ast::{
    AlterTableCmds, AlterTableDef, ColumnDefConstraint, ConstrType, Constraint, RawStmt, Stmt,
};

fn constraint_has_constant_expr(constraint: &Constraint) -> bool {
    constraint.raw_expr.is_some()
        && constraint.raw_expr.as_ref().unwrap_or(&json!({}))["A_Const"] != Value::Null
}

#[must_use]
pub fn adding_field_with_default(
    tree: &[RawStmt],
    pg_version: Option<Version>,
    _assume_in_transaction: bool,
) -> Vec<RuleViolation> {
    let mut errs = vec![];
    for raw_stmt in tree {
        match &raw_stmt.stmt {
            Stmt::AlterTableStmt(stmt) => {
                for AlterTableCmds::AlterTableCmd(cmd) in &stmt.cmds {
                    match &cmd.def {
                        Some(AlterTableDef::ColumnDef(def)) => {
                            for ColumnDefConstraint::Constraint(constraint) in &def.constraints {
                                if constraint.contype == ConstrType::Default {
                                    if let Some(pg_version) = pg_version {
                                        if pg_version > Version::new(11, None, None)
                                            && constraint_has_constant_expr(constraint)
                                        {
                                            continue;
                                        }
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
    use std::str::FromStr;

    use crate::{
        check_sql_with_rule,
        versions::Version,
        violations::{RuleViolation, RuleViolationKind},
    };

    use insta::assert_debug_snapshot;

    fn lint_sql(sql: &str, pg_version: Option<Version>) -> Vec<RuleViolation> {
        check_sql_with_rule(
            sql,
            &RuleViolationKind::AddingFieldWithDefault,
            pg_version,
            false,
        )
        .unwrap()
    }

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

        assert_debug_snapshot!(lint_sql(bad_sql, None));
        assert_debug_snapshot!(lint_sql(ok_sql, None));
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

        let pg_version_11 = Some(Version::from_str("11.0.0").unwrap());
        assert_debug_snapshot!(lint_sql(bad_sql, pg_version_11));
        assert_debug_snapshot!(lint_sql(ok_sql, pg_version_11));
    }
}
