use std::ops::Index;

use smallvec::SmallVec;
use squawk_syntax::SyntaxNodePtr;

use crate::arena::{IdxRange, LayeredArena, LayeredMultimap};
use crate::file::InIrFile;
use crate::interner::{Name, QualifiedName};
use crate::ir::{Column, ColumnTypeRef};
use crate::ir::{Columns, SequenceOwner, Table, TableId, TypeId};

#[derive(Debug, Default)]
pub struct TableIndex {
    arena: LayeredArena<Table>,
    by_name: LayeredMultimap<QualifiedName, TableId>,
    by_schema: LayeredMultimap<Name, TableId>,
    by_owner: LayeredMultimap<TableId, TableId>,
    by_type: LayeredMultimap<TypeId, TableId>,
    by_column_type: LayeredMultimap<ColumnTypeRef, TableId>,
}

impl TableIndex {
    pub(crate) fn on_top_of(base: &'static TableIndex) -> Self {
        TableIndex {
            arena: LayeredArena::on_top_of(&base.arena),
            by_name: LayeredMultimap::on_top_of(&base.by_name),
            by_schema: LayeredMultimap::on_top_of(&base.by_schema),
            by_owner: LayeredMultimap::on_top_of(&base.by_owner),
            by_type: LayeredMultimap::on_top_of(&base.by_type),
            by_column_type: LayeredMultimap::on_top_of(&base.by_column_type),
        }
    }

    pub(crate) fn alloc(&mut self, table: Table, columns: &LayeredArena<Column>) -> TableId {
        let Table {
            name,
            schema,
            sequence_owner,
            of_type,
            columns: table_columns,
            ..
        } = table;
        let id = self.arena.alloc(table);
        for column_type in ColumnTypeRef::in_columns(columns, table_columns.range) {
            self.by_column_type.insert(column_type, id);
        }
        self.by_name.insert(QualifiedName { schema, name }, id);
        self.by_schema.insert(schema, id);
        if let Some(owner) = sequence_owner {
            self.by_owner.insert(owner.table, id);
        }
        if let Some(ty) = of_type {
            self.by_type.insert(ty, id);
        }
        id
    }

    pub(crate) fn named(&self, schema: Name, name: Name) -> impl Iterator<Item = TableId> + '_ {
        self.by_name.get(QualifiedName { schema, name })
    }

    pub fn in_schema(&self, schema: Name) -> impl DoubleEndedIterator<Item = TableId> + '_ {
        self.by_schema.get(schema)
    }

    pub(crate) fn owned_sequences(&self, table: TableId) -> SmallVec<[TableId; 2]> {
        self.by_owner.get(table).collect()
    }

    pub(crate) fn of_type(&self, ty: TypeId) -> impl Iterator<Item = TableId> + '_ {
        self.by_type.get(ty)
    }

    pub(crate) fn with_column_type(
        &self,
        column_type: ColumnTypeRef,
    ) -> impl DoubleEndedIterator<Item = TableId> + '_ {
        self.by_column_type.get(column_type)
    }

    pub(crate) fn next_idx(&self) -> TableId {
        self.arena.next_idx()
    }

    pub(crate) fn range_from(&self, start: TableId) -> IdxRange<Table> {
        self.arena.range_from(start)
    }

    pub(crate) fn is_local(&self, id: TableId) -> bool {
        self.arena.is_local(id)
    }

    pub(crate) fn move_to(
        &mut self,
        id: TableId,
        schema: Name,
        name: Name,
        name_ptr: InIrFile<SyntaxNodePtr>,
    ) {
        self.local_mut(id).name_ptr = name_ptr;
        self.requalify(id, QualifiedName { schema, name });
    }

    pub(crate) fn set_schema(&mut self, id: TableId, schema: Name) -> bool {
        let Some(table) = self.arena.get_local_mut(id) else {
            return false;
        };
        let name = table.name;
        self.requalify(id, QualifiedName { schema, name });
        true
    }

    pub(crate) fn rename_schema(&mut self, from: Name, to: Name) {
        for id in self.by_schema.take(from) {
            let name = self[id].name;
            self.requalify(id, QualifiedName { schema: to, name });
        }
    }

    fn requalify(&mut self, id: TableId, current: QualifiedName) {
        let table = self.local_mut(id);
        let previous = QualifiedName {
            schema: std::mem::replace(&mut table.schema, current.schema),
            name: std::mem::replace(&mut table.name, current.name),
        };
        if previous == current {
            return;
        }
        self.by_name.remove(previous, id);
        self.by_name.insert(current, id);
        if previous.schema != current.schema {
            self.by_schema.remove(previous.schema, id);
            self.by_schema.insert(current.schema, id);
        }
    }

    pub(crate) fn set_columns(
        &mut self,
        id: TableId,
        columns: Columns,
        arena: &LayeredArena<Column>,
    ) -> Columns {
        let previous = std::mem::replace(&mut self.local_mut(id).columns, columns);
        for column_type in ColumnTypeRef::in_columns(arena, previous.range) {
            self.by_column_type.remove(column_type, id);
        }
        for column_type in ColumnTypeRef::in_columns(arena, columns.range) {
            self.by_column_type.insert(column_type, id);
        }
        previous
    }

    pub(crate) fn set_sequence_owner(&mut self, id: TableId, owner: Option<SequenceOwner>) {
        let previous = std::mem::replace(&mut self.local_mut(id).sequence_owner, owner);
        let previous = previous.map(|owner| owner.table);
        let current = owner.map(|owner| owner.table);
        if previous != current {
            if let Some(table) = previous {
                self.by_owner.remove(table, id);
            }
            if let Some(table) = current {
                self.by_owner.insert(table, id);
            }
        }
    }

    pub(crate) fn set_of_type(&mut self, id: TableId, ty: Option<TypeId>) -> Option<TypeId> {
        let previous = std::mem::replace(&mut self.local_mut(id).of_type, ty);
        if previous != ty {
            if let Some(previous) = previous {
                self.by_type.remove(previous, id);
            }
            if let Some(ty) = ty {
                self.by_type.insert(ty, id);
            }
        }
        previous
    }

    pub(crate) fn set_constraints_complete(&mut self, id: TableId, complete: bool) {
        self.local_mut(id).constraints_complete = complete;
    }

    fn local_mut(&mut self, id: TableId) -> &mut Table {
        self.arena
            .get_local_mut(id)
            .expect("base layer is immutable")
    }

    pub(crate) fn shrink_to_fit(&mut self) {
        self.arena.shrink_to_fit();
        self.by_name.shrink_to_fit();
        self.by_schema.shrink_to_fit();
        self.by_owner.shrink_to_fit();
        self.by_type.shrink_to_fit();
        self.by_column_type.shrink_to_fit();
    }
}

impl Index<TableId> for TableIndex {
    type Output = Table;

    fn index(&self, id: TableId) -> &Table {
        &self.arena[id]
    }
}
