use rustc_hash::FxHashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Name(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct QualifiedName {
    pub(crate) schema: Name,
    pub(crate) name: Name,
}

#[derive(Debug, Default)]
pub struct Interner {
    base: Option<&'static Interner>,
    names: Vec<Box<str>>,
    map: FxHashMap<Box<str>, Name>,
}

impl Interner {
    pub(crate) fn on_top_of(base: &'static Interner) -> Self {
        assert!(base.base.is_none());
        Interner {
            base: Some(base),
            names: vec![],
            map: FxHashMap::default(),
        }
    }

    pub(crate) fn intern(&mut self, text: &str) -> Name {
        if let Some(name) = self.lookup(text) {
            return name;
        }
        let name = Name(u32::try_from(self.base_len() + self.names.len()).unwrap());
        self.names.push(text.into());
        self.map.insert(text.into(), name);
        name
    }

    pub(crate) fn shrink_to_fit(&mut self) {
        self.names.shrink_to_fit();
        self.map.shrink_to_fit();
    }

    pub fn lookup(&self, text: &str) -> Option<Name> {
        self.base
            .and_then(|base| base.lookup(text))
            .or_else(|| self.map.get(text).copied())
    }

    pub fn resolve(&self, name: Name) -> &str {
        let i = name.0 as usize;
        match self.base {
            Some(base) if i < base.names.len() => base.resolve(name),
            _ => &self.names[i - self.base_len()],
        }
    }

    fn base_len(&self) -> usize {
        self.base.map_or(0, |base| base.names.len())
    }
}
