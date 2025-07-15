use num_traits::FromPrimitive;

use crate::decimal::{impls::numtraits::from_primitive_impl, Decimal};

from_primitive_impl!(
    Decimal, I,
    from_u8 <- u8,
    from_u16 <- u16,
    from_u32 <- u32,
    from_u64 <- u64 #TRY,
    from_usize <- usize #TRY,
    from_u128 <- u128 #TRY,

    from_i8 <- i8,
    from_i16 <- i16,
    from_i32 <- i32,
    from_i64 <- i64,
    from_isize <- isize,
    from_i128 <- i128 #TRY,

    from_f32 <- f32 #TRY,
    from_f64 <- f64 #TRY
);
