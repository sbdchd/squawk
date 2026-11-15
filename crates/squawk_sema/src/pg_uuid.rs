const UUID_LEN: usize = 16;

// see: https://github.com/postgres/postgres/blob/b82d69abf64fc0c2fc6fdd491d7cecb8244680c2/src/backend/utils/adt/uuid.c#L131
pub(crate) fn is_valid_uuid(text: &str) -> bool {
    let mut rest = text.as_bytes();
    let braces = if let [b'{', tail @ ..] = rest {
        rest = tail;
        true
    } else {
        false
    };
    for i in 0..UUID_LEN {
        let [hi, lo, tail @ ..] = rest else {
            return false;
        };
        if !hi.is_ascii_hexdigit() || !lo.is_ascii_hexdigit() {
            return false;
        }
        rest = tail;
        if i % 2 == 1
            && i < UUID_LEN - 1
            && let [b'-', tail @ ..] = rest
        {
            rest = tail;
        }
    }
    if braces {
        let [b'}', tail @ ..] = rest else {
            return false;
        };
        rest = tail;
    }
    rest.is_empty()
}

#[cfg(test)]
mod tests {
    use super::is_valid_uuid;

    #[test]
    fn valid_uuids() {
        assert!(is_valid_uuid("a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11"));
        assert!(is_valid_uuid("A0EEBC99-9C0B-4EF8-BB6D-6BB9BD380A11"));
        assert!(is_valid_uuid("{a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11}"));
        assert!(is_valid_uuid("a0eebc999c0b4ef8bb6d6bb9bd380a11"));
        assert!(is_valid_uuid("a0ee-bc99-9c0b-4ef8-bb6d-6bb9-bd38-0a11"));
        assert!(is_valid_uuid("{a0eebc99-9c0b4ef8-bb6d6bb9-bd380a11}"));

        assert!(!is_valid_uuid(""));
        assert!(!is_valid_uuid("not-a-uuid"));
        assert!(!is_valid_uuid("a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a1"));
        assert!(!is_valid_uuid("a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a111"));
        assert!(!is_valid_uuid("a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a1g"));
        assert!(!is_valid_uuid("a0eeb-c99-9c0b-4ef8-bb6d-6bb9bd380a11"));
        assert!(!is_valid_uuid("a0eebc99--9c0b-4ef8-bb6d-6bb9bd380a11"));
        assert!(!is_valid_uuid("a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11-"));
        assert!(!is_valid_uuid("-a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11"));
        assert!(!is_valid_uuid("{a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11"));
        assert!(!is_valid_uuid("a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11}"));
        assert!(!is_valid_uuid(" a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11"));
        assert!(!is_valid_uuid("a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11 "));
    }
}
