mod from_int;
mod from_str;
mod from_uint;
mod from_float;
mod to_int;
mod to_str;

use from_int::*;
use from_uint::*;
use to_int::*;

use crate::bint::{error::ParseError, intrinsics::*, UInt, doc};

impl<const N: usize> UInt<N> {
    #[inline(always)]
    #[doc = doc::convert::from!(u64 U 256)]
    pub const fn from_u64(n: u64) -> Self {
        Self(bnum::BUint::from_digit(n))
    }
}

from_uint_impl!(
    from_u8 <- u8,
    from_u16 <- u16,
    from_u32 <- u32,
    from_usize <- usize
);

try_from_uint_impl!(
    from_u128 <- u128
);

try_from_int_impl!(
    from_i8 <- i8 (from_u8 <- u8),
    from_i16 <- i16 (from_u16 <- u16),
    from_i32 <- i32 (from_u32 <- u32),
    from_i64 <- i64 (from_u64 <- u64),
    from_isize <- isize (from_usize <- usize),
    from_i128 <- i128 (#TRY from_u128 <- u128)
);

to_int_impl!(
    to_i8 -> i8,
    to_i16 -> i16,
    to_i32 -> i32,
    to_i64 -> i64,
    to_i128 -> i128,
    to_isize -> isize
);

to_int_impl!(
    to_u8 -> u8,
    to_u16 -> u16,
    to_u32 -> u32,
    to_u64 -> u64,
    to_u128 -> u128,
    to_usize -> usize
);
