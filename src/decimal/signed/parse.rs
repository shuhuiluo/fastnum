use core::str::from_utf8_unchecked;

use crate::decimal::{
    signed::{Decimal, Sign},
    unsigned, ParseError,
};

pub(crate) const fn from_str<const N: usize>(s: &str) -> Result<Decimal<N>, ParseError> {
    if s.is_empty() {
        return Err(ParseError::Empty);
    }

    let buf = s.as_bytes();

    #[allow(unsafe_code)]
    let (sign, src) = match buf[0] {
        b'+' => (Sign::Plus, unsafe {
            from_utf8_unchecked(buf.split_at(1).1)
        }),
        b'-' => (Sign::Minus, unsafe {
            from_utf8_unchecked(buf.split_at(1).1)
        }),
        _ => (Sign::NoSign, s),
    };

    match unsigned::parse::from_str(src) {
        Ok(value) => Ok(Decimal::new(value, sign)),
        Err(e) => Err(match (e, sign) {
            (ParseError::PosOverflow, Sign::Minus) => ParseError::NegOverflow,
            _ => e,
        }),
    }
}

macro_rules! from_float_impl {
    ($n: ident, $f: ident) => {
        pub(crate) const fn $n<const N: usize>(n: $f) -> Result<Decimal<N>, ParseError> {
            use crate::decimal::{unsigned::parse, utils::types::$f::*};

            if is_nan(n) {
                return Err(ParseError::NaN);
            }

            let b = to_bits(n);

            let sign = b & SIGN_MASK != 0;

            let (sign, n) = if sign {
                (Sign::Minus, -n)
            } else {
                (Sign::NoSign, n)
            };

            match parse::$n(n) {
                Ok(value) => Ok(Decimal::new(value, sign)),
                Err(e) => Err(e),
            }
        }
    };
}

from_float_impl!(from_f32, f32);
from_float_impl!(from_f64, f64);
