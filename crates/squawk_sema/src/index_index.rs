use smallvec::SmallVec;
use squawk_syntax::SyntaxNodePtr;

use crate::arena::{IdxRange, LayeredArena, LayeredMultimap};
use crate::file::InIrFile;
use crate::interner::{Name, QualifiedName};
use crate::ir::{Index, IndexId, TableId};

#[derive(Debug, Default)]
pub struct IndexIndex {
    arena: LayeredArena<Index>,
    keys: LayeredArena<Name>,
    by_name: LayeredMultimap<QualifiedName, IndexId>,
    by_schema: LayeredMultimap<Name, IndexId>,
    by_table: LayeredMultimap<TableId, IndexId>,
}

impl IndexIndex {
    pub(crate) fn on_top_of(base: &'static IndexIndex) -> Self {
        IndexIndex {
            arena: LayeredArena::on_top_of(&base.arena),
            keys: LayeredArena::on_top_of(&base.keys),
            by_name: LayeredMultimap::on_top_of(&base.by_name),
            by_schema: LayeredMultimap::on_top_of(&base.by_schema),
            by_table: LayeredMultimap::on_top_of(&base.by_table),
        }
    }

    pub(crate) fn alloc(&mut self, index: Index) -> IndexId {
        let Index {
            schema,
            name,
            table,
            ..
        } = index;
        let id = self.arena.alloc(index);
        self.by_name.insert(QualifiedName { schema, name }, id);
        self.by_schema.insert(schema, id);
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

    pub(crate) fn key(&self, id: IndexId) -> Option<impl Iterator<Item = Name> + '_> {
        let key = self.arena[id].key?;
        Some(key.iter().map(|column| self.keys[column]))
    }

    pub(crate) fn set_key(
        &mut self,
        id: IndexId,
        key: Option<IdxRange<Name>>,
    ) -> Option<IdxRange<Name>> {
        std::mem::replace(&mut self.local_mut(id).key, key)
    }

    pub(crate) fn named(&self, schema: Name, name: Name) -> impl Iterator<Item = IndexId> + '_ {
        self.by_name.get(QualifiedName { schema, name })
    }

    pub(crate) fn in_schema(&self, schema: Name) -> impl DoubleEndedIterator<Item = IndexId> + '_ {
        self.by_schema.get(schema)
    }

    pub(crate) fn on_table(&self, table: TableId) -> SmallVec<[IndexId; 4]> {
        self.by_table.get(table).rev().collect()
    }

    pub(crate) fn next_idx(&self) -> IndexId {
        self.arena.next_idx()
    }

    pub(crate) fn range_from(&self, start: IndexId) -> IdxRange<Index> {
        self.arena.range_from(start)
    }

    pub(crate) fn is_local(&self, id: IndexId) -> bool {
        self.arena.is_local(id)
    }

    pub(crate) fn move_to(
        &mut self,
        id: IndexId,
        schema: Name,
        name: Name,
        name_ptr: InIrFile<SyntaxNodePtr>,
    ) {
        self.local_mut(id).name_ptr = name_ptr;
        self.requalify(id, QualifiedName { schema, name });
    }

    pub(crate) fn set_schema(&mut self, id: IndexId, schema: Name) -> bool {
        let Some(index) = self.arena.get_local_mut(id) else {
            return false;
        };
        let name = index.name;
        self.requalify(id, QualifiedName { schema, name });
        true
    }

    pub(crate) fn rename_schema(&mut self, from: Name, to: Name) {
        for id in self.by_schema.take(from) {
            let name = self.arena[id].name;
            self.requalify(id, QualifiedName { schema: to, name });
        }
    }

    fn requalify(&mut self, id: IndexId, current: QualifiedName) {
        let index = self.local_mut(id);
        let previous = QualifiedName {
            schema: std::mem::replace(&mut index.schema, current.schema),
            name: std::mem::replace(&mut index.name, current.name),
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

    fn local_mut(&mut self, id: IndexId) -> &mut Index {
        self.arena
            .get_local_mut(id)
            .expect("base layer is immutable")
    }

    pub(crate) fn shrink_to_fit(&mut self) {
        self.arena.shrink_to_fit();
        self.keys.shrink_to_fit();
        self.by_name.shrink_to_fit();
        self.by_schema.shrink_to_fit();
        self.by_table.shrink_to_fit();
    }
}

impl std::ops::Index<IndexId> for IndexIndex {
    type Output = Index;

    fn index(&self, id: IndexId) -> &Index {
        &self.arena[id]
    }
}
