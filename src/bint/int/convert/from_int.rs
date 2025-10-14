macro_rules! from_int_impl {
    ($($from_int:ident <- $int:ident),*) => {
        $(
            #[inline(always)]
            #[doc = doc::convert::from!($int I 256)]
            pub const fn $from_int(int: $int) -> Self {
                debug_assert!($int::BITS <= Self::BITS);
                let digits = $crate::bint::convert::utils::digits_from_int_impl!(int, $int, $int::BITS, WRAP);
                Self::from_bits(UInt::from_digits(digits))
            }
        )*
    };
}

pub(crate) use from_int_impl;

macro_rules! try_from_int_impl {
    ($($from_int:ident <- $int:ident),*) => {
        $(
            #[inline(always)]
            #[doc = doc::convert::from!($int I 256)]
            pub const fn $from_int(int: $int) -> Result<Self, ParseError> {
                let bits = $int::BITS - int.leading_zeros();

                if bits > Self::BITS {
                     return Err(ParseError::PosOverflow);
                }

                let digits = $crate::bint::convert::utils::digits_from_int_impl!(int, $int, bits, WRAP);

                Ok(Self(bnum::BInt::from_bits(bnum::BUint::from_digits(digits))))
            }
        )*
    };
}

pub(crate) use try_from_int_impl;
