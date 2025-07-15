use crate::bint::{
    checked::checked_impl, doc, intrinsics::ExpType, uint::math, utils::tuple_to_option, Int, UInt,
};
use crate::bint::uint::intrinsics::Intrinsics;

checked_impl!(UInt, U);

impl<const N: usize> UInt<N> {
    #[doc = doc::checked::checked_add_signed!(U 256)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_add_signed(self, rhs: Int<N>) -> Option<Self> {
        tuple_to_option(self.overflowing_add_signed(rhs))
    }

    #[doc = doc::checked::checked_ilog2!(U 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn checked_ilog2(self) -> Option<ExpType> {
        self.0.checked_ilog2()
    }

    #[doc = doc::checked::checked_ilog10!(U 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn checked_ilog10(self) -> Option<ExpType> {
        if self.is_zero() {
            return None;
        }

        Some(math::ilog::ilog10(self))
    }

    #[doc = doc::checked::checked_ilog!(U 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn checked_ilog(self, base: Self) -> Option<ExpType> {
        self.0.checked_ilog(base.0)
    }

    #[doc = doc::checked::checked_next_power_of_two!(U 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn checked_next_power_of_two(self) -> Option<Self> {
        match self.0.checked_next_power_of_two() {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    #[doc = doc::checked::checked_power_of_ten!(U 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn checked_power_of_ten(power: ExpType) -> Option<Self> {
        if power > Intrinsics::<N>::MAX_POWER_OF_TEN {
            None
        } else {
            Some(Intrinsics::<N>::POWERS_OF_TEN.lookup(power))
        }
    }
}
