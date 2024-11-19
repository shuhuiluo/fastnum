use crate::decimal::unsigned::UnsignedDecimal;

impl<const N: usize> PartialEq for UnsignedDecimal<N> {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        self.eq(rhs)
    }
}

impl<const N: usize> Eq for UnsignedDecimal<N> {}
