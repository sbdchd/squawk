use rustc_hash::FxHashSet;
use squawk_syntax::ast;

use crate::name::Name;

pub(crate) struct TablesCreatedInTransaction {
    names: FxHashSet<Box<[Name]>>,
    inside_transaction: bool,
}

fn table_name_ref(path: ast::PathRef) -> Box<[Name]> {
    std::iter::successors(Some(path), |path| path.qualifier())
        .filter_map(|path| path.segment())
        .map(|segment| Name::from_node(&segment))
        .collect()
}

impl TablesCreatedInTransaction {
    pub(crate) fn new(assume_in_transaction: bool) -> Self {
        Self {
            names: FxHashSet::default(),
            inside_transaction: assume_in_transaction,
        }
    }

    pub(crate) fn update(&mut self, stmt: &ast::Stmt) {
        let table = match stmt {
            ast::Stmt::Begin(_) => {
                self.inside_transaction = true;
                None
            }
            ast::Stmt::Commit(ast::Commit::CommitTransaction(commit)) => {
                self.names.clear();
                self.inside_transaction =
                    matches!(commit.chain_clause(), Some(ast::ChainClause::AndChain(_)));
                None
            }
            ast::Stmt::Rollback(ast::Rollback::RollbackTransaction(rollback)) => {
                self.names.clear();
                self.inside_transaction =
                    matches!(rollback.chain_clause(), Some(ast::ChainClause::AndChain(_)));
                None
            }
            ast::Stmt::CreateTable(table) if self.inside_transaction => table.table_name(),
            ast::Stmt::CreateTableAs(table) if self.inside_transaction => table.table_name(),
            ast::Stmt::SelectInto(select) if self.inside_transaction => {
                select.into_clause().and_then(|into| into.table_name())
            }
            _ => None,
        };
        if let Some(path) = table.and_then(|table| table.path())
            && let Some(segment) = path.segment()
        {
            let name = std::iter::once(Name::from_node(&segment))
                .chain(
                    std::iter::successors(path.qualifier(), |path| path.qualifier())
                        .filter_map(|path| path.segment())
                        .map(|segment| Name::from_node(&segment)),
                )
                .collect();
            self.names.insert(name);
        }
    }

    pub(crate) fn contains(&self, path: &ast::PathRef) -> bool {
        !self.names.is_empty() && self.names.contains(&table_name_ref(path.clone()))
    }
}
