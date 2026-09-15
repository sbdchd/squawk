#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum IntegerRadix {
    Binary,
    Decimal,
    Hexadecimal,
    Octal,
}

impl IntegerRadix {
    pub(crate) fn base(self) -> u32 {
        match self {
            Self::Binary => 2,
            Self::Decimal => 10,
            Self::Hexadecimal => 16,
            Self::Octal => 8,
        }
    }
}

pub(crate) fn normalize_integer_literal(text: &str) -> (IntegerRadix, String) {
    let (radix, digits) = match text.as_bytes() {
        [b'0', b'b' | b'B', ..] => (IntegerRadix::Binary, &text[2..]),
        [b'0', b'o' | b'O', ..] => (IntegerRadix::Octal, &text[2..]),
        [b'0', b'x' | b'X', ..] => (IntegerRadix::Hexadecimal, &text[2..]),
        _ => (IntegerRadix::Decimal, text),
    };
    let digits = if radix == IntegerRadix::Decimal {
        digits
    } else {
        digits.strip_prefix('_').unwrap_or(digits)
    };

    (radix, digits.replace('_', ""))
}

pub(crate) fn binary_digits_to_hex(digits: &str) -> Option<String> {
    const HEX_DIGITS: &[u8; 16] = b"0123456789ABCDEF";

    if digits.is_empty() {
        return Some("".to_string());
    }

    let mut out = String::with_capacity(digits.len().div_ceil(4));
    let mut start = 0;

    while start < digits.len() {
        let chunk_len = if start == 0 {
            match digits.len() % 4 {
                0 => 4,
                n => n,
            }
        } else {
            4
        };
        let end = start + chunk_len;
        let value = u8::from_str_radix(&digits[start..end], 2).ok()?;
        out.push(HEX_DIGITS[value as usize] as char);
        start = end;
    }

    Some(out)
}

pub(crate) fn hex_digits_to_binary(digits: &str) -> Option<String> {
    const BINARY_DIGITS: [&str; 16] = [
        "0000", "0001", "0010", "0011", "0100", "0101", "0110", "0111", "1000", "1001", "1010",
        "1011", "1100", "1101", "1110", "1111",
    ];

    if digits.is_empty() {
        return Some("".to_string());
    }

    let mut out = String::with_capacity(digits.len() * 4);
    for ch in digits.chars() {
        let value = ch.to_digit(16)? as usize;
        out.push_str(BINARY_DIGITS[value]);
    }

    Some(out)
}
