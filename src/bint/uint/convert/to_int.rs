macro_rules! to_int_impl {
    ($($to_int:ident -> $int:ident),*) => {
        $(
            impl<const N: usize> UInt<N> {
                #[doc = doc::convert::to!($int U 256)]
                #[inline]
                pub const fn $to_int(self) -> Result<$int, ParseError> {
                    let digits = self.digits();
                    let mut out = 0;
                    let mut i = 0;

                    if Digit::BITS > <$int>::BITS {
                        let small = digits[i] as $int;
                        let trunc = small as Digit;
                        if digits[i] != trunc {
                            return Err(ParseError::PosOverflow);
                        }
                        out = small;
                        i = 1;
                    } else {
                        loop {
                            let shift = i << BIT_SHIFT;
                            if i >= N || shift >= <$int>::BITS as usize {
                                break;
                            }
                            out |= (digits[i] as $int) << shift;
                            i += 1;
                        }
                    }

                    #[allow(unused_comparisons)]
                    if out < 0 {
                        return Err(ParseError::PosOverflow);
                    }

                    while i < N {
                        if digits[i] != 0 {
                            return Err(ParseError::PosOverflow);
                        }
                        i += 1;
                    }

                    Ok(out)
                }
            }
        )*
    };
}

pub(crate) use to_int_impl;
