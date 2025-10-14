macro_rules! from_uint_impl {
    ($($name:ident <- $num:ident),*) => {
        $(
            #[inline(always)]
            #[doc = doc::convert::from!($num U 256)]
            pub const fn $name(n: $num) -> Self {
                debug_assert!($num::BITS <= Self::BITS);
                Self(bnum::BUint::from_digit(n as _))
            }
        )*
    };
}

pub(crate) use from_uint_impl;

macro_rules! try_from_uint_impl {
    ($($from_uint:ident <- $uint:ident),*) => {
        $(
            #[inline(always)]
            #[doc = doc::convert::from!($uint U 256)]
            pub const fn $from_uint(uint: $uint) -> Result<Self, ParseError> {
                let bits = $uint::BITS as usize - uint.leading_zeros() as usize;

                if bits > Self::BITS as usize {
                    return Err(ParseError::PosOverflow);
                }

                let digits = $crate::bint::convert::utils::digits_from_int_impl!(uint, $uint, bits);

                Ok(Self::from_digits(digits))
            }
        )*
    };
}

pub(crate) use try_from_uint_impl;
