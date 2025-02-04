use core::num::IntErrorKind;

use crate::decimal::{dec::convert, Decimal};

type D<const N: usize> = Decimal<N>;

impl<const N: usize> From<D<N>> for f32 {
    #[inline]
    fn from(d: D<N>) -> Self {
        convert::to_f32(d)
    }
}

impl<const N: usize> From<D<N>> for f64 {
    #[inline]
    fn from(d: D<N>) -> Self {
        convert::to_f64(d)
    }
}

macro_rules! try_to_impl {
    ($to_int: ident, $int: ty) => {
        impl<const N: usize> TryFrom<D<N>> for $int {
            type Error = IntErrorKind;

            #[inline]
            fn try_from(d: D<N>) -> Result<Self, Self::Error> {
                convert::$to_int(d)
            }
        }
    };
}

try_to_impl!(to_usize, usize);
try_to_impl!(to_u8, u8);
try_to_impl!(to_u16, u16);
try_to_impl!(to_u32, u32);
try_to_impl!(to_u64, u64);
try_to_impl!(to_u128, u128);

try_to_impl!(to_isize, isize);
try_to_impl!(to_i8, i8);
try_to_impl!(to_i16, i16);
try_to_impl!(to_i32, i32);
try_to_impl!(to_i64, i64);
try_to_impl!(to_i128, i128);
