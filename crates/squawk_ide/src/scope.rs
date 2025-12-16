use la_arena::Idx;
use std::collections::HashMap;

use crate::symbols::{Name, SymbolId};

pub(crate) type ScopeId = Idx<Scope>;

#[derive(Default, Debug)]
pub(crate) struct Scope {
    #[allow(dead_code)]
    pub(crate) parent: Option<ScopeId>,
    pub(crate) entries: HashMap<Name, Vec<SymbolId>>,
}

impl Scope {
    pub(crate) fn with_parent(parent: Option<ScopeId>) -> Self {
        Scope {
            parent,
            entries: HashMap::new(),
        }
    }

    pub(crate) fn insert(&mut self, name: Name, id: SymbolId) {
        self.entries.entry(name).or_default().push(id);
    }

    pub(crate) fn get(&self, name: &Name) -> Option<&[SymbolId]> {
        self.entries.get(name).map(|ids| ids.as_slice())
    }
}
