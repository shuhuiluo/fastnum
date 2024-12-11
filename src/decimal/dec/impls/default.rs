use crate::decimal::Decimal;

impl<const N: usize> Default for Decimal<N> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}
