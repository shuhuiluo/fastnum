macro_rules! wrapping_impl {
    ($Ty: ident, $sign: ident) => {
        #[doc = doc::saturating::impl_desc!()]
        impl<const N: usize> $Ty<N> {
            #[doc = doc::wrapping::wrapping_add!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn wrapping_add(self, rhs: Self) -> Self {
                Self(self.0.wrapping_add(rhs.0))
            }
            
            #[doc = doc::wrapping::wrapping_sub!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn wrapping_sub(self, rhs: Self) -> Self {
                Self(self.0.wrapping_sub(rhs.0))
            }
            
            #[doc = doc::wrapping::wrapping_mul!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn wrapping_mul(self, rhs: Self) -> Self {
                Self(self.0.wrapping_mul(rhs.0))
            }

            #[doc = doc::wrapping::wrapping_div!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn wrapping_div(self, rhs: Self) -> Self {
                Self(self.0.wrapping_div(rhs.0))
            }

            #[doc = doc::wrapping::wrapping_div_euclid!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn wrapping_div_euclid(self, rhs: Self) -> Self {
                Self(self.0.wrapping_div_euclid(rhs.0))
            }

            #[doc = doc::wrapping::wrapping_rem!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn wrapping_rem(self, rhs: Self) -> Self {
                Self(self.0.wrapping_rem(rhs.0))
            }

            #[doc = doc::wrapping::wrapping_rem_euclid!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn wrapping_rem_euclid(self, rhs: Self) -> Self {
                Self(self.0.wrapping_rem_euclid(rhs.0))
            }
            
            #[doc = doc::wrapping::wrapping_shl!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn wrapping_shl(self, rhs: ExpType) -> Self {
                Self(self.0.wrapping_shl(rhs))
            }

            #[doc = doc::wrapping::wrapping_shr!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn wrapping_shr(self, rhs: ExpType) -> Self {
                Self(self.0.wrapping_shr(rhs))
            }

            #[doc = doc::wrapping::wrapping_pow!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn wrapping_pow(self, pow: ExpType) -> Self {
                Self(self.0.wrapping_pow(pow))
            }
            
            #[doc = doc::wrapping::wrapping_neg!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn wrapping_neg(self) -> Self {
                Self(self.0.wrapping_neg())
            }
        }
    };
}

pub(crate) use wrapping_impl;