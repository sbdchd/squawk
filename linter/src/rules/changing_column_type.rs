use crate::{
    versions::Version,
    violations::{RuleViolation, RuleViolationKind},
};

use squawk_parser::ast::{AlterTableCmds, AlterTableType, RawStmt, Stmt};

#[must_use]
pub fn changing_column_type(
    tree: &[RawStmt],
    _pg_version: Option<Version>,
    _assume_in_transaction: bool,
) -> Vec<RuleViolation> {
    let mut errs = vec![];
    for raw_stmt in tree {
        match &raw_stmt.stmt {
            Stmt::AlterTableStmt(stmt) => {
                for AlterTableCmds::AlterTableCmd(cmd) in &stmt.cmds {
                    if cmd.subtype == AlterTableType::AlterColumnType {
                        errs.push(RuleViolation::new(
                            RuleViolationKind::ChangingColumnType,
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
        violations::{RuleViolation, RuleViolationKind},
    };
    use insta::assert_debug_snapshot;

    fn lint_sql(sql: &str) -> Vec<RuleViolation> {
        check_sql_with_rule(sql, &RuleViolationKind::ChangingColumnType, None, false).unwrap()
    }

    #[test]
    fn changing_field_type() {
        let bad_sql = r#"
BEGIN;
--
-- Alter field edits on recipe
--
ALTER TABLE "core_recipe" ALTER COLUMN "edits" TYPE text USING "edits"::text;
COMMIT;
        "#;
        assert_debug_snapshot!(lint_sql(bad_sql));

        let bad_sql = r#"
BEGIN;
--
-- Alter field foo on recipe
--
ALTER TABLE "core_recipe" ALTER COLUMN "foo" TYPE varchar(255) USING "foo"::varchar(255);
ALTER TABLE "core_recipe" ALTER COLUMN "foo" TYPE text USING "foo"::text;
COMMIT;
        "#;

        assert_debug_snapshot!(lint_sql(bad_sql));
    }
}
