use num_integer::{Integer, Roots};
use num_traits::{
    AsPrimitive, Bounded, CheckedAdd, CheckedDiv, CheckedEuclid, CheckedMul, CheckedNeg,
    CheckedRem, CheckedShl, CheckedShr, CheckedSub, Euclid, FromPrimitive, MulAdd, MulAddAssign,
    Num, One, Pow, PrimInt, Saturating, SaturatingAdd, SaturatingMul, SaturatingSub, Signed,
    ToPrimitive, WrappingAdd, WrappingMul, WrappingNeg, WrappingShl, WrappingShr,
    WrappingSub, Zero,
};

use crate::bint::{impls::numtraits::{numtraits_impl, from_primitive_impl}, Int, UInt, ParseError, intrinsics::ExpType};

numtraits_impl!(Int, I);

impl<const N: usize> Signed for Int<N> {
    #[inline]
    fn abs(&self) -> Self {
        Self::abs(*self)
    }

    #[inline]
    fn abs_sub(&self, other: &Self) -> Self {
        if *self <= *other {
            Self::ZERO
        } else {
            *self - *other
        }
    }

    #[inline]
    fn signum(&self) -> Self {
        Self::signum(*self)
    }

    #[inline]
    fn is_positive(&self) -> bool {
        Self::is_positive(*self)
    }

    #[inline]
    fn is_negative(&self) -> bool {
        Self::is_negative(*self)
    }
}

impl<const N: usize, const M: usize> AsPrimitive<UInt<M>> for Int<N> {
    #[inline]
    fn as_(self) -> UInt<M> {
        UInt(bnum::cast::CastFrom::cast_from(self.0))
    }
}

impl<const N: usize, const M: usize> AsPrimitive<Int<M>> for Int<N> {
    #[inline]
    fn as_(self) -> Int<M> {
        Int(bnum::cast::CastFrom::cast_from(self.0))
    }
}

from_primitive_impl!(
    Int, I,
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
