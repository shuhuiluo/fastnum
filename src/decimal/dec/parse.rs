use core::str::from_utf8_unchecked;

use crate::{
    decimal::{
        dec::{construct::construct, ControlBlock},
        Context, Decimal, DecimalError, ParseError, Sign,
    },
    int::UInt,
};

/// Creates and initializes a Decimal from string.
pub(crate) const fn from_str<const N: usize>(
    s: &str,
    ctx: Context,
) -> Result<Decimal<N>, ParseError> {
    if s.is_empty() {
        return Err(ParseError::Empty);
    }

    let buf = s.as_bytes();
    let len = buf.len();
    let mut cb = ControlBlock::default().set_context(ctx);

    let mut i = 0;

    // Parse sign or `NAN` special case
    match buf[0] {
        b'+' => {
            i = 1;
        }
        b'-' => {
            cb = cb.neg();
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
        return Ok(Decimal::INFINITY.with_cb(cb));
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
                return Err(overflow(cb.sign()));
            };

            let next = UInt::from_digit(n);
            let Some(v) = v.checked_add(next) else {
                return Err(overflow(cb.sign()));
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

    let exp = match make_exp(decimal_offset, exponent_value) {
        Ok(exp) => exp,
        Err(e) => {
            return Err(e);
        }
    };

    let dec = construct(value, exp, cb);

    if dec.is_nan() {
        return Err(ParseError::Unknown);
    } else if dec.is_infinite() {
        return Err(overflow(dec.sign()));
    }

    match dec.ok_or_err() {
        Ok(dec) => Ok(dec),
        Err(e) => Err(match e {
            DecimalError::Overflow => overflow(dec.sign()),
            DecimalError::Underflow => overflow(dec.sign()),
            _ => ParseError::Unknown,
        }),
    }
}

macro_rules! from_float_impl {
    ($n: ident, $f: ident) => {
        pub(crate) const fn $n<const N: usize>(n: $f) -> Result<Decimal<N>, ParseError> {
            use crate::decimal::utils::types::$f::*;

            if is_nan(n) {
                return Ok(Decimal::NAN);
            }

            let b = to_bits(n);

            let cb = if b & SIGN_MASK != 0 {
                ControlBlock::default().neg()
            } else {
                ControlBlock::default()
            };

            let frac = b & MAN_MASK;
            let exp = b & EXP_MASK;

            if frac == 0 && exp == EXP_MASK {
                return Ok(Decimal::INFINITY.with_cb(cb));
            }

            if frac == 0 && exp == 0 {
                return Ok(Decimal::ZERO.with_cb(cb));
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

                Ok(Decimal::new(result, scale, cb))
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

                Ok(Decimal::new(result, scale, cb))
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
const fn make_exp(
    decimal_offset: Option<i32>,
    exponent_value: Option<i32>,
) -> Result<i32, ParseError> {
    let decimal_offset = if let Some(decimal_offset) = decimal_offset {
        decimal_offset
    } else {
        0
    };

    let exponent_value = if let Some(exponent_value) = exponent_value {
        exponent_value
    } else {
        0
    };

    let Some(exp) = exponent_value.checked_sub(decimal_offset) else {
        return Err(ParseError::ExponentOverflow);
    };

    Ok(exp)
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
const fn overflow(sign: Sign) -> ParseError {
    if matches!(sign, Sign::Minus) {
        ParseError::NegOverflow
    } else {
        ParseError::PosOverflow
    }
}
