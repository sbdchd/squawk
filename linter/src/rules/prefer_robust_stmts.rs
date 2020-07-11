use crate::violations::{RuleViolation, RuleViolationKind};
use squawk_parser::ast::{AlterTableCmds, RootStmt, Stmt, TransactionStmtKind};

/// If a migration is running in a transaction, then we skip the statements
/// because if it fails part way through, it will revert.
/// For the cases where statements aren't running in a transaction, for instance,
/// when we CREATE INDEX CONCURRENTLY, we should try and make those migrations
/// more robust by using guards like `IF NOT EXISTS`. So if the migration fails
/// halfway through, it can be rerun without human intervention.
pub fn prefer_robust_stmts(tree: &[RootStmt]) -> Vec<RuleViolation> {
    let mut errs = vec![];
    let mut inside_transaction = false;
    for RootStmt::RawStmt(raw_stmt) in tree {
        match &raw_stmt.stmt {
            Stmt::TransactionStmt(stmt) => match stmt.kind {
                TransactionStmtKind::Begin => inside_transaction = true,
                TransactionStmtKind::Commit => inside_transaction = false,
                _ => continue,
            },
            Stmt::AlterTableStmt(stmt) => {
                for AlterTableCmds::AlterTableCmd(cmd) in &stmt.cmds {
                    if cmd.missing_ok || inside_transaction {
                        continue;
                    }
                    errs.push(RuleViolation::new(
                        RuleViolationKind::PreferRobustStmts,
                        raw_stmt,
                        None,
                    ));
                }
            }
            Stmt::IndexStmt(stmt) if !stmt.if_not_exists && !inside_transaction => {
                errs.push(RuleViolation::new(
                    RuleViolationKind::PreferRobustStmts,
                    raw_stmt,
                    None,
                ));
            }
            Stmt::CreateStmt(stmt) if !stmt.if_not_exists && !inside_transaction => {
                errs.push(RuleViolation::new(
                    RuleViolationKind::PreferRobustStmts,
                    raw_stmt,
                    None,
                ));
            }
            _ => continue,
        }
    }
    errs
}
