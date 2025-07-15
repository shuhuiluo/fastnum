use crate::bint::{doc, int::math, intrinsics::ExpType, num::num_impl, Int, UInt};

num_impl!(Int, I);

impl<const N: usize> Int<N> {
    #[doc = doc::num::neg!(I 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn neg(self) -> Self {
        Self(self.0.neg())
    }
    
    #[doc = doc::num::from_bits!(256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn from_bits(bits: UInt<N>) -> Self {
        Self(bnum::BInt::from_bits(bits.0))
    }

    #[doc = doc::num::to_bits!(256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn to_bits(self) -> UInt<N> {
        UInt(self.0.to_bits())
    }

    #[doc = doc::num::cast_unsigned!(256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn cast_unsigned(self) -> UInt<N> {
        self.to_bits()
    }

    #[doc = doc::num::unsigned_abs!(256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn unsigned_abs(self) -> UInt<N> {
        UInt(self.0.unsigned_abs())
    }

    #[doc = doc::num::abs!(256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn abs(self) -> Self {
        Self(self.0.abs())
    }

    #[doc = doc::num::signum!(256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn signum(self) -> Self {
        Self(self.0.signum())
    }

    #[doc = doc::num::is_positive!(256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn is_positive(self) -> bool {
        self.0.is_positive()
    }

    #[doc = doc::num::is_negative!(256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn is_negative(self) -> bool {
        self.0.is_negative()
    }

    #[doc = doc::num::abs_diff!(I 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn abs_diff(self, other: Self) -> UInt<N> {
        UInt(self.0.abs_diff(other.0))
    }

    #[doc = doc::num::div_rem!(I 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn div_rem(self, rhs: Self) -> (Self, Self) {
        math::div_rem(self, rhs)
    }
}
