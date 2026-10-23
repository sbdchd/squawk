use std::ops::Index;

use crate::arena::{IdxRange, LayeredArena, LayeredMultimap};
use crate::interner::Name;
use crate::ir::{Extension, ExtensionId};

#[derive(Debug, Default)]
pub(crate) struct ExtensionIndex {
    arena: LayeredArena<Extension>,
    by_name: LayeredMultimap<Name, ExtensionId>,
    by_schema: LayeredMultimap<Name, ExtensionId>,
}

impl ExtensionIndex {
    pub(crate) fn on_top_of(base: &'static ExtensionIndex) -> Self {
        ExtensionIndex {
            arena: LayeredArena::on_top_of(&base.arena),
            by_name: LayeredMultimap::on_top_of(&base.by_name),
            by_schema: LayeredMultimap::on_top_of(&base.by_schema),
        }
    }

    pub(crate) fn alloc(&mut self, extension: Extension) -> ExtensionId {
        let Extension { schema, name, .. } = extension;
        let id = self.arena.alloc(extension);
        self.by_name.insert(name, id);
        self.by_schema.insert(schema, id);
        id
    }

    pub(crate) fn in_schema(
        &self,
        schema: Name,
    ) -> impl DoubleEndedIterator<Item = ExtensionId> + '_ {
        self.by_schema.get(schema)
    }

    pub(crate) fn named(&self, name: Name) -> impl Iterator<Item = ExtensionId> + '_ {
        self.by_name.get(name)
    }

    pub(crate) fn next_idx(&self) -> ExtensionId {
        self.arena.next_idx()
    }

    pub(crate) fn range_from(&self, start: ExtensionId) -> IdxRange<Extension> {
        self.arena.range_from(start)
    }

    pub(crate) fn set_schema(&mut self, id: ExtensionId, schema: Name) -> bool {
        let Some(extension) = self.arena.get_local_mut(id) else {
            return false;
        };
        let previous = std::mem::replace(&mut extension.schema, schema);
        if previous != schema {
            self.by_schema.remove(previous, id);
            self.by_schema.insert(schema, id);
        }
        true
    }

    pub(crate) fn rename_schema(&mut self, from: Name, to: Name) {
        for id in self.by_schema.take(from) {
            if let Some(extension) = self.arena.get_local_mut(id) {
                extension.schema = to;
            }
            self.by_schema.insert(to, id);
        }
    }

    pub(crate) fn shrink_to_fit(&mut self) {
        self.arena.shrink_to_fit();
        self.by_name.shrink_to_fit();
        self.by_schema.shrink_to_fit();
    }
}

impl Index<ExtensionId> for ExtensionIndex {
    type Output = Extension;

    fn index(&self, id: ExtensionId) -> &Extension {
        &self.arena[id]
    }
}
