macro_rules! endian_impl {
    ($Ty: ident, $sign: ident, $Int: ident) => {
        #[doc = doc::endian::impl_desc!($Ty, $sign)]
        impl<const N: usize> $Ty<N> {
            #[doc = doc::endian::from_be!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn from_be(x: Self) -> Self {
                Self($Int::from_be(x.0))
            }

            #[doc = doc::endian::from_le!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn from_le(x: Self) -> Self {
                Self($Int::from_le(x.0))
            }

            #[doc = doc::endian::to_be!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn to_be(self) -> Self {
                Self(self.0.to_be())
            }

            #[doc = doc::endian::to_le!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn to_le(self) -> Self {
                Self(self.0.to_le())
            }

            #[doc = doc::endian::from_be_slice!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn from_be_slice(slice: &[u8]) -> Option<Self> {
                match $Int::from_be_slice(slice) {
                    Some(value) => Some(Self(value)),
                    None => None,
                }
            }

            #[doc = doc::endian::from_le_slice!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn from_le_slice(slice: &[u8]) -> Option<Self> {
                match $Int::from_le_slice(slice) {
                    Some(value) => Some(Self(value)),
                    None => None,
                }
            }
        }
    };
}

pub(crate) use endian_impl;
