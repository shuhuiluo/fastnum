use crate::decimal::signed::Decimal;

impl<const N: usize> Default for Decimal<N> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}
