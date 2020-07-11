use crate::rules::utils::tables_created_in_transaction;
use crate::violations::{RuleViolation, RuleViolationKind};
use squawk_parser::ast::{AlterTableCmds, AlterTableDef, RelationKind, RootStmt, Stmt};

pub fn constraint_missing_not_valid(tree: &[RootStmt]) -> Vec<RuleViolation> {
    let mut errs = vec![];
    let tables_created = tables_created_in_transaction(tree);
    for RootStmt::RawStmt(raw_stmt) in tree {
        match &raw_stmt.stmt {
            Stmt::AlterTableStmt(stmt) => {
                let RelationKind::RangeVar(range) = &stmt.relation;
                let tbl_name = &range.relname;
                for AlterTableCmds::AlterTableCmd(cmd) in &stmt.cmds {
                    match &cmd.def {
                        Some(AlterTableDef::Constraint(constraint)) => {
                            if !tables_created.contains(tbl_name) && constraint.initially_valid {
                                errs.push(RuleViolation::new(
                                    RuleViolationKind::ConstraintMissingNotValid,
                                    raw_stmt,
                                    None,
                                ));
                            }
                        }
                        _ => continue,
                    }
                }
            }
            _ => continue,
        }
    }
    errs
}
