use crate::rules::utils::tables_created_in_transaction;
use crate::violations::{RuleViolation, RuleViolationKind};
use squawk_parser::ast::{AlterTableCmds, AlterTableDef, ConstrType, RelationKind, RootStmt, Stmt};

pub fn disallow_unique_constraint(tree: &[RootStmt]) -> Vec<RuleViolation> {
    let tables_created = tables_created_in_transaction(tree);
    let mut errs = vec![];
    for RootStmt::RawStmt(raw_stmt) in tree {
        match &raw_stmt.stmt {
            Stmt::AlterTableStmt(stmt) => {
                let RelationKind::RangeVar(range) = &stmt.relation;
                let tbl_name = &range.relname;
                for AlterTableCmds::AlterTableCmd(cmd) in &stmt.cmds {
                    match &cmd.def {
                        Some(AlterTableDef::Constraint(constraint)) => {
                            if !tables_created.contains(tbl_name)
                                && constraint.contype == ConstrType::Unique
                                && constraint.indexname.is_none()
                            {
                                errs.push(RuleViolation::new(
                                    RuleViolationKind::DisallowedUniqueConstraint,
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
