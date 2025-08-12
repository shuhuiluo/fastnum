use crate::bint::{
    doc,
    overflowing::overflowing_impl,
    uint::{intrinsics::*, math},
    Int, UInt,
};

overflowing_impl!(UInt, U);

#[doc = doc::overflowing::impl_desc!()]
impl<const N: usize> UInt<N> {
    #[doc = doc::overflowing::overflowing_add_signed!(U 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn overflowing_add_signed(self, rhs: Int<N>) -> (Self, bool) {
        let (res, carry) = self.0.overflowing_add_signed(rhs.0);
        (Self(res), carry)
    }

    #[doc = doc::overflowing::overflowing_add!(U 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn overflowing_add_digit(self, rhs: u64) -> (Self, bool) {
        math::add::add_digit(self, rhs)
    }

    #[doc = doc::overflowing::overflowing_mul!(U 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
        math::mul::overflowing_mul(self, rhs)
    }

    #[doc = doc::overflowing::overflowing_mul_digit!(U 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn overflowing_mul_digit(self, rhs: u64) -> (Self, bool) {
        math::mul::overflowing_mul_digit(self, rhs)
    }
}
