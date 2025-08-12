macro_rules! overflowing_impl {
    ($Ty: ident, $sign: ident) => {
        #[doc = doc::overflowing::impl_desc!()]
        impl<const N: usize> $Ty<N> {
            #[doc = doc::overflowing::overflowing_add!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn overflowing_add(self, rhs: Self) -> (Self, bool) {
                let (res, carry) = self.0.overflowing_add(rhs.0);
                (Self(res), carry)
            }

            #[doc = doc::overflowing::overflowing_sub!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
                let (res, carry) = self.0.overflowing_sub(rhs.0);
                (Self(res), carry)
            }

            #[doc = doc::overflowing::overflowing_div!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn overflowing_div(self, rhs: Self) -> (Self, bool) {
                let (res, carry) = self.0.overflowing_div(rhs.0);
                (Self(res), carry)
            }

            #[doc = doc::overflowing::overflowing_div_euclid!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool) {
                let (res, carry) = self.0.overflowing_div_euclid(rhs.0);
                (Self(res), carry)
            }

            #[doc = doc::overflowing::overflowing_rem!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
                let (res, carry) = self.0.overflowing_rem(rhs.0);
                (Self(res), carry)
            }

            #[doc = doc::overflowing::overflowing_rem_euclid!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool) {
                let (res, carry) = self.0.overflowing_rem_euclid(rhs.0);
                (Self(res), carry)
            }

            #[doc = doc::overflowing::overflowing_shl!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn overflowing_shl(self, rhs: ExpType) -> (Self, bool) {
                let (res, carry) = self.0.overflowing_shl(rhs);
                (Self(res), carry)
            }

            #[doc = doc::overflowing::overflowing_shr!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn overflowing_shr(self, rhs: ExpType) -> (Self, bool) {
                let (res, carry) = self.0.overflowing_shr(rhs);
                (Self(res), carry)
            }

            #[doc = doc::overflowing::overflowing_pow!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn overflowing_pow(self, pow: ExpType) -> (Self, bool) {
                let (res, carry) = self.0.overflowing_pow(pow);
                (Self(res), carry)
            }

            #[doc = doc::overflowing::overflowing_neg!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn overflowing_neg(self) -> (Self, bool) {
                let (res, carry) = self.0.overflowing_neg();
                (Self(res), carry)
            }
        }
    };
}

pub(crate) use overflowing_impl;
