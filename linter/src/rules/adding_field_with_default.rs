use crate::violations::{RuleViolation, RuleViolationKind};
use squawk_parser::ast::{
    AlterTableCmds, AlterTableDef, ColumnDefConstraint, ConstrType, RootStmt, Stmt,
};

pub fn adding_field_with_default(tree: &[RootStmt]) -> Vec<RuleViolation> {
    let mut errs = vec![];
    for RootStmt::RawStmt(raw_stmt) in tree {
        match &raw_stmt.stmt {
            Stmt::AlterTableStmt(stmt) => {
                for AlterTableCmds::AlterTableCmd(cmd) in &stmt.cmds {
                    match &cmd.def {
                        Some(AlterTableDef::ColumnDef(def)) => {
                            for ColumnDefConstraint::Constraint(constraint) in &def.constraints {
                                if constraint.contype == ConstrType::Default {
                                    errs.push(RuleViolation::new(
                                        RuleViolationKind::AddingFieldWithDefault,
                                        raw_stmt,
                                        None,
                                    ));
                                }
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
