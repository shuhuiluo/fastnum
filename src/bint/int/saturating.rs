use crate::bint::{doc, intrinsics::ExpType, saturating::saturating_impl, Int, UInt};

saturating_impl!(Int, I);

#[doc = doc::saturating::impl_desc!()]
impl<const N: usize> Int<N> {
    #[doc = doc::saturating::saturating_add_unsigned!(I 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn saturating_add_unsigned(self, rhs: UInt<N>) -> Self {
        Self(self.0.saturating_add_unsigned(rhs.0))
    }

    #[doc = doc::saturating::saturating_sub_unsigned!(I 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn saturating_sub_unsigned(self, rhs: UInt<N>) -> Self {
        Self(self.0.saturating_sub_unsigned(rhs.0))
    }

    #[doc = doc::saturating::saturating_neg!(I 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn saturating_neg(self) -> Self {
        Self(self.0.saturating_neg())
    }

    #[doc = doc::saturating::saturating_abs!(I 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn saturating_abs(self) -> Self {
        Self(self.0.saturating_abs())
    }
}
