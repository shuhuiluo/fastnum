macro_rules! saturating_impl {
    ($Ty: ident, $sign: ident) => {
        #[doc = doc::saturating::impl_desc!()]
        impl<const N: usize> $Ty<N> {
            #[doc = doc::saturating::saturating_add!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn saturating_add(self, rhs: Self) -> Self {
                Self(self.0.saturating_add(rhs.0))
            }

            #[doc = doc::saturating::saturating_sub!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn saturating_sub(self, rhs: Self) -> Self {
                Self(self.0.saturating_sub(rhs.0))
            }

            #[doc = doc::saturating::saturating_mul!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn saturating_mul(self, rhs: Self) -> Self {
                Self(self.0.saturating_mul(rhs.0))
            }

            #[doc = doc::saturating::saturating_div!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn saturating_div(self, rhs: Self) -> Self {
                Self(self.0.saturating_div(rhs.0))
            }

            #[doc = doc::saturating::saturating_pow!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn saturating_pow(self, exp: ExpType) -> Self {
                Self(self.0.saturating_pow(exp))
            }
        }
    };
}

pub(crate) use saturating_impl;
