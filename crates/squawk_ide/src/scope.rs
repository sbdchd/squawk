use la_arena::Idx;
use rustc_hash::FxHashMap;

use crate::symbols::{Name, SymbolId};

pub(crate) type ScopeId = Idx<Scope>;

#[derive(Default, Debug, Clone, PartialEq)]
pub(crate) struct Scope {
    #[allow(dead_code)]
    pub(crate) parent: Option<ScopeId>,
    pub(crate) entries: FxHashMap<Name, Vec<SymbolId>>,
}

impl Scope {
    pub(crate) fn with_parent(parent: Option<ScopeId>) -> Self {
        Scope {
            parent,
            entries: FxHashMap::default(),
        }
    }

    pub(crate) fn insert(&mut self, name: Name, id: SymbolId) {
        self.entries.entry(name).or_default().push(id);
    }

    pub(crate) fn get(&self, name: &Name) -> Option<&[SymbolId]> {
        self.entries.get(name).map(|ids| ids.as_slice())
    }
}
