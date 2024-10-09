use squawk_parser::ast::{
    AlterTableCmds, AlterTableDef, AlterTableType, ColumnDef, RawStmt, Stmt, TableElt,
    TransactionStmtKind,
};
use std::collections::HashSet;

pub fn tables_created_in_transaction(
    tree: &[RawStmt],
    assume_in_transaction: bool,
) -> HashSet<String> {
    let mut created_table_names = HashSet::new();
    let mut inside_transaction = assume_in_transaction;
    for raw_stmt in tree {
        match &raw_stmt.stmt {
            Stmt::TransactionStmt(stmt) => match stmt.kind {
                TransactionStmtKind::Begin | TransactionStmtKind::Start => {
                    inside_transaction = true;
                }
                TransactionStmtKind::Commit => inside_transaction = false,
                _ => continue,
            },
            Stmt::CreateStmt(stmt) if inside_transaction => {
                let stmt = &stmt.relation;
                let table_name = &stmt.relname;
                created_table_names.insert(table_name.clone());
            }
            _ => continue,
        }
    }
    created_table_names
}

pub fn columns_create_or_modified(stmt: &Stmt) -> Vec<&ColumnDef> {
    let mut columns = vec![];
    match stmt {
        Stmt::CreateStmt(stmt) => {
            for column_def in &stmt.table_elts {
                if let TableElt::ColumnDef(column_def) = column_def {
                    columns.push(column_def);
                }
            }
        }
        Stmt::AlterTableStmt(stmt) => {
            for AlterTableCmds::AlterTableCmd(cmd) in &stmt.cmds {
                if cmd.subtype == AlterTableType::AddColumn
                    || cmd.subtype == AlterTableType::AlterColumnType
                {
                    if let Some(AlterTableDef::ColumnDef(column_def)) = &cmd.def {
                        columns.push(column_def);
                    }
                }
            }
        }
        _ => {}
    }
    columns
}
