macro_rules! to_uint_impl {
    ($($to_uint:ident -> $uint:ident),*) => {
        $(
            impl<const N: usize> Int<N> {
                #[doc = concat!("Converts [Self] into [`prim@", stringify!($uint), "`].")]
                #[inline]
                pub const fn $to_uint(self) -> Result<$uint, ParseError> {
                    if self.is_negative() {
                        Err(ParseError::Signed)
                    } else {
                        UInt(self.0.to_bits()).$to_uint()
                    }
                }
            }
        )*
    };
}

pub(crate) use to_uint_impl;
