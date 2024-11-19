use core::ops::Neg;

use crate::decimal::signed::Decimal;

impl<const N: usize> Neg for Decimal<N> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        self.neg()
    }
}

