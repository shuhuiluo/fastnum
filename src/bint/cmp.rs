macro_rules! cmp_impl {
    ($Ty: ident, $sign: ident) => {
        impl<const N: usize> $Ty<N> {
            #[doc = doc::cmp::eq!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn eq(&self, other: &Self) -> bool {
                self.0.eq(&other.0)
            }

            #[doc = doc::cmp::ne!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn ne(&self, other: &Self) -> bool {
                self.0.ne(&other.0)
            }

            #[doc = doc::cmp::is_zero!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn is_zero(&self) -> bool {
                self.0.is_zero()
            }

            #[doc = doc::cmp::is_one!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn is_one(&self) -> bool {
                self.0.is_one()
            }

            #[doc = doc::cmp::cmp!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.0.cmp(&other.0)
            }

            #[doc = doc::cmp::max!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn max(self, other: Self) -> Self {
                Self(self.0.max(other.0))
            }

            #[doc = doc::cmp::min!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn min(self, other: Self) -> Self {
                Self(self.0.min(other.0))
            }

            #[doc = doc::cmp::clamp!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn clamp(self, min: Self, max: Self) -> Self {
                Self(self.0.clamp(min.0, max.0))
            }

            #[doc = doc::cmp::lt!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn lt(&self, other: &Self) -> bool {
                self.0.lt(&other.0)
            }

            #[doc = doc::cmp::le!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn le(&self, other: &Self) -> bool {
                self.0.le(&other.0)
            }

            #[doc = doc::cmp::gt!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn gt(&self, other: &Self) -> bool {
                self.0.gt(&other.0)
            }

            #[doc = doc::cmp::ge!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn ge(&self, other: &Self) -> bool {
                self.0.ge(&other.0)
            }
        }
    };
}

pub(crate) use cmp_impl;
