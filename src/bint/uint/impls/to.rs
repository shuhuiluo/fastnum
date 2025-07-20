use crate::bint::{
    impls::to::{try_to_float_impl, try_to_impl},
    ParseError, UInt,
};

try_to_impl!(
    UInt, U
    to_u8 u8,
    to_u16 u16,
    to_u32 u32,
    to_u64 u64,
    to_u128 u128,
    to_usize usize,

    to_i8 i8,
    to_i16 i16,
    to_i32 i32,
    to_i64 i64,
    to_i128 i128,
    to_isize isize
);

try_to_float_impl!(UInt, U f32, f64);
