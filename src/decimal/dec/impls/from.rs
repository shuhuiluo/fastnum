use crate::{
    decimal::{
        dec::{parse, ControlBlock},
        Decimal, ParseError,
    },
    int::UInt,
};

macro_rules! from_uint {
    ($($uint: tt),*) => {
        $(
            impl<const N: usize> From<$uint> for Decimal<N>
            {
                #[inline]
                fn from(n: $uint) -> Self {
                    Self::new(UInt::from(n), 0, ControlBlock::default())
                }
            }
        )*
    }
}

macro_rules! from_int {
    ($($int: tt),*) => {
        $(
            impl<const N: usize> From<$int> for Decimal<N> {
                #[inline]
                fn from(n: $int) -> Self {
                    let cb =
                    if n.is_negative() {
                        ControlBlock::default().neg()
                    } else {
                        ControlBlock::default()
                    };

                    Self::new(UInt::from(n.unsigned_abs()), 0, cb)
                }
            }
        )*
    }
}

from_uint!(u8, u16, u32, u64, u128, usize);
from_int!(i8, i16, i32, i64, i128, isize);

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
