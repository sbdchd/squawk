use std::cmp::Ordering;

use num_bigint::BigUint;
use squawk_syntax::identifier::is_scanner_space;

const DEC_DIGITS: i64 = 4;
const NUMERIC_DSCALE_MAX: i64 = 0x3FFF;
const NUMERIC_MAX_DECIMAL_DIGITS: u32 = 131_072;
const NUMERIC_MAX_DECIMAL_BITS: u64 = 435_412;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct NumericTypmod {
    pub(crate) precision: i32,
    pub(crate) scale: i32,
}

impl NumericTypmod {
    pub(crate) fn new(precision: i64, scale: i64) -> Option<NumericTypmod> {
        if !(1..=1000).contains(&precision) || !(-1000..=1000).contains(&scale) {
            return None;
        }
        Some(NumericTypmod {
            precision: i32::try_from(precision).ok()?,
            scale: i32::try_from(scale).ok()?,
        })
    }

    pub(crate) fn max_digits(self) -> i32 {
        self.precision - self.scale
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NumericInputError {
    InvalidSyntax,
    FieldOverflow,
    InfiniteOverflow,
    FormatOverflow,
}

struct Decimal<'a> {
    text: &'a [u8],
    weight: i64,
    dscale: i64,
}

impl Decimal<'_> {
    fn digits(&self) -> impl Iterator<Item = (i64, u8)> + '_ {
        self.text
            .iter()
            .filter(|byte| byte.is_ascii_digit())
            .zip(0..)
            .map(|(byte, i)| (self.weight - i, byte - b'0'))
    }

    // see: https://github.com/postgres/postgres/blob/b82d69abf64fc0c2fc6fdd491d7cecb8244680c2/src/backend/utils/adt/numeric.c#L7584
    fn check_format(&self) -> Result<(), NumericInputError> {
        if self.dscale > NUMERIC_DSCALE_MAX {
            return Err(NumericInputError::FormatOverflow);
        }
        if let Some((exponent, _)) = self.digits().find(|(_, digit)| *digit != 0)
            && i16::try_from(exponent.div_euclid(DEC_DIGITS)).is_err()
        {
            return Err(NumericInputError::FormatOverflow);
        }
        Ok(())
    }

    // see: https://github.com/postgres/postgres/blob/b82d69abf64fc0c2fc6fdd491d7cecb8244680c2/src/backend/utils/adt/numeric.c#L1229
    fn check_typmod(&self, typmod: NumericTypmod) -> Result<(), NumericInputError> {
        let scale = i64::from(typmod.scale);
        let mut leading = None;
        let mut all_nines = true;
        let mut round_up = false;
        for (exponent, digit) in self.digits() {
            if exponent < -scale {
                round_up = exponent == -scale - 1 && digit >= 5;
                break;
            }
            if leading.is_none() && digit != 0 {
                leading = Some(exponent);
            }
            if leading.is_some() && digit != 9 {
                all_nines = false;
            }
        }
        let leading = match leading {
            Some(exponent) if round_up && all_nines => exponent + 1,
            Some(exponent) => exponent,
            None if round_up => -scale,
            None => return Ok(()),
        };
        if leading >= i64::from(typmod.max_digits()) {
            return Err(NumericInputError::FieldOverflow);
        }
        Ok(())
    }
}

// see: https://github.com/postgres/postgres/blob/b82d69abf64fc0c2fc6fdd491d7cecb8244680c2/src/backend/utils/adt/numeric.c#L626
pub(crate) fn check_numeric(
    text: &str,
    typmod: Option<NumericTypmod>,
) -> Result<(), NumericInputError> {
    let text = text.trim_start_matches(is_scanner_space).as_bytes();
    let unsigned = match text {
        [b'+' | b'-', rest @ ..] => rest,
        _ => text,
    };
    if !matches!(unsigned.first(), Some(b'0'..=b'9' | b'.')) {
        let starts_with = |text: &[u8], prefix: &[u8]| {
            text.get(..prefix.len())
                .is_some_and(|start| start.eq_ignore_ascii_case(prefix))
        };
        let (rest, infinite) = if starts_with(text, b"nan") {
            (&text[3..], false)
        } else if starts_with(unsigned, b"infinity") {
            (&unsigned[8..], true)
        } else if starts_with(unsigned, b"inf") {
            (&unsigned[3..], true)
        } else {
            return Err(NumericInputError::InvalidSyntax);
        };
        if !is_blank(rest) {
            return Err(NumericInputError::InvalidSyntax);
        }
        if infinite && typmod.is_some() {
            return Err(NumericInputError::InfiniteOverflow);
        }
        return Ok(());
    }
    let radix = match unsigned {
        [b'0', b'x' | b'X', ..] => 16,
        [b'0', b'o' | b'O', ..] => 8,
        [b'0', b'b' | b'B', ..] => 2,
        _ => 10,
    };
    let mut buf;
    let decimal = if radix == 10 {
        let (decimal, rest) = parse_decimal(unsigned)?;
        if !is_blank(rest) {
            return Err(NumericInputError::InvalidSyntax);
        }
        decimal
    } else {
        let (value, rest) = parse_non_decimal(&unsigned[2..], radix)?;
        if !is_blank(rest) {
            return Err(NumericInputError::InvalidSyntax);
        }
        match value {
            NonDecimal::Small(value) => {
                buf = [0u8; 39];
                decimal_from_u128(value, &mut buf)
            }
            NonDecimal::Big(digits) => return check_big_integer(digits, radix, typmod),
        }
    };
    decimal.check_format()?;
    match typmod {
        Some(typmod) => decimal.check_typmod(typmod),
        None => Ok(()),
    }
}

// see: https://github.com/postgres/postgres/blob/b82d69abf64fc0c2fc6fdd491d7cecb8244680c2/src/backend/utils/adt/numeric.c#L6817
fn parse_decimal(text: &[u8]) -> Result<(Decimal<'_>, &[u8]), NumericInputError> {
    let is_digit_at = |i: usize| text.get(i).is_some_and(u8::is_ascii_digit);
    let mut i = 0;
    let mut have_dp = false;
    if text.first() == Some(&b'.') {
        have_dp = true;
        i = 1;
    }
    if !is_digit_at(i) {
        return Err(NumericInputError::InvalidSyntax);
    }
    let mut int_digits: i64 = 0;
    let mut frac_digits: i64 = 0;
    while let Some(&byte) = text.get(i) {
        match byte {
            b'0'..=b'9' => {
                if have_dp {
                    frac_digits += 1;
                } else {
                    int_digits += 1;
                }
                i += 1;
            }
            b'.' => {
                if have_dp {
                    return Err(NumericInputError::InvalidSyntax);
                }
                have_dp = true;
                i += 1;
                if text.get(i) == Some(&b'_') {
                    return Err(NumericInputError::InvalidSyntax);
                }
            }
            b'_' => {
                i += 1;
                if !is_digit_at(i) {
                    return Err(NumericInputError::InvalidSyntax);
                }
            }
            _ => break,
        }
    }
    let mantissa = &text[..i];
    let mut exponent: i64 = 0;
    if let Some(b'e' | b'E') = text.get(i) {
        i += 1;
        let neg = match text.get(i) {
            Some(b'+') => {
                i += 1;
                false
            }
            Some(b'-') => {
                i += 1;
                true
            }
            _ => false,
        };
        if !is_digit_at(i) {
            return Err(NumericInputError::InvalidSyntax);
        }
        while let Some(&byte) = text.get(i) {
            match byte {
                b'0'..=b'9' => {
                    exponent = exponent * 10 + i64::from(byte - b'0');
                    if exponent > i64::from(i32::MAX / 2) {
                        return Err(NumericInputError::FormatOverflow);
                    }
                    i += 1;
                }
                b'_' => {
                    i += 1;
                    if !is_digit_at(i) {
                        return Err(NumericInputError::InvalidSyntax);
                    }
                }
                _ => break,
            }
        }
        if neg {
            exponent = -exponent;
        }
    }
    let decimal = Decimal {
        text: mantissa,
        weight: int_digits - 1 + exponent,
        dscale: (frac_digits - exponent).max(0),
    };
    Ok((decimal, &text[i..]))
}

enum NonDecimal<'a> {
    Small(u128),
    Big(&'a [u8]),
}

// see: https://github.com/postgres/postgres/blob/b82d69abf64fc0c2fc6fdd491d7cecb8244680c2/src/backend/utils/adt/numeric.c#L7047
fn parse_non_decimal(
    text: &[u8],
    radix: u32,
) -> Result<(NonDecimal<'_>, &[u8]), NumericInputError> {
    let is_digit_at = |i: usize| {
        text.get(i)
            .is_some_and(|&byte| char::from(byte).is_digit(radix))
    };
    let mut value = Some(0u128);
    let mut i = 0;
    while let Some(&byte) = text.get(i) {
        if let Some(digit) = char::from(byte).to_digit(radix) {
            value = value
                .and_then(|value| value.checked_mul(u128::from(radix)))
                .and_then(|value| value.checked_add(u128::from(digit)));
            i += 1;
        } else if byte == b'_' {
            i += 1;
            if !is_digit_at(i) {
                return Err(NumericInputError::InvalidSyntax);
            }
        } else {
            break;
        }
    }
    if i == 0 {
        return Err(NumericInputError::InvalidSyntax);
    }
    let value = match value {
        Some(value) => NonDecimal::Small(value),
        None => NonDecimal::Big(&text[..i]),
    };
    Ok((value, &text[i..]))
}

fn check_big_integer(
    digits: &[u8],
    radix: u32,
    typmod: Option<NumericTypmod>,
) -> Result<(), NumericInputError> {
    let mut values = Vec::with_capacity(digits.len());
    values.extend(
        digits
            .iter()
            .filter_map(|&byte| char::from(byte).to_digit(radix))
            .filter_map(|digit| u8::try_from(digit).ok()),
    );
    let value = BigUint::from_radix_be(&values, radix).ok_or(NumericInputError::InvalidSyntax)?;
    let ten = BigUint::from(10u32);
    let overflows = match value.bits().cmp(&NUMERIC_MAX_DECIMAL_BITS) {
        Ordering::Less => false,
        Ordering::Equal => value >= ten.pow(NUMERIC_MAX_DECIMAL_DIGITS),
        Ordering::Greater => true,
    };
    if overflows {
        return Err(NumericInputError::FormatOverflow);
    }
    let Some(typmod) = typmod else {
        return Ok(());
    };
    let Ok(max_digits) = u32::try_from(typmod.max_digits()) else {
        return Err(NumericInputError::FieldOverflow);
    };
    let half = match u32::try_from(-typmod.scale - 1) {
        Ok(exponent) => BigUint::from(5u32) * ten.pow(exponent),
        Err(_) => BigUint::ZERO,
    };
    if value + half >= ten.pow(max_digits) {
        return Err(NumericInputError::FieldOverflow);
    }
    Ok(())
}

fn decimal_from_u128(mut value: u128, buf: &mut [u8; 39]) -> Decimal<'_> {
    let mut start = buf.len();
    let mut weight = -1;
    loop {
        start -= 1;
        weight += 1;
        buf[start] = b'0' + (value % 10) as u8;
        value /= 10;
        if value == 0 {
            break;
        }
    }
    Decimal {
        text: &buf[start..],
        weight,
        dscale: 0,
    }
}

fn is_blank(text: &[u8]) -> bool {
    text.iter().all(|&byte| is_scanner_space(char::from(byte)))
}

#[cfg(test)]
mod tests {
    use num_bigint::BigUint;

    use super::{
        NUMERIC_MAX_DECIMAL_BITS, NUMERIC_MAX_DECIMAL_DIGITS, NumericInputError, NumericTypmod,
        check_numeric,
    };

    #[test]
    fn check_numerics() {
        use NumericInputError::{FieldOverflow, FormatOverflow, InfiniteOverflow, InvalidSyntax};

        let typmod = |precision, scale| NumericTypmod::new(precision, scale);

        assert_eq!(check_numeric(" 1.5 ", None), Ok(()));
        assert_eq!(check_numeric("+.5", None), Ok(()));
        assert_eq!(check_numeric("1.", None), Ok(()));
        assert_eq!(check_numeric("1_000.000_1", None), Ok(()));
        assert_eq!(check_numeric("1.e5", None), Ok(()));
        assert_eq!(check_numeric("1e5_0", None), Ok(()));
        assert_eq!(check_numeric("0x_F", None), Ok(()));
        assert_eq!(check_numeric("-0b11", None), Ok(()));
        assert_eq!(check_numeric("nan", None), Ok(()));
        assert_eq!(check_numeric("-inf", None), Ok(()));
        assert_eq!(check_numeric("1e131071", None), Ok(()));
        assert_eq!(check_numeric("1e-16383", None), Ok(()));
        assert_eq!(check_numeric("0e1073741823", None), Ok(()));

        assert_eq!(check_numeric("", None), Err(InvalidSyntax));
        assert_eq!(check_numeric(".", None), Err(InvalidSyntax));
        assert_eq!(check_numeric("1.2.3", None), Err(InvalidSyntax));
        assert_eq!(check_numeric("1._5", None), Err(InvalidSyntax));
        assert_eq!(check_numeric("1_.5", None), Err(InvalidSyntax));
        assert_eq!(check_numeric("1e", None), Err(InvalidSyntax));
        assert_eq!(check_numeric("1e_5", None), Err(InvalidSyntax));
        assert_eq!(check_numeric("0x", None), Err(InvalidSyntax));
        assert_eq!(check_numeric("-NaN", None), Err(InvalidSyntax));
        assert_eq!(check_numeric("infinit", None), Err(InvalidSyntax));
        assert_eq!(check_numeric("- 1", None), Err(InvalidSyntax));

        assert_eq!(check_numeric("1e131072", None), Err(FormatOverflow));
        assert_eq!(check_numeric("1e-16384", None), Err(FormatOverflow));
        assert_eq!(check_numeric("0e-16384", typmod(5, 2)), Err(FormatOverflow));
        assert_eq!(check_numeric("0e1073741824", None), Err(FormatOverflow));

        assert_eq!(check_numeric("9.94", typmod(2, 1)), Ok(()));
        assert_eq!(check_numeric("9.96", typmod(2, 1)), Err(FieldOverflow));
        assert_eq!(check_numeric("0.006", typmod(2, 2)), Ok(()));
        assert_eq!(check_numeric("0.995", typmod(2, 2)), Err(FieldOverflow));
        assert_eq!(check_numeric("99949", typmod(3, -2)), Ok(()));
        assert_eq!(check_numeric("99950", typmod(3, -2)), Err(FieldOverflow));
        assert_eq!(check_numeric("0.0095", typmod(1, 3)), Err(FieldOverflow));
        assert_eq!(check_numeric("5", typmod(1, -4)), Ok(()));
        assert_eq!(check_numeric("0xFF", typmod(3, 0)), Ok(()));
        assert_eq!(check_numeric("0xFF", typmod(2, 0)), Err(FieldOverflow));
        assert_eq!(check_numeric("NaN", typmod(3, 1)), Ok(()));
        assert_eq!(
            check_numeric("Infinity", typmod(3, 1)),
            Err(InfiniteOverflow)
        );

        let big = "0x1FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF";
        assert_eq!(check_numeric(big, None), Ok(()));
        assert_eq!(check_numeric(big, typmod(39, 0)), Ok(()));
        assert_eq!(check_numeric(big, typmod(3, 0)), Err(FieldOverflow));
        assert_eq!(check_numeric(&format!("{big}z"), None), Err(InvalidSyntax));
        assert_eq!(
            check_numeric(&format!("0x{}", "F".repeat(108_800)), None),
            Ok(())
        );
        assert_eq!(
            check_numeric(&format!("0x{}", "F".repeat(108_854)), None),
            Err(FormatOverflow)
        );
        assert_eq!(
            check_numeric(&format!("0x8{}", "0".repeat(108_852)), None),
            Ok(())
        );
        assert_eq!(
            BigUint::from(10u32).pow(NUMERIC_MAX_DECIMAL_DIGITS).bits(),
            NUMERIC_MAX_DECIMAL_BITS
        );
    }
}
