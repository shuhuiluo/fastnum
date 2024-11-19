use core::fmt::{self, Debug, Display, Formatter, LowerExp, UpperExp};

use crate::decimal::{format, signed::Decimal, utils::name::TypeName};

impl<const N: usize> Display for Decimal<N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let digits = self.decimal_digits().to_str_radix(10);
        let scale = self.fractional_digits_count();
        format::format(digits, scale, self.sign, f)
    }
}

impl<const N: usize> LowerExp for Decimal<N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let digits = self.decimal_digits().to_str_radix(10);
        let scale = self.fractional_digits_count();
        format::format_exponential(digits, scale, self.sign, f, "e")
    }
}

impl<const N: usize> UpperExp for Decimal<N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let digits = self.decimal_digits().to_str_radix(10);
        let scale = self.fractional_digits_count();
        format::format_exponential(digits, scale, self.sign, f, "E")
    }
}

impl<const N: usize> Debug for Decimal<N>
where
    Self: TypeName,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if f.alternate() {
            write!(
                f,
                "{}(\"{}{}e{:}\")",
                Self::type_name(),
                self.sign,
                self.decimal_digits(),
                -self.fractional_digits_count()
            )
        } else {
            write!(
                f,
                "{}(sign = \"{}\", scale={}, digits=[{:?}])",
                Self::type_name(),
                self.sign,
                self.fractional_digits_count(),
                self.decimal_digits()
            )
        }
    }
}
