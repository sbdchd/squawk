use std::fmt;

use squawk_syntax::identifier::is_scanner_space;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum IntType {
    Int2,
    Int4,
    Int8,
}

impl fmt::Display for IntType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            IntType::Int2 => "int2",
            IntType::Int4 => "int4",
            IntType::Int8 => "int8",
        })
    }
}

impl IntType {
    fn bits(self) -> u32 {
        match self {
            IntType::Int2 => 16,
            IntType::Int4 => 32,
            IntType::Int8 => 64,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum IntInputError {
    InvalidSyntax,
    OutOfRange,
}

// see: https://github.com/postgres/postgres/blob/b82d69abf64fc0c2fc6fdd491d7cecb8244680c2/src/backend/utils/adt/numutils.c#L388
pub(crate) fn parse_int(text: &str, ty: IntType) -> Result<i64, IntInputError> {
    let limit = 1u64 << (ty.bits() - 1);
    let mut rest = text.trim_start_matches(is_scanner_space).as_bytes();
    let neg = match rest {
        [b'-', tail @ ..] => {
            rest = tail;
            true
        }
        [b'+', tail @ ..] => {
            rest = tail;
            false
        }
        _ => false,
    };
    let (radix, digits) = match rest {
        [b'0', b'x' | b'X', digits @ ..] => (16, digits),
        [b'0', b'o' | b'O', digits @ ..] => (8, digits),
        [b'0', b'b' | b'B', digits @ ..] => (2, digits),
        _ => (10, rest),
    };
    let is_digit = |byte: u8| char::from(byte).is_digit(radix);
    let mut magnitude: u64 = 0;
    let mut i = 0;
    while let Some(&byte) = digits.get(i) {
        if let Some(digit) = char::from(byte).to_digit(radix) {
            if magnitude > limit / u64::from(radix) {
                return Err(IntInputError::OutOfRange);
            }
            magnitude = magnitude * u64::from(radix) + u64::from(digit);
            i += 1;
        } else if byte == b'_' {
            if radix == 10 && i == 0 {
                return Err(IntInputError::InvalidSyntax);
            }
            i += 1;
            if !digits.get(i).is_some_and(|&byte| is_digit(byte)) {
                return Err(IntInputError::InvalidSyntax);
            }
        } else {
            break;
        }
    }
    if i == 0
        || !digits[i..]
            .iter()
            .all(|&byte| is_scanner_space(char::from(byte)))
    {
        return Err(IntInputError::InvalidSyntax);
    }
    if neg {
        if magnitude > limit {
            return Err(IntInputError::OutOfRange);
        }
        return 0i64
            .checked_sub_unsigned(magnitude)
            .ok_or(IntInputError::OutOfRange);
    }
    if magnitude >= limit {
        return Err(IntInputError::OutOfRange);
    }
    i64::try_from(magnitude).map_err(|_| IntInputError::OutOfRange)
}

#[cfg(test)]
mod tests {
    use super::{IntInputError, IntType, parse_int};

    #[test]
    fn parse_ints() {
        use IntInputError::{InvalidSyntax, OutOfRange};
        use IntType::{Int2, Int4, Int8};

        assert_eq!(parse_int(" 42 ", Int4), Ok(42));
        assert_eq!(parse_int("+1", Int4), Ok(1));
        assert_eq!(parse_int("1_000", Int4), Ok(1000));
        assert_eq!(parse_int("0x1F", Int4), Ok(31));
        assert_eq!(parse_int("0x_1F", Int4), Ok(31));
        assert_eq!(parse_int("0o17", Int4), Ok(15));
        assert_eq!(parse_int("0b101", Int4), Ok(5));
        assert_eq!(parse_int("-32768", Int2), Ok(-32768));
        assert_eq!(parse_int("-0x8000", Int2), Ok(-32768));
        assert_eq!(parse_int("-9223372036854775808", Int8), Ok(i64::MIN));
        assert_eq!(parse_int("9223372036854775807", Int8), Ok(i64::MAX));

        assert_eq!(parse_int("", Int4), Err(InvalidSyntax));
        assert_eq!(parse_int("-", Int4), Err(InvalidSyntax));
        assert_eq!(parse_int("- 1", Int4), Err(InvalidSyntax));
        assert_eq!(parse_int("_1", Int4), Err(InvalidSyntax));
        assert_eq!(parse_int("1_", Int4), Err(InvalidSyntax));
        assert_eq!(parse_int("1__0", Int4), Err(InvalidSyntax));
        assert_eq!(parse_int("0X", Int4), Err(InvalidSyntax));
        assert_eq!(parse_int("0b2", Int4), Err(InvalidSyntax));
        assert_eq!(parse_int("1.5", Int4), Err(InvalidSyntax));
        assert_eq!(parse_int("1e3", Int4), Err(InvalidSyntax));

        assert_eq!(parse_int("32768", Int2), Err(OutOfRange));
        assert_eq!(parse_int("0x8000", Int2), Err(OutOfRange));
        assert_eq!(parse_int("-2147483649", Int4), Err(OutOfRange));
        assert_eq!(parse_int("9223372036854775808", Int8), Err(OutOfRange));
        assert_eq!(parse_int("-9223372036854775809", Int8), Err(OutOfRange));
        assert_eq!(parse_int("99999999999999999999x", Int8), Err(OutOfRange));
    }
}
