use std::ops::Index;

use crate::arena::{IdxRange, LayeredArena, LayeredMultimap};
use crate::interner::Name;
use crate::ir::{Constraint, ConstraintId, ConstraintKind, ConstraintOwner, IndexId};

#[derive(Debug, Default)]
pub(crate) struct ConstraintIndex {
    arena: LayeredArena<Constraint>,
    by_name: LayeredMultimap<Name, ConstraintId>,
    by_owner: LayeredMultimap<ConstraintOwner, ConstraintId>,
    by_index: LayeredMultimap<IndexId, ConstraintId>,
}

impl ConstraintIndex {
    pub(crate) fn on_top_of(base: &'static ConstraintIndex) -> Self {
        ConstraintIndex {
            arena: LayeredArena::on_top_of(&base.arena),
            by_name: LayeredMultimap::on_top_of(&base.by_name),
            by_owner: LayeredMultimap::on_top_of(&base.by_owner),
            by_index: LayeredMultimap::on_top_of(&base.by_index),
        }
    }

    pub(crate) fn alloc(&mut self, constraint: Constraint) -> ConstraintId {
        let Constraint {
            name, owner, index, ..
        } = constraint;
        let id = self.arena.alloc(constraint);
        self.by_name.insert(name, id);
        self.by_owner.insert(owner, id);
        if let Some(index) = index {
            self.by_index.insert(index, id);
        }
        id
    }

    pub(crate) fn named(&self, name: Name) -> impl DoubleEndedIterator<Item = ConstraintId> + '_ {
        self.by_name.get(name)
    }

    pub(crate) fn owned_by(
        &self,
        owner: ConstraintOwner,
    ) -> impl DoubleEndedIterator<Item = ConstraintId> + '_ {
        self.by_owner.get(owner)
    }

    pub(crate) fn backed_by(
        &self,
        index: IndexId,
    ) -> impl DoubleEndedIterator<Item = ConstraintId> + '_ {
        self.by_index.get(index)
    }

    pub(crate) fn next_idx(&self) -> ConstraintId {
        self.arena.next_idx()
    }

    pub(crate) fn range_from(&self, start: ConstraintId) -> IdxRange<Constraint> {
        self.arena.range_from(start)
    }

    pub(crate) fn set_kind(&mut self, id: ConstraintId, kind: ConstraintKind) -> ConstraintKind {
        std::mem::replace(&mut self.local_mut(id).kind, kind)
    }

    pub(crate) fn set_index(&mut self, id: ConstraintId, index: Option<IndexId>) {
        let previous = std::mem::replace(&mut self.local_mut(id).index, index);
        if previous != index {
            if let Some(previous) = previous {
                self.by_index.remove(previous, id);
            }
            if let Some(index) = index {
                self.by_index.insert(index, id);
            }
        }
    }

    fn local_mut(&mut self, id: ConstraintId) -> &mut Constraint {
        self.arena
            .get_local_mut(id)
            .expect("base layer is immutable")
    }

    pub(crate) fn shrink_to_fit(&mut self) {
        self.arena.shrink_to_fit();
        self.by_name.shrink_to_fit();
        self.by_owner.shrink_to_fit();
        self.by_index.shrink_to_fit();
    }
}

impl Index<ConstraintId> for ConstraintIndex {
    type Output = Constraint;

    fn index(&self, id: ConstraintId) -> &Constraint {
        &self.arena[id]
    }
}
