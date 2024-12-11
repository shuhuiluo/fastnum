use crate::decimal::UnsignedDecimal;

impl<const N: usize> PartialEq for UnsignedDecimal<N> {
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

impl<const N: usize> Eq for UnsignedDecimal<N> {}
