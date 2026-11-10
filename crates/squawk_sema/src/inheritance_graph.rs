use std::collections::hash_map::Entry;

use rustc_hash::FxHashMap;
use smallvec::{SmallVec, smallvec};

use crate::ir::TableId;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum InheritanceKind {
    Inherits,
    Partition,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct Inheritance {
    pub(crate) parent: TableId,
    pub(crate) child: TableId,
    pub(crate) kind: InheritanceKind,
}

#[derive(Default)]
pub(crate) struct InheritanceGraph {
    by_parent: FxHashMap<TableId, SmallVec<[Inheritance; 2]>>,
    by_child: FxHashMap<TableId, SmallVec<[Inheritance; 2]>>,
}

impl InheritanceGraph {
    pub(crate) fn add(&mut self, inheritance: Inheritance) -> bool {
        let parents = self.by_child.entry(inheritance.child).or_default();
        if parents.contains(&inheritance) {
            return false;
        }
        parents.push(inheritance);
        self.by_parent
            .entry(inheritance.parent)
            .or_default()
            .push(inheritance);
        true
    }

    pub(crate) fn remove(&mut self, inheritance: Inheritance) -> bool {
        Self::remove_from(&mut self.by_child, inheritance.child, inheritance)
            && Self::remove_from(&mut self.by_parent, inheritance.parent, inheritance)
    }

    fn remove_from(
        map: &mut FxHashMap<TableId, SmallVec<[Inheritance; 2]>>,
        table: TableId,
        inheritance: Inheritance,
    ) -> bool {
        let Entry::Occupied(mut entry) = map.entry(table) else {
            return false;
        };
        let Some(position) = entry
            .get()
            .iter()
            .position(|&existing| existing == inheritance)
        else {
            return false;
        };
        entry.get_mut().remove(position);
        if entry.get().is_empty() {
            entry.remove();
        }
        true
    }

    pub(crate) fn children(&self, parent: TableId) -> &[Inheritance] {
        self.by_parent
            .get(&parent)
            .map_or(&[], |children| children.as_slice())
    }

    pub(crate) fn parents(&self, child: TableId) -> &[Inheritance] {
        self.by_child
            .get(&child)
            .map_or(&[], |parents| parents.as_slice())
    }

    pub(crate) fn descendants(
        &self,
        table: TableId,
        is_live: impl Fn(TableId) -> bool,
    ) -> SmallVec<[TableId; 8]> {
        Self::walk(table, is_live, |table| {
            self.children(table)
                .iter()
                .map(|inheritance| inheritance.child)
        })
    }

    pub(crate) fn ancestors(
        &self,
        table: TableId,
        is_live: impl Fn(TableId) -> bool,
    ) -> SmallVec<[TableId; 8]> {
        Self::walk(table, is_live, |table| {
            self.parents(table)
                .iter()
                .map(|inheritance| inheritance.parent)
        })
    }

    fn walk<I: Iterator<Item = TableId>>(
        table: TableId,
        is_live: impl Fn(TableId) -> bool,
        next: impl Fn(TableId) -> I,
    ) -> SmallVec<[TableId; 8]> {
        let mut found: SmallVec<[TableId; 8]> = smallvec![];
        let mut stack: SmallVec<[TableId; 8]> = smallvec![table];
        while let Some(current) = stack.pop() {
            for related in next(current) {
                if related != table && is_live(related) && !found.contains(&related) {
                    found.push(related);
                    stack.push(related);
                }
            }
        }
        found
    }
}
