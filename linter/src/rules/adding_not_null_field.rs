use crate::versions::Version;
use crate::violations::{RuleViolation, RuleViolationKind};
use crate::ViolationMessage;

use squawk_parser::ast::{AlterTableCmds, AlterTableType, RawStmt, Stmt};

#[must_use]
pub fn adding_not_nullable_field(
    tree: &[RawStmt],
    _pg_version: Option<Version>,
    _assume_in_transaction: bool,
) -> Vec<RuleViolation> {
    let mut errs = vec![];

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

    fn lint_sql(sql: &str) -> Vec<RuleViolation> {
        check_sql_with_rule(sql, &RuleViolationKind::AddingNotNullableField, None, false).unwrap()
    }

    use insta::assert_debug_snapshot;

    #[test]
    fn set_not_null() {
        let bad_sql = r#"
ALTER TABLE "core_recipe" ALTER COLUMN "foo" SET NOT NULL;
        "#;
        assert_debug_snapshot!(lint_sql(bad_sql));
    }

    #[test]
    fn adding_field_that_is_not_nullable() {
        let ok_sql = r#"
BEGIN;
-- This will cause a table rewrite for Postgres versions before 11, but that is handled by
-- adding-field-with-default.
ALTER TABLE "core_recipe" ADD COLUMN "foo" integer DEFAULT 10 NOT NULL;
ALTER TABLE "core_recipe" ALTER COLUMN "foo" DROP DEFAULT;
COMMIT;
        "#;
        assert_debug_snapshot!(lint_sql(ok_sql));
    }

    #[test]
    fn adding_field_that_is_not_nullable_without_default() {
        let ok_sql = r#"
-- This won't work if the table is populated, but that error is caught by adding-required-field.
ALTER TABLE "core_recipe" ADD COLUMN "foo" integer NOT NULL;
        "#;
        assert_debug_snapshot!(lint_sql(ok_sql));
    }
}
