use crate::bint::{doc, strict::strict_impl, uint::intrinsics::*, Int, UInt};

strict_impl!(UInt, U);

impl<const N: usize> UInt<N> {
    #[doc = doc::strict::strict_add_signed!(U 256)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn strict_add_signed(self, rhs: Int<N>) -> Self {
        Self(self.0.strict_add_signed(rhs.0))
    }

    #[doc = doc::strict::strict_power_of_ten!(U 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn strict_power_of_ten(power: ExpType) -> Self {
        Self::checked_power_of_ten(power).expect("power of trn is too large")
    }
}
