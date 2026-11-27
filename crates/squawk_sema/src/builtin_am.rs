#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BuiltinAm {
    Btree,
    Gist,
    Hash,
}

impl BuiltinAm {
    pub(crate) fn name(self) -> &'static str {
        match self {
            BuiltinAm::Btree => "btree",
            BuiltinAm::Gist => "gist",
            BuiltinAm::Hash => "hash",
        }
    }
}
