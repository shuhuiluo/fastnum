use crate::{
    decimal::{
        unsigned::{parse, UnsignedDecimal},
        ParseError,
    },
    int::UInt,
};

macro_rules! from_uint {
    ($($uint: tt),*) => {
        $(
            impl<const N: usize> From<$uint> for UnsignedDecimal<N>
            where
                UInt<N>: From<$uint>
            {
                #[inline]
                fn from(int: $uint) -> Self {
                    Self::new(UInt::from(int), 0)
                }
            }
        )*
    }
}

macro_rules! try_from_int {
    ($($int: tt as $uint: tt),*) => {
        $(
            impl<const N: usize> TryFrom<$int> for UnsignedDecimal<N>
            where
                UInt<N>: From<$uint>
            {
                type Error = ParseError;

                #[inline]
                fn try_from(int: $int) -> Result<Self, Self::Error> {
                    if int.is_negative() {
                        return Err(ParseError::Signed);
                    }
                    let bits = int as $uint;
                    Ok(Self::new(UInt::from(bits), 0))
                }
            }
        )*
    }
}

from_uint!(u8, u16, u32, u64, u128, usize);
try_from_int!(
    i8 as u8,
    i16 as u16,
    i32 as u32,
    isize as usize,
    i64 as u64,
    i128 as u128
);

impl<const N: usize> TryFrom<f32> for UnsignedDecimal<N> {
    type Error = ParseError;

    #[inline]
    fn try_from(n: f32) -> Result<Self, Self::Error> {
        parse::from_f32(n)
    }
}

impl<const N: usize> TryFrom<f64> for UnsignedDecimal<N> {
    type Error = ParseError;

    #[inline]
    fn try_from(n: f64) -> Result<Self, Self::Error> {
        parse::from_f64(n)
    }
}
