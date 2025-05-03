// TODO: figure out a better way to handle quoted and unquoted idents
pub(crate) fn trim_quotes(s: &str) -> &str {
    if s.starts_with('"') && s.ends_with('"') {
        &s[1..s.len() - 1]
    } else {
        s
    }
}
