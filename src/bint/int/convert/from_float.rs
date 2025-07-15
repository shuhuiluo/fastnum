use crate::bint::{ParseError, Int, UInt, doc};

macro_rules! from_float_impl {
    () => {
        impl<const N: usize> Int<N> {
            from_float_impl!(from_f32, f32);
            from_float_impl!(from_f64, f64);
        }
    };
    ($method: ident, $float: ident) => {
        #[inline(always)]
        #[doc = doc::convert::from!($float I 256)]
        pub const fn $method(f: $float) -> Result<Self, ParseError> {
            if f.is_sign_negative() {
                let i = match UInt::$method(-f) {
                    Ok(u) => {
                        Self::from_bits(u)
                    }, 
                    Err(e) => return Err(e),
                };
                
                if i.eq(&Self::MIN) {
                    Ok(Self::MIN)
                } else if i.is_negative() {
                    Err(ParseError::NegOverflow)
                } else {
                    Ok(i.neg())
                }
            } else {
                let i = match UInt::$method(f) {
                    Ok(u) => {
                        Self::from_bits(u)
                    }, 
                    Err(e) => return Err(e),
                };
                
                if i.is_negative() {
                    Err(ParseError::PosOverflow)
                } else {
                    Ok(i)
                }
            }
        }
    };
}

pub(crate) use from_float_impl;

from_float_impl!();