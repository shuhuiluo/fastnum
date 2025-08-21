macro_rules! to_str_impl {
    ($Ty: ident, $sign: ident, $Int: ident) => {
        #[doc = doc::convert::to_str_radix!($sign 256)]
        #[must_use = doc::must_use_op!()]
        #[inline(always)]
        pub fn to_str_radix(&self, radix: u32) -> String {
            self.0.to_str_radix(radix)
        }
    };
}

pub(crate) use to_str_impl;
