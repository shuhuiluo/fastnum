use core::cmp::Ordering;

use crate::decimal::signed::Decimal;

impl<const N: usize> PartialOrd for Decimal<N> {
    #[inline]
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(std::cmp::Ord::cmp(self, rhs))
    }
}

impl<const N: usize> Ord for Decimal<N> {
    #[inline]
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.cmp(rhs)
    }
}
