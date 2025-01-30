use crate::{versions::Version, violations::RuleViolation, violations::RuleViolationKind};

use squawk_parser::ast::{AlterTableCmds, AlterTableType, RawStmt, Stmt};

#[must_use]
pub fn ban_drop_not_null(
    tree: &[RawStmt],
    _pg_version: Option<Version>,
    _assume_in_transaction: bool,
) -> Vec<RuleViolation> {
    let mut errs = vec![];
    for raw_stmt in tree {
        match &raw_stmt.stmt {
            Stmt::AlterTableStmt(stmt) => {
                for cmd in &stmt.cmds {
                    let AlterTableCmds::AlterTableCmd(cmd) = cmd;
                    if cmd.subtype == AlterTableType::DropNotNull {
                        errs.push(RuleViolation::new(
                            RuleViolationKind::BanDropNotNull,
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
        check_sql_with_rule(sql, &RuleViolationKind::BanDropNotNull, None, false).unwrap()
    }

    #[test]
    fn ban_drop_not_null() {
        let bad_sql = r#"
ALTER TABLE "bar_tbl" ALTER COLUMN "foo_col" DROP NOT NULL;
  "#;
        assert_debug_snapshot!(lint_sql(bad_sql));
    }
}
