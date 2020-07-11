use crate::violations::{RuleViolation, RuleViolationKind};
use squawk_parser::ast::{
    AlterTableCmds, AlterTableDef, AlterTableType, ColumnDefConstraint, ConstrType, RootStmt, Stmt,
};

pub fn adding_not_nullable_field(tree: &[RootStmt]) -> Vec<RuleViolation> {
    let mut errs = vec![];
    for RootStmt::RawStmt(raw_stmt) in tree {
        match &raw_stmt.stmt {
            Stmt::AlterTableStmt(stmt) => {
                for AlterTableCmds::AlterTableCmd(cmd) in &stmt.cmds {
                    if cmd.subtype == AlterTableType::AddColumn {
                        if let Some(AlterTableDef::ColumnDef(column_def)) = &cmd.def {
                            for ColumnDefConstraint::Constraint(constraint) in
                                &column_def.constraints
                            {
                                if constraint.contype == ConstrType::NotNull {
                                    errs.push(RuleViolation::new(
                                        RuleViolationKind::AddingNotNullableField,
                                        raw_stmt,
                                        None,
                                    ));
                                }
                            }
                        }
                    }
                }
            }
            _ => continue,
        }
    }
    errs
}
