use core::str::from_utf8_unchecked;

use crate::{
    decimal::{unsigned::UnsignedDecimal, ParseError},
    int::UInt,
};

/// Creates and initializes a Decimal from string.
pub(crate) const fn from_str<const N: usize>(s: &str) -> Result<UnsignedDecimal<N>, ParseError> {
    if s.is_empty() {
        return Err(ParseError::Empty);
    }

    let buf = s.as_bytes();
    let len = buf.len();

    let mut decimal_offset: Option<i128> = None;
    let mut exponent_value: Option<i128> = None;
    let mut dot = None;

    let mut is_first_digit = true;

    // Digits are stored as little endian (the least significant digit is first).
    let mut value = UInt::ZERO;

    let mut i = 0;

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
                return Err(ParseError::PosOverflow);
            };

            let next = UInt::from_digit(n);
            let Some(v) = v.checked_add(next) else {
                return Err(ParseError::PosOverflow);
            };

            value = v;
        }

        if let Some(current) = decimal_offset {
            let Some(dig) = checked_usize_i128(digits_count) else {
                return Err(ParseError::ExponentOverflow);
            };
            let Some(current) = current.checked_add(dig) else {
                return Err(ParseError::ExponentOverflow);
            };
            decimal_offset = Some(current);
        } else if let Some(dot_pos) = dot {
            let Some(dig) = checked_usize_i128(digits_count) else {
                return Err(ParseError::ExponentOverflow);
            };
            let Some(pos) = checked_usize_i128(dot_pos) else {
                return Err(ParseError::ExponentOverflow);
            };
            let Some(current) = dig.checked_sub(pos) else {
                return Err(ParseError::ExponentOverflow);
            };
            decimal_offset = Some(current);
        }
    }

    let scale = match make_scale(decimal_offset, exponent_value) {
        Ok(scale) => scale,
        Err(e) => {
            return Err(e);
        }
    };

    Ok(UnsignedDecimal::new(value, scale))
}

macro_rules! from_float_impl {
    ($n: ident, $f: ident) => {
        pub(crate) const fn $n<const N: usize>(n: $f) -> Result<UnsignedDecimal<N>, ParseError> {
            use crate::decimal::utils::types::$f::*;

            if is_nan(n) {
                return Err(ParseError::NaN);
            }

            let b = to_bits(n);

            let sign = b & SIGN_MASK != 0;

            if sign {
                return Err(ParseError::Signed);
            }

            let frac = b & MAN_MASK;
            let exp = b & EXP_MASK;

            if frac == 0 && exp == EXP_MASK {
                return Err(ParseError::Infinite);
            }

            if frac == 0 && exp == 0 {
                return Ok(UnsignedDecimal::ZERO);
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
                let scale = Subnormal::<N>::POW as i64;

                Ok(UnsignedDecimal::new(result, scale))
            } else {
                // normal

                let frac = frac | MAN_MASK_NORMAL;
                let pow = (exp >> (MANTISSA_DIGITS - 1)) as i64
                    - (MAX_EXP - 1)
                    - (MANTISSA_DIGITS - 1) as i64;

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
                    let reduced_pow = pow + trailing_zeros as i64;

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

                Ok(UnsignedDecimal::new(result, scale))
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
const fn checked_usize_i128(u: usize) -> Option<i128> {
    let max = i128::MAX as usize;
    if u > max {
        None
    } else {
        Some(u as i128)
    }
}

#[inline]
const fn i128_i64(u: i128) -> Result<i64, ParseError> {
    let min = i64::MIN as i128;
    let max = i64::MAX as i128;
    if u < min || u > max {
        Err(ParseError::ExponentOverflow)
    } else {
        Ok(u as i64)
    }
}

#[inline]
const fn make_scale(
    decimal_offset: Option<i128>,
    exponent_value: Option<i128>,
) -> Result<i64, ParseError> {
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

    i128_i64(scale)
}

#[inline]
const fn parse_exp(buf: &[u8], pos: usize) -> Result<i128, ParseError> {
    if pos >= buf.len() {
        return Err(ParseError::Empty);
    }

    #[allow(unsafe_code)]
    let src = unsafe { from_utf8_unchecked(buf.split_at(pos).1) };

    match i128::from_str_radix(src, 10) {
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
