use std::fmt::{self, Debug, Display, Formatter, LowerExp, UpperExp};

use crate::decimal::format;
use crate::decimal::signed::Decimal;
use crate::{U128, U256, U512};

macro_rules! macro_impl {
    ($UINT: ident, $bits: literal) => {
        impl Display for Decimal<$UINT> {
            #[inline]
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                let digits = self.significant_digits().to_str_radix(10);
                let scale = self.fractional_digit_count();
                format::format(digits, scale, self.sign, f)
            }
        }

        impl LowerExp for Decimal<$UINT> {
            #[inline]
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                let digits = self.significant_digits().to_str_radix(10);
                let scale = self.fractional_digit_count();
                format::format_exponential(digits, scale, self.sign, f, "e")
            }
        }

        impl UpperExp for Decimal<$UINT> {
            #[inline]
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                let digits = self.significant_digits().to_str_radix(10);
                let scale = self.fractional_digit_count();
                format::format_exponential(digits, scale, self.sign, f, "E")
            }
        }

        impl Debug for Decimal<$UINT> {
            #[inline]
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                if f.alternate() {
                    write!(
                        f,
                        "D{}(\"{}{}e{:}\")",
                        $bits,
                        self.sign,
                        self.significant_digits(),
                        -self.fractional_digit_count()
                    )
                } else {
                    write!(
                        f,
                        "D{}(sign = \"{}\", scale={}, digits=[{:?}])",
                        $bits,
                        self.sign,
                        self.fractional_digit_count(),
                        self.significant_digits()
                    )
                }
            }
        }
    };
}

macro_impl!(U128, 128);
macro_impl!(U256, 256);
macro_impl!(U512, 512);
