use crate::bint::{
    impls::from::{from_impl, try_from_impl},
    ParseError, UInt,
};

from_impl!(
    UInt, U
    from_u8 u8,
    from_u16 u16,
    from_u32 u32,
    from_u64 u64,
    from_usize usize
);

try_from_impl!(
    UInt, U

    from_u128 u128,

    from_i8 i8,
    from_i16 i16,
    from_i32 i32,
    from_i64 i64,
    from_i128 i128,
    from_isize isize
);
