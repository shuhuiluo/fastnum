macro_rules! strict_impl {
    ($Ty: ident, $sign: ident) => {
        #[doc = doc::strict::impl_desc!()]
        impl<const N: usize> $Ty<N> {
            #[doc = doc::strict::strict_add!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn strict_add(self, rhs: Self) -> Self {
                Self(self.0.strict_add(rhs.0))
            }

            #[doc = doc::strict::strict_sub!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn strict_sub(self, rhs: Self) -> Self {
                Self(self.0.strict_sub(rhs.0))
            }

            #[doc = doc::strict::strict_div!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn strict_div(self, rhs: Self) -> Self {
                Self(self.0.strict_div(rhs.0))
            }

            #[doc = doc::strict::strict_div_euclid!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn strict_div_euclid(self, rhs: Self) -> Self {
                Self(self.0.strict_div_euclid(rhs.0))
            }

            #[doc = doc::strict::strict_rem!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn strict_rem(self, rhs: Self) -> Self {
                Self(self.0.strict_rem(rhs.0))
            }

            #[doc = doc::strict::strict_rem_euclid!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn strict_rem_euclid(self, rhs: Self) -> Self {
                Self(self.0.strict_rem_euclid(rhs.0))
            }

            #[doc = doc::strict::strict_shl!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn strict_shl(self, rhs: ExpType) -> Self {
                Self(self.0.strict_shl(rhs))
            }

            #[doc = doc::strict::strict_shr!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn strict_shr(self, rhs: ExpType) -> Self {
                Self(self.0.strict_shr(rhs))
            }

            #[doc = doc::strict::strict_pow!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn strict_pow(self, exp: ExpType) -> Self {
                Self(self.0.strict_pow(exp))
            }

            #[doc = doc::strict::strict_neg!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn strict_neg(self) -> Self {
                self.checked_neg().expect("attempt to negate with overflow")
            }
        }
    };
}

pub(crate) use strict_impl;
