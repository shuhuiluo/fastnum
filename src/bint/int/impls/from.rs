use crate::bint::{
    impls::from::{from_impl, try_from_impl},
    Int, ParseError,
};

from_impl!(
    Int, I

    from_i8 i8,
    from_i16 i16,
    from_i32 i32,
    from_i64 i64,
    from_isize isize,

    from_u8 u8,
    from_u16 u16,
    from_u32 u32
);

try_from_impl!(
    Int, I

    from_u64 u64,
    from_usize usize,
    from_u128 u128,
    from_i128 i128
);
