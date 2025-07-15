use crate::bint::{
    checked::checked_impl, doc, intrinsics::ExpType, utils::tuple_to_option, Int, UInt,
};

checked_impl!(Int, I);

impl<const N: usize> Int<N> {
    #[doc = doc::checked::checked_add_unsigned!(I 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn checked_add_unsigned(self, rhs: UInt<N>) -> Option<Self> {
        tuple_to_option(self.overflowing_add_unsigned(rhs))
    }

    #[doc = doc::checked::checked_sub_unsigned!(I 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn checked_sub_unsigned(self, rhs: UInt<N>) -> Option<Self> {
        tuple_to_option(self.overflowing_sub_unsigned(rhs))
    }

    #[doc = doc::checked::checked_ilog2!(I 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn checked_ilog2(self) -> Option<ExpType> {
        if self.is_negative() {
            None
        } else {
            self.to_bits().checked_ilog2()
        }
    }

    #[doc = doc::checked::checked_ilog10!(I 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn checked_ilog10(self) -> Option<ExpType> {
        if self.is_negative() {
            None
        } else {
            self.to_bits().checked_ilog10()
        }
    }

    #[doc = doc::checked::checked_ilog!(I 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn checked_ilog(self, base: Self) -> Option<ExpType> {
        if base.is_negative() || self.is_negative() {
            None
        } else {
            self.to_bits().checked_ilog(base.to_bits())
        }
    }

    #[doc = doc::checked::checked_abs!(I 256)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_abs(self) -> Option<Self> {
        tuple_to_option(self.overflowing_abs())
    }
}
