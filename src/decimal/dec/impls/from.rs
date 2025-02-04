use crate::decimal::{dec::parse, Decimal};

type D<const N: usize> = Decimal<N>;

macro_rules! from_impl {
    ($n: ident, $ty: ty) => {
        impl<const N: usize> From<$ty> for D<N> {
            #[inline]
            fn from(n: $ty) -> Self {
                parse::$n(n)
            }
        }
    };
}

from_impl!(from_u8, u8);
from_impl!(from_u16, u16);
from_impl!(from_u32, u32);
from_impl!(from_u64, u64);
from_impl!(from_u128, u128);
from_impl!(from_usize, usize);

from_impl!(from_i8, i8);
from_impl!(from_i16, i16);
from_impl!(from_i32, i32);
from_impl!(from_i64, i64);
from_impl!(from_i128, i128);
from_impl!(from_isize, isize);

from_impl!(from_f32, f32);
from_impl!(from_f64, f64);
