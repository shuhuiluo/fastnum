use crate::{
    bint::{doc, strict::strict_impl, uint::intrinsics::*, Int, UInt},
    utils::err_msg,
};

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
        Self::checked_power_of_ten(power).expect(err_msg!("power of ten is too large"))
    }

    #[doc = doc::strict::strict_power_of_five!(U 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn strict_power_of_five(power: ExpType) -> Self {
        Self::checked_power_of_five(power).expect(err_msg!("power of five is too large"))
    }
}
