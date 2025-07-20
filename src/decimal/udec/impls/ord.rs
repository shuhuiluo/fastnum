use core::cmp::Ordering;

use crate::decimal::UnsignedDecimal;

impl<const N: usize> PartialOrd for UnsignedDecimal<N> {
    #[inline]
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(core::cmp::Ord::cmp(self, rhs))
    }
}

impl<const N: usize> Ord for UnsignedDecimal<N> {
    #[inline]
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.cmp(rhs)
    }
}
