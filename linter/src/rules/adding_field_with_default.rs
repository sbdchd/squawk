use crate::violations::{RuleViolation, RuleViolationKind};
use squawk_parser::ast::{
    AlterTableCmds, AlterTableDef, ColumnDefConstraint, ConstrType, RootStmt, Stmt,
};

#[must_use]
pub fn adding_field_with_default(tree: &[RootStmt]) -> Vec<RuleViolation> {
    let mut errs = vec![];
    for RootStmt::RawStmt(raw_stmt) in tree {
        match &raw_stmt.stmt {
            Stmt::AlterTableStmt(stmt) => {
                for AlterTableCmds::AlterTableCmd(cmd) in &stmt.cmds {
                    match &cmd.def {
                        Some(AlterTableDef::ColumnDef(def)) => {
                            for ColumnDefConstraint::Constraint(constraint) in &def.constraints {
                                if constraint.contype == ConstrType::Default {
                                    errs.push(RuleViolation::new(
                                        RuleViolationKind::AddingFieldWithDefault,
                                        raw_stmt,
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
    use crate::check_sql;
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

        assert_debug_snapshot!(check_sql(bad_sql, &["prefer-robust-stmts".into()]));
        assert_debug_snapshot!(check_sql(ok_sql, &["prefer-robust-stmts".into()]));
    }
}
