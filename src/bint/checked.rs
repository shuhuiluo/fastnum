macro_rules! checked_impl {
    ($Ty: ident, $sign: ident) => {
        #[doc = doc::checked::impl_desc!()]
        impl<const N: usize> $Ty<N> {
            #[doc = doc::checked::checked_add!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn checked_add(self, rhs: Self) -> Option<Self> {
                tuple_to_option(self.overflowing_add(rhs))
            }

            #[doc = doc::checked::checked_sub!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
                tuple_to_option(self.overflowing_sub(rhs))
            }

            #[doc = doc::checked::checked_div!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn checked_div(self, rhs: Self) -> Option<Self> {
                tuple_to_option(self.overflowing_div(rhs))
            }

            #[doc = doc::checked::checked_div_euclid!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn checked_div_euclid(self, rhs: Self) -> Option<Self> {
                tuple_to_option(self.overflowing_div_euclid(rhs))
            }

            #[doc = doc::checked::checked_rem!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
                tuple_to_option(self.overflowing_rem(rhs))
            }

            #[doc = doc::checked::checked_rem_euclid!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn checked_rem_euclid(self, rhs: Self) -> Option<Self> {
                tuple_to_option(self.overflowing_rem_euclid(rhs))
            }

            #[doc = doc::checked::checked_shl!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn checked_shl(self, rhs: ExpType) -> Option<Self> {
                tuple_to_option(self.overflowing_shl(rhs))
            }

            #[doc = doc::checked::checked_shr!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn checked_shr(self, rhs: ExpType) -> Option<Self> {
                tuple_to_option(self.overflowing_shr(rhs))
            }

            #[doc = doc::checked::checked_pow!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn checked_pow(self, pow: ExpType) -> Option<Self> {
                tuple_to_option(self.overflowing_pow(pow))
            }

            #[doc = doc::checked::checked_next_multiple_of!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn checked_next_multiple_of(self, rhs: Self) -> Option<Self> {
                match self.0.checked_next_multiple_of(rhs.0) {
                    Some(value) => Some(Self(value)),
                    None => None,
                }
            }

            #[doc = doc::checked::checked_neg!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn checked_neg(self) -> Option<Self> {
                tuple_to_option(self.overflowing_neg())
            }
        }
    };
}

pub(crate) use checked_impl;
