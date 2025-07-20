use num_integer::{Integer, Roots};
use num_traits::{
    AsPrimitive, Bounded, CheckedAdd, CheckedDiv, CheckedEuclid, CheckedMul, CheckedNeg,
    CheckedRem, CheckedShl, CheckedShr, CheckedSub, Euclid, FromPrimitive, MulAdd, MulAddAssign,
    Num, One, Pow, PrimInt, Saturating, SaturatingAdd, SaturatingMul, SaturatingSub, ToPrimitive,
    Unsigned, WrappingAdd, WrappingMul, WrappingNeg, WrappingShl, WrappingShr, WrappingSub, Zero,
};

use crate::bint::{
    impls::numtraits::{from_primitive_impl, numtraits_impl},
    intrinsics::ExpType,
    Int, ParseError, UInt,
};

numtraits_impl!(UInt, U);

impl<const N: usize> Unsigned for UInt<N> {}

impl<const N: usize, const M: usize> AsPrimitive<UInt<M>> for UInt<N> {
    #[inline]
    fn as_(self) -> UInt<M> {
        UInt(bnum::cast::CastFrom::cast_from(self.0))
    }
}

impl<const N: usize, const M: usize> AsPrimitive<Int<M>> for UInt<N> {
    #[inline]
    fn as_(self) -> Int<M> {
        Int(bnum::cast::CastFrom::cast_from(self.0))
    }
}

from_primitive_impl!(
    UInt, U,
    from_u8 <- u8,
    from_u16 <- u16,
    from_u32 <- u32,
    from_u64 <- u64,
    from_usize <- usize,
    from_u128 <- u128 #TRY,

    from_i8 <- i8 #TRY,
    from_i16 <- i16 #TRY,
    from_i32 <- i32 #TRY,
    from_i64 <- i64 #TRY,
    from_isize <- isize #TRY,
    from_i128 <- i128 #TRY,

    from_f32 <- f32 #TRY,
    from_f64 <- f64 #TRY
);
