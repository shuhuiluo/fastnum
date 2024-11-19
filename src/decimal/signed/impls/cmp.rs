use crate::decimal::{signed::Decimal};

impl<const N: usize> PartialEq for Decimal<N>
{
    #[inline]
    fn eq(&self, rhs: &Decimal<N>) -> bool {
        self.eq(rhs)
    }
}

impl<const N: usize> Eq for Decimal<N> {}
