use crate::violations::{RuleViolation, RuleViolationKind};
use squawk_parser::ast::{ColumnDefTypeName, QualifiedName, RootStmt, Stmt, TableElt};

/// It's easier to update the check constraint on a text field than a varchar()
/// size since the check constraint can use NOT VALID with a separate VALIDATE
/// call.
pub fn prefer_text_field(tree: &[RootStmt]) -> Vec<RuleViolation> {
    let mut errs = vec![];
    for RootStmt::RawStmt(raw_stmt) in tree {
        match &raw_stmt.stmt {
            Stmt::CreateStmt(stmt) => {
                for TableElt::ColumnDef(column_def) in &stmt.table_elts {
                    let ColumnDefTypeName::TypeName(type_name) = &column_def.type_name;
                    for QualifiedName::String(field_type_name) in &type_name.names {
                        if field_type_name.str == "varchar" {
                            errs.push(RuleViolation::new(
                                RuleViolationKind::PreferTextField,
                                raw_stmt,
                                None,
                            ));
                        }
                    }
                }
            }
            _ => continue,
        }
    }
    errs
}
