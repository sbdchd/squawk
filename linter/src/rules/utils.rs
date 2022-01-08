use squawk_parser::ast::{RootStmt, Stmt, TransactionStmtKind};
use std::collections::HashSet;

pub fn tables_created_in_transaction(tree: &[RootStmt]) -> HashSet<String> {
    let mut created_table_names = HashSet::new();
    let mut inside_transaction = false;
    for RootStmt::RawStmt(raw_stmt) in tree {
        match &raw_stmt.stmt {
            Stmt::TransactionStmt(stmt) => match stmt.kind {
                TransactionStmtKind::Begin => inside_transaction = true,
                TransactionStmtKind::Commit => inside_transaction = false,
                _ => continue,
            },
            Stmt::CreateStmt(stmt) if inside_transaction => {
                let stmt = &stmt.relation;
                let table_name = &stmt.relname;
                created_table_names.insert(table_name.to_owned());
            }
            _ => continue,
        }
    }
    created_table_names
}
