use crate::bint::Int;

impl<const N: usize> Default for Int<N> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}
