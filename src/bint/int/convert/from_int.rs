macro_rules! from_int_impl {
    ($($name:ident <- $int:ident ($from_uint:ident <- $uint:ident)),*) => {
        impl<const N: usize> Int<N> {
            $(
                #[inline(always)]
                #[doc = doc::convert::from!($int I 256)]
                pub const fn $name(int: $int) -> Self {
                    debug_assert!($int::BITS <= Self::BITS);
                    Self::from_bits(UInt::$from_uint(int.cast_unsigned()))
                }
            )*
        }
    };
}

pub(crate) use from_int_impl;

macro_rules! try_from_int_impl {
    ($($from_int:ident <- $int:ident),*) => {
        impl<const N: usize> Int<N> {
            $(
                #[inline(always)]
                #[doc = doc::convert::from!($int I 256)]
                pub const fn $from_int(int: $int) -> Result<Self, ParseError> {
                    if $int::BITS - int.leading_zeros() > Self::BITS {
                         return Err(ParseError::PosOverflow);
                    } else {
                        let mut digits = if int.is_negative() {
                            *(UInt::MAX.digits())
                        } else {
                            *(UInt::ZERO.digits())
                        };
                        let mut i = 0;
                        while i << intrinsics::BIT_SHIFT < $int::BITS as usize {
                            let d = (int >> (i << intrinsics::BIT_SHIFT)) as intrinsics::Digit;
                            digits[i] = d;
                            i += 1;
                        }
                        
                        Ok(Self(bnum::BInt::from_bits(bnum::BUint::from_digits(digits))))
                    }
                }
            )*
        }
    };
}

pub(crate) use try_from_int_impl;