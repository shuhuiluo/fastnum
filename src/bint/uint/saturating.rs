use crate::bint::{doc, intrinsics::ExpType, saturating::saturating_impl, Int, UInt};

saturating_impl!(UInt, U);

#[doc = doc::saturating::impl_desc!()]
impl<const N: usize> UInt<N> {
    #[doc = doc::saturating::saturating_add_signed!(U 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn saturating_add_signed(self, rhs: Int<N>) -> Self {
        Self(self.0.saturating_add_signed(rhs.0))
    }

    #[doc = doc::saturating::saturating_neg!(U 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn saturating_neg(self) -> Self {
        Self::ZERO
    }
}
