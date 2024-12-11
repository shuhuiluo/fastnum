use crate::decimal::Decimal;

impl<const N: usize> PartialEq for Decimal<N> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.eq(other)
    }

    #[allow(clippy::partialeq_ne_impl)]
    #[inline]
    fn ne(&self, other: &Self) -> bool {
        self.ne(other)
    }
}

impl<const N: usize> Eq for Decimal<N> {}
