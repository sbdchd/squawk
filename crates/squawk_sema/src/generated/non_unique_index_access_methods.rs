// pg version: 18.3
// update via:
//   cargo xtask sync-builtins

pub(crate) const NON_UNIQUE_INDEX_ACCESS_METHODS: &[&str] = &[
    "bloom", "brin", "gin", "gist", "hash", "hnsw", "ivfflat", "spgist",
];
