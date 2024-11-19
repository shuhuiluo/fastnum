use core::fmt::{self, Debug, Display, Formatter, LowerExp, UpperExp};

use crate::decimal::{format, signed::Sign, unsigned::UnsignedDecimal, utils::name::TypeName};

impl<const N: usize> Display for UnsignedDecimal<N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let digits = self.value.to_str_radix(10);
        let scale = self.scale;
        format::format(digits, scale, Sign::NoSign, f)
    }
}

impl<const N: usize> LowerExp for UnsignedDecimal<N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let digits = self.value.to_str_radix(10);
        let scale = self.scale;
        format::format_exponential(digits, scale, Sign::NoSign, f, "e")
    }
}

impl<const N: usize> UpperExp for UnsignedDecimal<N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let digits = self.value.to_str_radix(10);
        let scale = self.scale;
        format::format_exponential(digits, scale, Sign::NoSign, f, "E")
    }
}

impl<const N: usize> Debug for UnsignedDecimal<N>
where
    Self: TypeName,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if f.alternate() {
            write!(
                f,
                "{}(\"{}e{:}\")",
                Self::type_name(),
                self.value,
                -self.scale
            )
        } else {
            write!(
                f,
                "{}(scale={}, digits=[{:?}])",
                Self::type_name(),
                self.scale,
                self.value
            )
        }
    }
}
