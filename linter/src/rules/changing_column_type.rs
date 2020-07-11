use crate::violations::{RuleViolation, RuleViolationKind};
use squawk_parser::ast::{AlterTableCmds, AlterTableType, RootStmt, Stmt};

pub fn changing_column_type(tree: &[RootStmt]) -> Vec<RuleViolation> {
    let mut errs = vec![];
    for RootStmt::RawStmt(raw_stmt) in tree {
        match &raw_stmt.stmt {
            Stmt::AlterTableStmt(stmt) => {
                for AlterTableCmds::AlterTableCmd(cmd) in &stmt.cmds {
                    if cmd.subtype == AlterTableType::AlterColumnType {
                        errs.push(RuleViolation::new(
                            RuleViolationKind::ChangingColumnType,
                            raw_stmt,
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
    use crate::check_sql;
    use insta::assert_debug_snapshot;
    #[test]
    fn test_changing_field_type() {
        let bad_sql = r#"
BEGIN;
--
-- Alter field edits on recipe
--
ALTER TABLE "core_recipe" ALTER COLUMN "edits" TYPE text USING "edits"::text;
COMMIT;
        "#;
        assert_debug_snapshot!(check_sql(bad_sql, &[]));

        let bad_sql = r#"
BEGIN;
--
-- Alter field foo on recipe
--
ALTER TABLE "core_recipe" ALTER COLUMN "foo" TYPE varchar(255) USING "foo"::varchar(255);
ALTER TABLE "core_recipe" ALTER COLUMN "foo" TYPE text USING "foo"::text;
COMMIT;
        "#;

        assert_debug_snapshot!(check_sql(bad_sql, &[]));
    }
}
