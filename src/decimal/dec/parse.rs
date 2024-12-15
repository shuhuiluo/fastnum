use core::str::from_utf8_unchecked;

use crate::{
    decimal::{Decimal, Flags, ParseError},
    int::UInt,
};

/// Creates and initializes a Decimal from string.
pub(crate) const fn from_str<const N: usize>(s: &str) -> Result<Decimal<N>, ParseError> {
    if s.is_empty() {
        return Err(ParseError::Empty);
    }

    let buf = s.as_bytes();
    let len = buf.len();
    let mut flags = Flags::default();

    let mut i = 0;

    // Parse sign or `NAN` special case
    match buf[0] {
        b'+' => {
            i = 1;
        }
        b'-' => {
            flags = flags.neg();
            i = 1;
        }
        b'n' | b'N' => {
            if bytes_equal_ci(buf.split_at(1).1, b"an") {
                return Ok(Decimal::NAN);
            }
        }
        _ => {}
    }

    if i == len {
        return Err(ParseError::Empty);
    }

    // Parse special cases Inf/Infinity
    if bytes_equal_ci(buf.split_at(i).1, b"inf") || bytes_equal_ci(buf.split_at(i).1, b"infinity") {
        return if flags.is_negative() {
            Ok(Decimal::NEG_INFINITY)
        } else {
            Ok(Decimal::INFINITY)
        };
    }

    let mut decimal_offset: Option<i32> = None;
    let mut exponent_value: Option<i32> = None;
    let mut dot = None;

    let mut is_first_digit = true;

    // Digits are stored as little endian (the least significant digit is first).
    let mut value = UInt::ZERO;

    while i < len {
        let mut digits_count = 0;
        let mut n = 0;

        while i < len && (digits_count < POWER) {
            let b = buf[i];
            let d = match b {
                b'.' => {
                    if dot.is_some() {
                        return Err(ParseError::InvalidLiteral);
                    } else {
                        dot = Some(digits_count);
                        i += 1;
                        continue;
                    }
                }
                b'_' => {
                    i += 1;
                    continue;
                }
                b'E' | b'e' => {
                    if exponent_value.is_some() {
                        return Err(ParseError::InvalidLiteral);
                    } else {
                        match parse_exp(buf, i + 1) {
                            Ok(exp) => {
                                exponent_value = Some(exp);
                            }
                            Err(e) => {
                                return Err(e);
                            }
                        }
                        i = len;
                        break;
                    }
                }
                b'0'..=b'9' => b - b'0',
                _ => {
                    return Err(ParseError::InvalidLiteral);
                }
            };

            n = n * (RADIX as Digit) + d as Digit;
            digits_count += 1;
            i += 1;
        }

        if is_first_digit {
            if digits_count == 0 {
                return Err(ParseError::Empty);
            }
            value = UInt::from_digit(n);
            is_first_digit = false;
        } else if digits_count > 0 {
            let multiplier = UInt::from_digit(base(digits_count as u64));
            let Some(v) = value.checked_mul(multiplier) else {
                return Err(overflow(flags));
            };

            let next = UInt::from_digit(n);
            let Some(v) = v.checked_add(next) else {
                return Err(overflow(flags));
            };

            value = v;
        }

        if let Some(current) = decimal_offset {
            let Some(dig) = checked_usize_i32(digits_count) else {
                return Err(ParseError::ExponentOverflow);
            };
            let Some(current) = current.checked_add(dig) else {
                return Err(ParseError::ExponentOverflow);
            };
            decimal_offset = Some(current);
        } else if let Some(dot_pos) = dot {
            let Some(dig) = checked_usize_i32(digits_count) else {
                return Err(ParseError::ExponentOverflow);
            };
            let Some(pos) = checked_usize_i32(dot_pos) else {
                return Err(ParseError::ExponentOverflow);
            };
            let Some(current) = dig.checked_sub(pos) else {
                return Err(ParseError::ExponentOverflow);
            };
            decimal_offset = Some(current);
        }
    }

    // TODO: Adjust scale & subnormal & etc 
    let scale = match make_scale(decimal_offset, exponent_value) {
        Ok(scale) => scale,
        Err(e) => {
            return Err(e);
        }
    };

    Ok(Decimal::new(value, scale, flags))
}

macro_rules! from_float_impl {
    ($n: ident, $f: ident) => {
        pub(crate) const fn $n<const N: usize>(n: $f) -> Result<Decimal<N>, ParseError> {
            use crate::decimal::utils::types::$f::*;

            if is_nan(n) {
                return Ok(Decimal::NAN);
            }

            let b = to_bits(n);

            let flags = if b & SIGN_MASK != 0 {
                Flags::NEG
            } else {
                Flags::EMPTY
            };

            let frac = b & MAN_MASK;
            let exp = b & EXP_MASK;

            if frac == 0 && exp == EXP_MASK {
                return Ok(Decimal::INFINITY.with_flags(flags));
            }

            if frac == 0 && exp == 0 {
                return Ok(Decimal::ZERO.with_flags(flags));
            }

            if exp == 0 {
                // subnormal

                let Some(subnormal_base) = Subnormal::<N>::SUBNORMAL_BASE else {
                    return Err(ParseError::PosOverflow);
                };

                let result = match subnormal_base.overflowing_mul(uint(frac)) {
                    (r, false) => r,
                    _ => {
                        return Err(ParseError::PosOverflow);
                    }
                };
                let scale = Subnormal::<N>::POW as i16;

                Ok(Decimal::new(result, scale, flags))
            } else {
                // normal

                // TODO: i16 (!)
                let frac = frac | MAN_MASK_NORMAL;
                let pow = (exp >> (MANTISSA_DIGITS - 1)) as i16
                    - (MAX_EXP - 1) as i16
                    - (MANTISSA_DIGITS - 1) as i16;

                let result;
                let scale;

                if pow == 0 {
                    result = uint(frac);
                    scale = 0;
                } else if pow < 0 {
                    let mut trailing_zeros = frac.trailing_zeros();
                    if trailing_zeros > -pow as u32 {
                        trailing_zeros = -pow as u32;
                    }

                    let reduced_frac = frac >> trailing_zeros;
                    let reduced_pow = pow + trailing_zeros as i16;

                    let (shift, false) = UInt::from_digit(5).overflowing_pow(-reduced_pow as u32)
                    else {
                        return Err(ParseError::PosOverflow);
                    };

                    result = match shift.overflowing_mul(uint(reduced_frac)) {
                        (r, false) => r,
                        _ => {
                            return Err(ParseError::PosOverflow);
                        }
                    };
                    scale = -reduced_pow;
                } else {
                    let (shift, false) = UInt::from_digit(2).overflowing_pow(pow as u32) else {
                        return Err(ParseError::PosOverflow);
                    };
                    result = match shift.overflowing_mul(uint(frac)) {
                        (r, false) => r,
                        _ => {
                            return Err(ParseError::PosOverflow);
                        }
                    };
                    scale = 0;
                }

                Ok(Decimal::new(result, scale, flags))
            }
        }
    };
}

from_float_impl!(from_f32, f32);
from_float_impl!(from_f64, f64);

type Digit = u64;

const RADIX: u8 = 10;
const POWER: usize = 19;

#[inline]
const fn checked_usize_i32(u: usize) -> Option<i32> {
    let max = i32::MAX as usize;
    if u > max {
        None
    } else {
        Some(u as i32)
    }
}

#[inline]
const fn i32_i16(u: i32) -> Result<i16, ParseError> {
    let min = i16::MIN as i32;
    let max = i16::MAX as i32;
    if u < min || u > max {
        Err(ParseError::ExponentOverflow)
    } else {
        Ok(u as i16)
    }
}

#[inline]
const fn make_scale(
    decimal_offset: Option<i32>,
    exponent_value: Option<i32>,
) -> Result<i16, ParseError> {
    let decimal_offset = match decimal_offset {
        Some(decimal_offset) => decimal_offset,
        None => 0,
    };

    let exponent_value = match exponent_value {
        Some(exponent_value) => exponent_value,
        None => 0,
    };

    let Some(scale) = decimal_offset.checked_sub(exponent_value) else {
        return Err(ParseError::ExponentOverflow);
    };

    i32_i16(scale)
}

#[inline]
const fn parse_exp(buf: &[u8], pos: usize) -> Result<i32, ParseError> {
    if pos >= buf.len() {
        return Err(ParseError::Empty);
    }

    #[allow(unsafe_code)]
    let src = unsafe { from_utf8_unchecked(buf.split_at(pos).1) };

    match i32::from_str_radix(src, 10) {
        Ok(exp) => Ok(exp),
        Err(e) => {
            let e = ParseError::from_int_error_kind(e.kind());
            Err(match e {
                ParseError::PosOverflow => ParseError::ExponentOverflow,
                ParseError::NegOverflow => ParseError::ExponentOverflow,
                _ => e,
            })
        }
    }
}

#[inline]
const fn base(n: Digit) -> Digit {
    match n {
        0 => 1,
        1 => 10,
        2 => 100,
        3 => 1000,
        4 => 10000,
        5 => 100000,
        6 => 1000000,
        7 => 10000000,
        8 => 100000000,
        9 => 1000000000,
        10 => 10000000000,
        11 => 100000000000,
        12 => 1000000000000,
        13 => 10000000000000,
        14 => 100000000000000,
        15 => 1000000000000000,
        16 => 10000000000000000,
        17 => 100000000000000000,
        18 => 1000000000000000000,
        19 => 10000000000000000000,
        _ => panic!("base number overflow"),
    }
}

#[inline]
const fn bytes_equal_ci(lhs: &[u8], rhs: &[u8]) -> bool {
    if lhs.len() != rhs.len() {
        return false;
    }
    let mut i = 0;
    while i < lhs.len() {
        if lhs[i].to_ascii_lowercase() != rhs[i].to_ascii_lowercase() {
            return false;
        }
        i += 1;
    }
    true
}

#[inline(always)]
const fn overflow(flags: Flags) -> ParseError {
    if flags.is_negative() {
        ParseError::NegOverflow
    } else {
        ParseError::PosOverflow
    }
}

// #[inline]
// const fn str_equal_ci(lhs: &str, rhs: &str) -> bool {
//     bytes_equal_ci(lhs.as_bytes(), rhs.as_bytes())
// }
