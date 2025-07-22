use crate::bint::{doc, intrinsics::ExpType, wrapping::wrapping_impl, Int, UInt};

wrapping_impl!(UInt, U);

#[doc = doc::wrapping::impl_desc!()]
impl<const N: usize> UInt<N> {
    #[doc = doc::wrapping::wrapping_add_signed!(U 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn wrapping_add_signed(self, rhs: Int<N>) -> Self {
        Self(self.0.wrapping_add_signed(rhs.0))
    }

    #[doc = doc::wrapping::wrapping_next_power_of_two!(U 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn wrapping_next_power_of_two(self) -> Self {
        Self(self.0.wrapping_next_power_of_two())
    }

    #[doc = doc::wrapping::wrapping_mul_digit!(U 256)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn wrapping_mul_digit(self, rhs: u64) -> Self {
        self.overflowing_mul_digit(rhs).0
    }
}
