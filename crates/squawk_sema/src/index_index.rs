use smallvec::SmallVec;
use squawk_syntax::SyntaxNodePtr;

use crate::arena::{IdxRange, LayeredArena, LayeredMultimap};
use crate::file::InIrFile;
use crate::interner::{Name, QualifiedName};
use crate::ir::{Index, IndexId, TableId};
use crate::qualified_index::QualifiedIndex;

#[derive(Debug, Default)]
pub struct IndexIndex {
    indexes: QualifiedIndex<Index>,
    keys: LayeredArena<Name>,
    by_table: LayeredMultimap<TableId, IndexId>,
}

impl IndexIndex {
    pub(crate) fn on_top_of(base: &'static IndexIndex) -> Self {
        IndexIndex {
            indexes: QualifiedIndex::on_top_of(&base.indexes),
            keys: LayeredArena::on_top_of(&base.keys),
            by_table: LayeredMultimap::on_top_of(&base.by_table),
        }
    }

    pub(crate) fn alloc(&mut self, index: Index) -> IndexId {
        let table = index.table;
        let id = self.indexes.alloc(index);
        if let Some(table) = table {
            self.by_table.insert(table, id);
        }
        id
    }

    pub(crate) fn alloc_key(&mut self, columns: impl IntoIterator<Item = Name>) -> IdxRange<Name> {
        self.keys.alloc_many(columns)
    }

    pub(crate) fn alloc_key_copies(&mut self, key: IdxRange<Name>) -> IdxRange<Name> {
        self.keys.alloc_copies(key)
    }

    pub(crate) fn alloc_renamed_key(
        &mut self,
        key: IdxRange<Name>,
        from: Name,
        to: Name,
    ) -> IdxRange<Name> {
        self.keys
            .alloc_mapped_copies(key, |column| if column == from { to } else { column })
    }

    pub(crate) fn key_names(&self, key: IdxRange<Name>) -> impl Iterator<Item = Name> + '_ {
        key.iter().map(|column| self.keys[column])
    }

    pub(crate) fn key(&self, id: IndexId) -> Option<impl Iterator<Item = Name> + '_> {
        let key = self.indexes[id].key?;
        Some(key.iter().map(|column| self.keys[column]))
    }

    pub(crate) fn set_key(
        &mut self,
        id: IndexId,
        key: Option<IdxRange<Name>>,
    ) -> Option<IdxRange<Name>> {
        std::mem::replace(&mut self.indexes.local_mut(id).key, key)
    }

    pub(crate) fn columns(&self, id: IndexId) -> impl Iterator<Item = Name> + '_ {
        self.indexes[id]
            .columns
            .iter()
            .map(|column| self.keys[column])
    }

    pub(crate) fn set_columns(&mut self, id: IndexId, columns: IdxRange<Name>) -> IdxRange<Name> {
        std::mem::replace(&mut self.indexes.local_mut(id).columns, columns)
    }

    pub(crate) fn set_parent(&mut self, id: IndexId, parent: Option<IndexId>) -> Option<IndexId> {
        std::mem::replace(&mut self.indexes.local_mut(id).parent, parent)
    }

    pub(crate) fn qualified(&self) -> &QualifiedIndex<Index> {
        &self.indexes
    }

    pub(crate) fn in_schema(&self, schema: Name) -> impl DoubleEndedIterator<Item = IndexId> + '_ {
        self.indexes.in_schema(schema)
    }

    pub(crate) fn on_table(&self, table: TableId) -> SmallVec<[IndexId; 4]> {
        self.by_table.get(table).rev().collect()
    }

    pub(crate) fn next_idx(&self) -> IndexId {
        self.indexes.next_idx()
    }

    pub(crate) fn range_from(&self, start: IndexId) -> IdxRange<Index> {
        self.indexes.range_from(start)
    }

    pub(crate) fn is_local(&self, id: IndexId) -> bool {
        self.indexes.is_local(id)
    }

    pub(crate) fn move_to(
        &mut self,
        id: IndexId,
        schema: Name,
        name: Name,
        name_ptr: InIrFile<SyntaxNodePtr>,
    ) {
        self.indexes.local_mut(id).name_ptr = name_ptr;
        self.indexes.requalify(id, QualifiedName { schema, name });
    }

    pub(crate) fn set_schema(&mut self, id: IndexId, schema: Name) -> bool {
        self.indexes.set_schema(id, schema)
    }

    pub(crate) fn rename_schema(&mut self, from: Name, to: Name) {
        self.indexes.rename_schema(from, to);
    }

    pub(crate) fn shrink_to_fit(&mut self) {
        self.indexes.shrink_to_fit();
        self.keys.shrink_to_fit();
        self.by_table.shrink_to_fit();
    }
}

impl std::ops::Index<IndexId> for IndexIndex {
    type Output = Index;

    fn index(&self, id: IndexId) -> &Index {
        &self.indexes[id]
    }
}
