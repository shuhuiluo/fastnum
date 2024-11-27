use crate::decimal::{
    signed::{parse, Decimal, Sign},
    unsigned::UnsignedDecimal,
    ParseError,
};

macro_rules! from_uint {
    ($($uint: tt),*) => {
        $(
            impl<const N: usize> From<$uint> for Decimal<N>
            {
                #[inline]
                fn from(n: $uint) -> Self {
                    Self::new(UnsignedDecimal::from(n), Sign::NoSign)
                }
            }
        )*
    }
}

macro_rules! from_int {
    ($($int: tt as $uint: tt),*) => {
        $(
            impl<const N: usize> From<$int> for Decimal<N>
            where
                UnsignedDecimal<N>: From<$uint>
            {
                #[inline]
                fn from(n: $int) -> Self {
                    if n.is_negative() {
                        Self::new(UnsignedDecimal::from(n.unsigned_abs()), Sign::Minus)
                    } else {
                        Self::new(UnsignedDecimal::from(n as $uint), Sign::NoSign)
                    }
                }
            }
        )*
    }
}

from_uint!(u8, u16, u32, u64, u128, usize);
from_int!(
    i8 as u8,
    i16 as u16,
    i32 as u32,
    isize as usize,
    i64 as u64,
    i128 as u128
);

impl<const N: usize> TryFrom<f32> for Decimal<N> {
    type Error = ParseError;

    #[inline]
    fn try_from(n: f32) -> Result<Self, Self::Error> {
        parse::from_f32(n)
    }
}

impl<const N: usize> TryFrom<f64> for Decimal<N> {
    type Error = ParseError;

    #[inline]
    fn try_from(n: f64) -> Result<Self, Self::Error> {
        parse::from_f64(n)
    }
}

impl<const N: usize> From<UnsignedDecimal<N>> for Decimal<N>
{
    #[inline]
    fn from(ud: UnsignedDecimal<N>) -> Self {
        Self::from_unsigned(ud)
    }
}