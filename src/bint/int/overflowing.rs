use crate::bint::{doc, intrinsics::*, overflowing::overflowing_impl, Int, UInt};

overflowing_impl!(Int, I);

#[doc = doc::overflowing::impl_desc!()]
impl<const N: usize> Int<N> {
    #[doc = doc::overflowing::overflowing_add_unsigned!(I 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn overflowing_add_unsigned(self, rhs: UInt<N>) -> (Self, bool) {
        let (res, carry) = self.0.overflowing_add_unsigned(rhs.0);
        (Self(res), carry)
    }

    #[doc = doc::overflowing::overflowing_sub_unsigned!(I 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn overflowing_sub_unsigned(self, rhs: UInt<N>) -> (Self, bool) {
        let (res, carry) = self.0.overflowing_sub_unsigned(rhs.0);
        (Self(res), carry)
    }

    #[doc = doc::overflowing::overflowing_abs!(I 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn overflowing_abs(self) -> (Self, bool) {
        let (res, carry) = self.0.overflowing_abs();
        (Self(res), carry)
    }
}
