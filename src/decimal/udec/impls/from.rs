use crate::decimal::{Decimal, DecimalError, ParseError, UnsignedDecimal};

type D<const N: usize> = Decimal<N>;
type UD<const N: usize> = UnsignedDecimal<N>;

impl<const N: usize> TryFrom<D<N>> for UD<N> {
    type Error = DecimalError;

    #[inline]
    fn try_from(d: D<N>) -> Result<Self, Self::Error> {
        if d.is_negative() {
            return Err(DecimalError::Invalid);
        }
        Ok(Self::new(d))
    }
}

impl<const N: usize> From<UD<N>> for D<N> {
    #[inline]
    fn from(d: UD<N>) -> Self {
        d.0
    }
}

macro_rules! from_uint {
    ($($uint: tt),*) => {
        $(
            impl<const N: usize> From<$uint> for UD<N> {
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
            impl<const N: usize> TryFrom<$int> for UD<N> {
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

impl<const N: usize> TryFrom<f32> for UD<N> {
    type Error = ParseError;

    #[inline]
    fn try_from(n: f32) -> Result<Self, Self::Error> {
        if n.is_sign_negative() {
            return Err(ParseError::Signed);
        }
        Ok(Self::new(Decimal::from(n)))
    }
}

impl<const N: usize> TryFrom<f64> for UD<N> {
    type Error = ParseError;

    #[inline]
    fn try_from(n: f64) -> Result<Self, Self::Error> {
        if n.is_sign_negative() {
            return Err(ParseError::Signed);
        }
        Ok(Self::new(Decimal::from(n)))
    }
}
