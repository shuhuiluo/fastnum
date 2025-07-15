macro_rules! from_uint_impl {
    ($($name:ident <- $num:ident),*) => {
        impl<const N: usize> UInt<N> {
            $(
                #[inline(always)]
                #[doc = doc::convert::from!($num U 256)]
                pub const fn $name(n: $num) -> Self {
                    debug_assert!($num::BITS <= Self::BITS);
                    Self(bnum::BUint::from_digit(n as _))
                }
            )*
        }
    };
}

pub(crate) use from_uint_impl;

macro_rules! try_from_uint_impl {
    ($($from_uint:ident <- $uint:ident),*) => {
        impl<const N: usize> UInt<N> {
            $(
                #[inline(always)]
                #[doc = doc::convert::from!($uint U 256)]
                pub const fn $from_uint(uint: $uint) -> Result<Self, ParseError> {
                    let uint_bits = $uint::BITS as usize - uint.leading_zeros() as usize;

                    if uint_bits > Self::BITS as usize {
                        return Err(ParseError::PosOverflow);
                    }

                    let mut digits = [0; N];
                    let mut i = 0;
                    while i << BIT_SHIFT < uint_bits {
                        let d = (uint >> (i << BIT_SHIFT)) as Digit;
                        if d != 0 {
                            digits[i] = d;
                        }
                        i += 1;
                    }

                    Ok(Self::from_digits(digits))
                }
            )*
        }
    };
}

pub(crate) use try_from_uint_impl;