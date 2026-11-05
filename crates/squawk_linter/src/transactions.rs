use rustc_hash::FxHashSet;
use smallvec::SmallVec;
use squawk_syntax::ast;

use crate::name::Name;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TransactionControl {
    Begin,
    End { and_chain: bool },
}

impl TransactionControl {
    pub(crate) fn from_stmt(stmt: &ast::Stmt) -> Option<Self> {
        match stmt {
            ast::Stmt::Begin(_) => Some(Self::Begin),
            ast::Stmt::Commit(ast::Commit::CommitTransaction(commit)) => Some(Self::End {
                and_chain: matches!(commit.chain_clause(), Some(ast::ChainClause::AndChain(_))),
            }),
            ast::Stmt::Rollback(ast::Rollback::RollbackTransaction(rollback)) => Some(Self::End {
                and_chain: matches!(rollback.chain_clause(), Some(ast::ChainClause::AndChain(_))),
            }),
            _ => None,
        }
    }
}

pub(crate) struct TransactionState {
    inside_transaction: bool,
}

impl TransactionState {
    pub(crate) fn new(assume_in_transaction: bool) -> Self {
        Self {
            inside_transaction: assume_in_transaction,
        }
    }

    pub(crate) fn update(&mut self, stmt: &ast::Stmt) -> Option<TransactionControl> {
        let control = TransactionControl::from_stmt(stmt)?;
        self.inside_transaction = match control {
            TransactionControl::Begin => true,
            TransactionControl::End { and_chain } => self.inside_transaction && and_chain,
        };
        Some(control)
    }

    pub(crate) fn inside_transaction(&self) -> bool {
        self.inside_transaction
    }
}

pub(crate) struct TablesCreatedInTransaction {
    names: FxHashSet<SmallVec<[Name; 2]>>,
    transaction: TransactionState,
}

pub(crate) fn table_name_ref(path: ast::PathRef) -> SmallVec<[Name; 2]> {
    let mut name: SmallVec<[Name; 2]> = std::iter::successors(Some(path), |path| path.qualifier())
        .filter_map(|path| path.segment())
        .map(|segment| Name::from_node(&segment))
        .collect();
    strip_public_schema(&mut name);
    name
}

fn strip_public_schema(name: &mut SmallVec<[Name; 2]>) {
    if let [_, schema] = name.as_slice()
        && schema.is_public_schema()
    {
        name.pop();
    }
}

impl TablesCreatedInTransaction {
    pub(crate) fn new(assume_in_transaction: bool) -> Self {
        Self {
            names: FxHashSet::default(),
            transaction: TransactionState::new(assume_in_transaction),
        }
    }

    pub(crate) fn update(&mut self, stmt: &ast::Stmt) {
        match self.transaction.update(stmt) {
            Some(TransactionControl::End { .. }) => {
                self.names.clear();
                return;
            }
            Some(TransactionControl::Begin) => return,
            None => (),
        }
        let inside_transaction = self.transaction.inside_transaction();
        let table = match stmt {
            ast::Stmt::CreateTable(table) if inside_transaction => table.table_name(),
            ast::Stmt::CreateTableAs(table) if inside_transaction => table.table_name(),
            ast::Stmt::SelectInto(select) if inside_transaction => {
                select.into_clause().and_then(|into| into.table_name())
            }
            _ => None,
        };
        if let Some(path) = table.and_then(|table| table.path())
            && let Some(segment) = path.segment()
        {
            let mut name = std::iter::once(Name::from_node(&segment))
                .chain(
                    std::iter::successors(path.qualifier(), |path| path.qualifier())
                        .filter_map(|path| path.segment())
                        .map(|segment| Name::from_node(&segment)),
                )
                .collect();
            strip_public_schema(&mut name);
            self.names.insert(name);
        }
    }

    pub(crate) fn contains(&self, path: &ast::PathRef) -> bool {
        !self.names.is_empty() && self.names.contains(&table_name_ref(path.clone()))
    }
}
