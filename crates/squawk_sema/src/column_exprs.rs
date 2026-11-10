use rustc_hash::FxHashMap;

use crate::interner::Name;
use crate::ir::TableId;
use crate::sema_error::GeneratedKind;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct ColumnExprId(pub(crate) u32);

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum ColumnExprKind {
    Default,
    Generated { kind: GeneratedKind },
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct ColumnExpr {
    pub(crate) id: ColumnExprId,
    pub(crate) kind: ColumnExprKind,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ColumnKey {
    pub(crate) table: TableId,
    pub(crate) column: Name,
}

#[derive(Default)]
pub(crate) struct ColumnExprs {
    by_column: FxHashMap<ColumnKey, ColumnExpr>,
    by_id: FxHashMap<ColumnExprId, ColumnKey>,
}

impl ColumnExprs {
    pub(crate) fn get(&self, key: ColumnKey) -> Option<ColumnExpr> {
        self.by_column.get(&key).copied()
    }

    pub(crate) fn column_of(&self, id: ColumnExprId) -> Option<ColumnKey> {
        self.by_id.get(&id).copied()
    }

    pub(crate) fn set(
        &mut self,
        key: ColumnKey,
        expression: Option<ColumnExpr>,
    ) -> Option<ColumnExpr> {
        let previous = match expression {
            Some(expression) => self.by_column.insert(key, expression),
            None => self.by_column.remove(&key),
        };
        if let Some(previous) = previous {
            self.by_id.remove(&previous.id);
        }
        if let Some(expression) = expression {
            self.by_id.insert(expression.id, key);
        }
        previous
    }
}
