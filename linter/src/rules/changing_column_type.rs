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
