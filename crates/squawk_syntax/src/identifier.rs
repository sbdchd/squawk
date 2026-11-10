// via: https://github.com/postgres/postgres/blob/228a1f9542792c6533ef74c2e7aefad0da1d9a7a/src/include/pg_config_manual.h#L39C6-L39C6
pub const MAX_IDENT_BYTES: usize = 63;

pub fn truncate_identifier(text: &str) -> &str {
    &text[..text.floor_char_boundary(MAX_IDENT_BYTES)]
}
