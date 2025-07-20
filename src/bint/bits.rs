macro_rules! bits_impl {
    ($Ty: ident, $sign: ident) => {
        impl<const N: usize> $Ty<N> {
            #[doc = doc::bits::bitand!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn bitand(self, rhs: Self) -> Self {
                Self(self.0.bitand(rhs.0))
            }

            #[doc = doc::bits::bitor!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn bitor(self, rhs: Self) -> Self {
                Self(self.0.bitor(rhs.0))
            }

            #[doc = doc::bits::bitxor!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn bitxor(self, rhs: Self) -> Self {
                Self(self.0.bitxor(rhs.0))
            }

            #[doc = doc::bits::not!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn not(self) -> Self {
                Self(self.0.not())
            }
        }
    };
}

pub(crate) use bits_impl;
