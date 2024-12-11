use crate::decimal::{Decimal, ParseError, UnsignedDecimal};

macro_rules! from_uint {
    ($($uint: tt),*) => {
        $(
            impl<const N: usize> From<$uint> for UnsignedDecimal<N> {
                #[inline]
                fn from(int: $uint) -> Self {
                    Self::new(Decimal::from(int))
                }
            }
        )*
    }
}

macro_rules! try_from_int {
    ($($int: tt),*) => {
        $(
            impl<const N: usize> TryFrom<$int> for UnsignedDecimal<N> {
                type Error = ParseError;

                #[inline]
                fn try_from(int: $int) -> Result<Self, Self::Error> {
                    if int.is_negative() {
                        return Err(ParseError::Signed);
                    }
                    Ok(Self::new(Decimal::from(int)))
                }
            }
        )*
    }
}

from_uint!(u8, u16, u32, u64, u128, usize);
try_from_int!(i8, i16, i32, i64, i128, isize);

impl<const N: usize> TryFrom<f32> for UnsignedDecimal<N> {
    type Error = ParseError;

    #[inline]
    fn try_from(n: f32) -> Result<Self, Self::Error> {
        if n.is_sign_negative() {
            return Err(ParseError::Signed);
        }
        Ok(Self::new(Decimal::try_from(n)?))
    }
}

impl<const N: usize> TryFrom<f64> for UnsignedDecimal<N> {
    type Error = ParseError;

    #[inline]
    fn try_from(n: f64) -> Result<Self, Self::Error> {
        if n.is_sign_negative() {
            return Err(ParseError::Signed);
        }
        Ok(Self::new(Decimal::try_from(n)?))
    }
}
