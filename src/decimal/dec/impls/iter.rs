use core::iter::Sum;

use crate::decimal::Decimal;

impl<const N: usize> Sum for Decimal<N> {
    #[inline]
    fn sum<I: Iterator<Item = Decimal<N>>>(iter: I) -> Decimal<N> {
        iter.fold(Decimal::<N>::ZERO, |a, b| a + b)
    }
}
