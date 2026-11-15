use std::{cmp::Ordering, fmt};

use squawk_syntax::identifier::is_scanner_space;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FloatType {
    Float4,
    Float8,
}

impl fmt::Display for FloatType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            FloatType::Float4 => "float4",
            FloatType::Float8 => "float8",
        })
    }
}

impl FloatType {
    fn mantissa_bits(self) -> u32 {
        match self {
            FloatType::Float4 => f32::MANTISSA_DIGITS,
            FloatType::Float8 => f64::MANTISSA_DIGITS,
        }
    }

    fn min_exp(self) -> i32 {
        match self {
            FloatType::Float4 => f32::MIN_EXP,
            FloatType::Float8 => f64::MIN_EXP,
        }
    }

    fn max_exp(self) -> i32 {
        match self {
            FloatType::Float4 => f32::MAX_EXP,
            FloatType::Float8 => f64::MAX_EXP,
        }
    }

    fn underflow_exponent(self) -> i64 {
        i64::from(self.min_exp()) - 1 - i64::from(self.mantissa_bits())
    }

    fn max_exponent(self) -> i64 {
        i64::from(self.max_exp()) - 1
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FloatInputError {
    InvalidSyntax,
    OutOfRange,
}

// see: https://github.com/postgres/postgres/blob/26bd08fb5c069fbdf55867adc398cfca0f55d5c8/src/backend/utils/adt/float.c#L436
pub(crate) fn check_float(text: &str, ty: FloatType) -> Result<(), FloatInputError> {
    let text = text.trim_start_matches(is_scanner_space);
    let unsigned = text.strip_prefix(['+', '-']).unwrap_or(text);
    let bytes = unsigned.as_bytes();
    let (len, out_of_range) = if let Some(len) = special_float_len(bytes) {
        (len, false)
    } else if let Some(hex) = scan_hex_float(bytes) {
        (hex.len, hex.out_of_range(ty))
    } else if let Some(len) = scan_decimal_float(bytes) {
        (len, decimal_float_out_of_range(&unsigned[..len], ty))
    } else {
        return Err(FloatInputError::InvalidSyntax);
    };
    if out_of_range {
        return Err(FloatInputError::OutOfRange);
    }
    if !bytes[len..]
        .iter()
        .all(|&byte| is_scanner_space(char::from(byte)))
    {
        return Err(FloatInputError::InvalidSyntax);
    }
    Ok(())
}

fn special_float_len(text: &[u8]) -> Option<usize> {
    let starts_with = |prefix: &str| {
        text.get(..prefix.len())
            .is_some_and(|head| head.eq_ignore_ascii_case(prefix.as_bytes()))
    };
    if starts_with("infinity") {
        return Some(8);
    }
    if starts_with("inf") {
        return Some(3);
    }
    if !starts_with("nan") {
        return None;
    }
    if text.get(3) == Some(&b'(')
        && let Some(close) = text[4..].iter().position(|&byte| byte == b')')
    {
        return Some(4 + close + 1);
    }
    Some(3)
}

fn scan_digits(text: &[u8], is_digit: impl Fn(u8) -> bool) -> usize {
    text.iter().take_while(|&&byte| is_digit(byte)).count()
}

struct Mantissa {
    int_digits: usize,
    len: usize,
}

impl Mantissa {
    fn scan(text: &[u8], is_digit: impl Fn(u8) -> bool + Copy) -> Option<Mantissa> {
        let int_digits = scan_digits(text, is_digit);
        let frac_digits = match text.get(int_digits) {
            Some(b'.') => scan_digits(&text[int_digits + 1..], is_digit),
            _ => 0,
        };
        if int_digits == 0 && frac_digits == 0 {
            return None;
        }
        let len = match text.get(int_digits) {
            Some(b'.') => int_digits + 1 + frac_digits,
            _ => int_digits,
        };
        Some(Mantissa { int_digits, len })
    }
}

struct Exponent {
    len: usize,
    value: i64,
}

fn scan_exponent(text: &[u8], markers: [u8; 2]) -> Exponent {
    let none = Exponent { len: 0, value: 0 };
    let Some((marker, rest)) = text.split_first() else {
        return none;
    };
    if !markers.contains(marker) {
        return none;
    }
    let (neg, digits) = match rest {
        [b'-', digits @ ..] => (true, digits),
        [b'+', digits @ ..] => (false, digits),
        digits => (false, digits),
    };
    let count = scan_digits(digits, |byte| byte.is_ascii_digit());
    if count == 0 {
        return none;
    }
    let magnitude = digits[..count].iter().fold(0i64, |value, &byte| {
        value
            .saturating_mul(10)
            .saturating_add(i64::from(byte - b'0'))
    });
    Exponent {
        len: rest.len() - digits.len() + count + 1,
        value: if neg { -magnitude } else { magnitude },
    }
}

fn scan_decimal_float(text: &[u8]) -> Option<usize> {
    let len = Mantissa::scan(text, |byte| byte.is_ascii_digit())?.len;
    Some(len + scan_exponent(&text[len..], *b"eE").len)
}

fn decimal_float_out_of_range(number: &str, ty: FloatType) -> bool {
    let (is_zero, is_infinite) = match ty {
        FloatType::Float4 => match number.parse::<f32>() {
            Ok(value) => (value == 0.0, value.is_infinite()),
            Err(_) => return false,
        },
        FloatType::Float8 => match number.parse::<f64>() {
            Ok(value) => (value == 0.0, value.is_infinite()),
            Err(_) => return false,
        },
    };
    if is_infinite {
        return true;
    }
    let mantissa_end = number.find(['e', 'E']).unwrap_or(number.len());
    let nonzero = number.as_bytes()[..mantissa_end]
        .iter()
        .any(|byte| (b'1'..=b'9').contains(byte));
    is_zero && nonzero
}

struct HexFloat<'a> {
    len: usize,
    int_digits: usize,
    digits: &'a [u8],
    exponent: i64,
}

fn scan_hex_float(text: &[u8]) -> Option<HexFloat<'_>> {
    let [b'0', b'x' | b'X', rest @ ..] = text else {
        return None;
    };
    let mantissa = Mantissa::scan(rest, |byte| byte.is_ascii_hexdigit())?;
    let mantissa_len = mantissa.len;
    let exponent = scan_exponent(&rest[mantissa_len..], *b"pP");
    Some(HexFloat {
        len: 2 + mantissa_len + exponent.len,
        int_digits: mantissa.int_digits,
        digits: &rest[..mantissa_len],
        exponent: exponent.value,
    })
}

impl HexFloat<'_> {
    fn bits(&self) -> impl Iterator<Item = bool> + Clone + '_ {
        self.digits
            .iter()
            .filter_map(|&byte| char::from(byte).to_digit(16))
            .flat_map(|digit| (0..4).rev().map(move |shift| digit >> shift & 1 == 1))
    }

    fn top_bit_exponent(&self, leading_zeros: usize) -> i64 {
        let int_bits = i64::try_from(4 * self.int_digits).unwrap_or(i64::MAX);
        let leading_zeros = i64::try_from(leading_zeros).unwrap_or(i64::MAX);
        self.exponent
            .saturating_add(int_bits)
            .saturating_sub(leading_zeros)
            .saturating_sub(1)
    }

    fn out_of_range(&self, ty: FloatType) -> bool {
        let Some(leading_zeros) = self.bits().position(|bit| bit) else {
            return false;
        };
        let significant = self.bits().skip(leading_zeros);
        let exponent = self.top_bit_exponent(leading_zeros);
        match exponent.cmp(&ty.max_exponent()) {
            Ordering::Greater => return true,
            Ordering::Equal => return starts_with_ones(significant, ty.mantissa_bits() + 1),
            Ordering::Less => {}
        }
        match exponent.cmp(&ty.underflow_exponent()) {
            Ordering::Less => true,
            Ordering::Equal => is_power_of_two(significant),
            Ordering::Greater => false,
        }
    }
}

fn starts_with_ones(bits: impl Iterator<Item = bool>, count: u32) -> bool {
    let count = usize::try_from(count).unwrap_or(usize::MAX);
    bits.take(count).filter(|&bit| bit).count() == count
}

fn is_power_of_two(mut bits: impl Iterator<Item = bool>) -> bool {
    bits.next();
    bits.all(|bit| !bit)
}

#[cfg(test)]
mod tests {
    use super::{FloatInputError, FloatType, check_float};

    #[test]
    fn check_floats() {
        use FloatInputError::{InvalidSyntax, OutOfRange};
        use FloatType::{Float4, Float8};

        for text in [
            " 1.5 ",
            "+.5",
            "1.",
            "1.e5",
            "-1E-5",
            "0e-999",
            "0.000e99999999999999999999",
            "4e-320",
            "3e-324",
            "1.7976931348623157e308",
            "NaN",
            "-nan",
            "nan(abc)",
            "nan(a b)",
            "INF",
            "-Infinity",
            "0x1p3",
            "-0X1.8P1",
            "0x.8",
            "0x1.",
            "0x1fffffffffffffp971",
            "0x1.fffffffffffff7fp1023",
            "0x1p-1074",
            "0x1.0000000000001p-1075",
            "0x0p99999999999999999999",
        ] {
            assert_eq!(check_float(text, Float8), Ok(()), "{text}");
        }
        assert_eq!(check_float("3.4028235e38", Float4), Ok(()));
        assert_eq!(check_float("1e-45", Float4), Ok(()));
        assert_eq!(check_float("0x1.fffffep127", Float4), Ok(()));
        assert_eq!(check_float("0x1.000002p-150", Float4), Ok(()));

        for text in [
            "", " ", "-", ".", ".e1", "e1", "1e", "1e+", "1_0", "1.5x", "0x", "0x1p", "0x.p1",
            "infin", "infx", "nan(", "nan(a)x", "1 2",
        ] {
            assert_eq!(check_float(text, Float8), Err(InvalidSyntax), "{text}");
        }

        for text in [
            "1e400",
            "-1e400",
            "1e400x",
            "1e99999999999999999999",
            "0x1p1024",
            "0x1.fffffffffffff8p1023",
        ] {
            assert_eq!(check_float(text, Float8), Err(OutOfRange), "{text}");
        }
        for text in ["1e-400", "2e-324", "0x1p-1075", "-0x1p-1076"] {
            assert_eq!(check_float(text, Float8), Err(OutOfRange), "{text}");
        }
        assert_eq!(check_float("1e39", Float4), Err(OutOfRange));
        assert_eq!(check_float("0x1.ffffffp127", Float4), Err(OutOfRange));
        assert_eq!(check_float("1e-46", Float4), Err(OutOfRange));
        assert_eq!(check_float("0x1p-150", Float4), Err(OutOfRange));
    }
}
