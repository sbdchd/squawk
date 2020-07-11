use crate::rules::utils::tables_created_in_transaction;
use crate::violations::{RuleViolation, RuleViolationKind};
use squawk_parser::ast::{RelationKind, RootStmt, Stmt};

pub fn require_concurrent_index_creation(tree: &[RootStmt]) -> Vec<RuleViolation> {
    let tables_created = tables_created_in_transaction(tree);
    let mut errs = vec![];
    for RootStmt::RawStmt(raw_stmt) in tree {
        match &raw_stmt.stmt {
            Stmt::IndexStmt(stmt) => {
                let RelationKind::RangeVar(range) = &stmt.relation;
                let tbl_name = &range.relname;
                if !stmt.concurrent && !tables_created.contains(tbl_name) {
                    errs.push(RuleViolation::new(
                        RuleViolationKind::RequireConcurrentIndexCreation,
                        raw_stmt,
                        None,
                    ));
                }
            }
            _ => continue,
        }
    }
    errs
}
