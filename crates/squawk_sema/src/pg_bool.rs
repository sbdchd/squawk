use squawk_syntax::identifier::is_scanner_space;

pub(crate) const BOOL_INPUTS: [&str; 6] = ["true", "false", "yes", "no", "on", "off"];

// see: https://github.com/postgres/postgres/blob/b82d69abf64fc0c2fc6fdd491d7cecb8244680c2/src/backend/utils/adt/bool.c#L127
pub(crate) fn parse_bool(text: &str) -> Option<bool> {
    let text = text.trim_matches(is_scanner_space);
    let first = text.bytes().next()?;
    let is_prefix_of = |keyword: &str| {
        text.len() <= keyword.len() && keyword[..text.len()].eq_ignore_ascii_case(text)
    };
    match first.to_ascii_lowercase() {
        b't' if is_prefix_of("true") => Some(true),
        b'f' if is_prefix_of("false") => Some(false),
        b'y' if is_prefix_of("yes") => Some(true),
        b'n' if is_prefix_of("no") => Some(false),
        b'o' if text.len() >= 2 && is_prefix_of("on") => Some(true),
        b'o' if text.len() >= 2 && is_prefix_of("off") => Some(false),
        b'1' if text.len() == 1 => Some(true),
        b'0' if text.len() == 1 => Some(false),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::parse_bool;

    #[test]
    fn parse_bools() {
        assert_eq!(parse_bool("t"), Some(true));
        assert_eq!(parse_bool("TRUE"), Some(true));
        assert_eq!(parse_bool(" yes\n"), Some(true));
        assert_eq!(parse_bool("of"), Some(false));
        assert_eq!(parse_bool("On"), Some(true));
        assert_eq!(parse_bool("n"), Some(false));
        assert_eq!(parse_bool("0"), Some(false));
        assert_eq!(parse_bool("1"), Some(true));
        assert_eq!(parse_bool(""), None);
        assert_eq!(parse_bool("o"), None);
        assert_eq!(parse_bool("01"), None);
        assert_eq!(parse_bool("truex"), None);
        assert_eq!(parse_bool("ture"), None);
        assert_eq!(parse_bool("é"), None);
    }
}
