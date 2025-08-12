macro_rules! to_int_impl {
    ($($to_int:ident -> $int:ident),*) => {
        $(
            impl<const N: usize> Int<N> {
                #[doc = concat!("Converts [Self] into [`prim@", stringify!($int), "`].")]
                #[inline]
                pub const fn $to_int(self) -> Result<$int, ParseError> {
                    use intrinsics::{Digit, DIGIT_BIT_SHIFT, DIGIT_BITS};

                    let neg = self.is_negative();
                    let (mut out, padding) = if neg {
                        (-1, Digit::MAX)
                    } else {
                        (0, Digit::MIN)
                    };
                    let mut i = 0;
                    let u = self.0.to_bits();
                    let digits = u.digits();

                    if DIGIT_BITS > <$int>::BITS {
                        let small = digits[i] as $int;
                        let trunc = small as Digit;
                        if digits[i] != trunc {
                            return Err(ParseError::PosOverflow);
                        }
                        out = small;
                        i = 1;
                    } else {
                        if neg {
                            loop {
                                let shift = i << DIGIT_BIT_SHIFT;
                                if i >= N || shift >= <$int>::BITS as usize {
                                    break;
                                }
                                out &= !(((!digits[i]) as $int) << shift);
                                i += 1;
                            }
                        } else {
                            loop {
                                let shift = i << DIGIT_BIT_SHIFT;
                                if i >= N || shift >= <$int>::BITS as usize {
                                    break;
                                }
                                out |= (digits[i] as $int) << shift;
                                i += 1;
                            }
                        }
                    }

                    while i < N {
                        if digits[i] != padding {
                            return Err(ParseError::PosOverflow);
                        }
                        i += 1;
                    }

                    if out.is_negative() != neg {
                        return Err(ParseError::PosOverflow);
                    }

                    Ok(out)
                }
            }
        )*
    };
}

pub(crate) use to_int_impl;
