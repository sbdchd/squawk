use crate::violations::{RuleViolation, RuleViolationKind};
use squawk_parser::ast::{ObjectType, RootStmt, Stmt};

pub fn renaming_column(tree: &[RootStmt]) -> Vec<RuleViolation> {
    let mut errs = vec![];
    for RootStmt::RawStmt(raw_stmt) in tree {
        match &raw_stmt.stmt {
            Stmt::RenameStmt(stmt) => match stmt.rename_type {
                ObjectType::Column => {
                    errs.push(RuleViolation::new(
                        RuleViolationKind::RenamingColumn,
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
