use crate::bint::{doc, int::intrinsics::*, strict::strict_impl, Int, UInt};

strict_impl!(Int, I);

#[doc = doc::strict::impl_desc!()]
impl<const N: usize> Int<N> {
    #[doc = doc::strict::strict_abs!(I 256)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn strict_abs(self) -> Self {
        Self(self.0.strict_abs())
    }

    #[doc = doc::strict::strict_add_unsigned!(I 256)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn strict_add_unsigned(self, rhs: UInt<N>) -> Self {
        Self(self.0.strict_add_unsigned(rhs.0))
    }

    #[doc = doc::strict::strict_sub_unsigned!(I 256)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn strict_sub_unsigned(self, rhs: UInt<N>) -> Self {
        Self(self.0.strict_sub_unsigned(rhs.0))
    }
}
