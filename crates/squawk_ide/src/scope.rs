use rustc_hash::FxHashMap;

use crate::name::AsName;
use crate::symbols::{Name, SymbolId};

#[derive(Default, Debug, Clone, PartialEq)]
pub(crate) struct Scope {
    pub(crate) entries: FxHashMap<Name, Vec<SymbolId>>,
}

impl Scope {
    pub(crate) fn insert(&mut self, name: Name, id: SymbolId) {
        self.entries.entry(name).or_default().push(id);
    }

    pub(crate) fn get<N: AsName + ?Sized>(&self, name: &N) -> Option<&[SymbolId]> {
        self.entries.get(name.as_name()).map(|ids| ids.as_slice())
    }
}
