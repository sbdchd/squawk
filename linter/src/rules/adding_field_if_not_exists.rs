use crate::{
    versions::Version,
    violations::{RuleViolation, RuleViolationKind},
};

use squawk_parser::ast::{AlterTableCmds, AlterTableType, RawStmt, Stmt};

#[must_use]
pub fn adding_field_if_not_exists(
    tree: &[RawStmt],
    _pg_version: Option<Version>,
    _assume_in_transaction: bool,
) -> Vec<RuleViolation> {
    let mut errs = vec![];

    for raw_stmt in tree {
        match &raw_stmt.stmt {
            Stmt::AlterTableStmt(stmt) => {
                for AlterTableCmds::AlterTableCmd(cmd) in &stmt.cmds {
                    if cmd.subtype == AlterTableType::AddColumn && !cmd.missing_ok {
                        errs.push(RuleViolation::new(
                            RuleViolationKind::AddingFieldIfNotExists,
                            raw_stmt.into(),
                            None,
                        ));
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
        versions::Version,
        violations::{RuleViolation, RuleViolationKind},
    };

    use insta::assert_debug_snapshot;

    fn lint_sql(sql: &str, pg_version: Option<Version>) -> Vec<RuleViolation> {
        check_sql_with_rule(
            sql,
            &RuleViolationKind::AddingFieldIfNotExists,
            pg_version,
            false,
        )
        .unwrap()
    }

    #[test]
    fn test_example_bad() {
        let bad_sql = r#"
ALTER TABLE "core_recipe" ADD COLUMN "foo" integer DEFAULT 10;
"#;
        assert_debug_snapshot!(lint_sql(bad_sql, None));
    }

    #[test]
    fn test_example_ok() {
        let ok_sql = r#"
ALTER TABLE "core_recipe" ADD COLUMN IF NOT EXISTS "foo" integer;
        "#;
        assert_debug_snapshot!(lint_sql(ok_sql, None));
    }
}
