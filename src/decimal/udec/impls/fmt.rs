use core::fmt::{self, Debug, Display, Formatter, LowerExp, UpperExp};

use crate::decimal::UnsignedDecimal;

impl<const N: usize> Display for UnsignedDecimal<N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl<const N: usize> LowerExp for UnsignedDecimal<N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        LowerExp::fmt(&self.0, f)
    }
}

impl<const N: usize> UpperExp for UnsignedDecimal<N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        UpperExp::fmt(&self.0, f)
    }
}

impl<const N: usize> Debug for UnsignedDecimal<N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if f.alternate() {
            if self.is_nan() {
                return write!(f, "{}(NaN)", Self::type_name());
            } else if self.is_infinite() {
                return write!(f, "{}(Inf)", Self::type_name(),);
            }

            let alert = if self.flags().has_signals() { "! " } else { "" };
            write!(
                f,
                "{}({}{}e{})",
                Self::type_name(),
                alert,
                self.0.digits(),
                (self.fractional_digits_count() as i32).saturating_neg()
            )
        } else {
            write!(
                f,
                "{}(digits=[{:?}], exp=[{}], flags=[{}], signals=[{}])",
                Self::type_name(),
                self.0.digits(),
                (self.fractional_digits_count() as i32).saturating_neg(),
                self.flags(), 
                self.flags().signals()
            )
        }
    }
}
