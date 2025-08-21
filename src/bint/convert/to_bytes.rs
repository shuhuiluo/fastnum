macro_rules! to_bytes_impl {
    ($Ty: ident, $sign: ident, $Int: ident) => {
        #[doc = doc::convert::to_radix_be!($sign 256)]
        #[must_use = doc::must_use_op!()]
        #[inline(always)]
        pub fn to_radix_be(&self, radix: u32) -> Vec<u8> {
            self.0.to_radix_be(radix)
        }

        #[doc = doc::convert::to_radix_le!($sign 256)]
        #[must_use = doc::must_use_op!()]
        #[inline(always)]
        pub fn to_radix_le(&self, radix: u32) -> Vec<u8> {
            self.0.to_radix_le(radix)
        }
    };
}

pub(crate) use to_bytes_impl;
