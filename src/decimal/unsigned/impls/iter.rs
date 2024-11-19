use core::iter::Sum;

use crate::decimal::UnsignedDecimal;

impl<const N: usize> Sum for UnsignedDecimal<N> {
    #[inline]
    fn sum<I: Iterator<Item = UnsignedDecimal<N>>>(iter: I) -> UnsignedDecimal<N> {
        iter.fold(UnsignedDecimal::<N>::ZERO, |a, b| a + b)
    }
}
