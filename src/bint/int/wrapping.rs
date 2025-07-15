use crate::bint::{doc, intrinsics::ExpType, wrapping::wrapping_impl, Int, UInt};

wrapping_impl!(Int, I);

#[doc = doc::wrapping::impl_desc!()]
impl<const N: usize> Int<N> {
    #[doc = doc::wrapping::wrapping_add_unsigned!(I 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn wrapping_add_unsigned(self, rhs: UInt<N>) -> Self {
        Self(self.0.wrapping_add_unsigned(rhs.0))
    }

    #[doc = doc::wrapping::wrapping_sub_unsigned!(I 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn wrapping_sub_unsigned(self, rhs: UInt<N>) -> Self {
        Self(self.0.wrapping_sub_unsigned(rhs.0))
    }

    #[doc = doc::wrapping::wrapping_abs!(I 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn wrapping_abs(self) -> Self {
        Self(self.0.wrapping_abs())
    }
}
