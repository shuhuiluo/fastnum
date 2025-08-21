macro_rules! from_bytes_impl {
    ($Ty: ident, $sign: ident, $Int: ident) => {
        #[doc = doc::convert::parse_bytes!($sign 256)]
        #[must_use = doc::must_use_op!()]
        #[inline(always)]
        pub const fn parse_bytes(buf: &[u8], radix: u32) -> Option<Self> {
            match $Int::parse_bytes(buf, radix) {
                Some(int) => Some(Self(int)),
                None => None,
            }
        }

        #[doc = doc::convert::from_radix_be!($sign 256)]
        #[must_use = doc::must_use_op!()]
        #[inline(always)]
        pub const fn from_radix_be(buf: &[u8], radix: u32) -> Option<Self> {
            match $Int::from_radix_be(buf, radix) {
                Some(int) => Some(Self(int)),
                None => None,
            }
        }

        #[doc = doc::convert::from_radix_le!($sign 256)]
        #[must_use = doc::must_use_op!()]
        #[inline(always)]
        pub const fn from_radix_le(buf: &[u8], radix: u32) -> Option<Self> {
            match $Int::from_radix_le(buf, radix) {
                Some(int) => Some(Self(int)),
                None => None,
            }
        }
    };
}

pub(crate) use from_bytes_impl;
