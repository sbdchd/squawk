use crate::violations::{RuleViolation, RuleViolationKind};
use squawk_parser::ast::{ObjectType, RootStmt, Stmt};

pub fn renaming_table(tree: &[RootStmt]) -> Vec<RuleViolation> {
    let mut errs = vec![];
    for RootStmt::RawStmt(raw_stmt) in tree {
        match &raw_stmt.stmt {
            Stmt::RenameStmt(stmt) => match stmt.rename_type {
                ObjectType::Table => {
                    errs.push(RuleViolation::new(
                        RuleViolationKind::RenamingTable,
                        raw_stmt,
                        None,
                    ));
                }
                _ => continue,
            },
            _ => continue,
        }
    }
    errs
}
