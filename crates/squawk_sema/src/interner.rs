use std::hash::BuildHasher;

use hashbrown::HashTable;
use rustc_hash::FxBuildHasher;

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
    text: String,
    spans: Vec<Span>,
    table: HashTable<Name>,
}

#[derive(Debug, Clone, Copy)]
struct Span {
    start: u32,
    end: u32,
}

fn local_str<'a>(text: &'a str, spans: &[Span], index: usize) -> &'a str {
    let span = spans[index];
    &text[span.start as usize..span.end as usize]
}

impl Interner {
    pub(crate) fn on_top_of(base: &'static Interner) -> Self {
        assert!(base.base.is_none());
        Interner {
            base: Some(base),
            text: String::new(),
            spans: vec![],
            table: HashTable::new(),
        }
    }

    pub(crate) fn intern(&mut self, text: &str) -> Name {
        let hash = FxBuildHasher.hash_one(text);
        if let Some(name) = self.lookup_hashed(text, hash) {
            return name;
        }
        let base_len = self.base_len();
        let name = Name(u32::try_from(base_len + self.spans.len()).unwrap());
        let start = u32::try_from(self.text.len()).unwrap();
        self.text.push_str(text);
        let end = u32::try_from(self.text.len()).unwrap();
        self.spans.push(Span { start, end });
        let (buf, spans) = (&self.text, &self.spans);
        self.table.insert_unique(hash, name, |name| {
            FxBuildHasher.hash_one(local_str(buf, spans, name.0 as usize - base_len))
        });
        name
    }

    pub(crate) fn shrink_to_fit(&mut self) {
        self.text.shrink_to_fit();
        self.spans.shrink_to_fit();
        let base_len = self.base_len();
        let (buf, spans) = (&self.text, &self.spans);
        self.table.shrink_to_fit(|name| {
            FxBuildHasher.hash_one(local_str(buf, spans, name.0 as usize - base_len))
        });
    }

    pub fn lookup(&self, text: &str) -> Option<Name> {
        self.lookup_hashed(text, FxBuildHasher.hash_one(text))
    }

    fn lookup_hashed(&self, text: &str, hash: u64) -> Option<Name> {
        self.base
            .and_then(|base| base.lookup_hashed(text, hash))
            .or_else(|| {
                let base_len = self.base_len();
                self.table
                    .find(hash, |name| {
                        local_str(&self.text, &self.spans, name.0 as usize - base_len) == text
                    })
                    .copied()
            })
    }

    pub fn resolve(&self, name: Name) -> &str {
        let i = name.0 as usize;
        match self.base {
            Some(base) if i < base.spans.len() => base.resolve(name),
            _ => local_str(&self.text, &self.spans, i - self.base_len()),
        }
    }

    fn base_len(&self) -> usize {
        self.base.map_or(0, |base| base.spans.len())
    }
}
