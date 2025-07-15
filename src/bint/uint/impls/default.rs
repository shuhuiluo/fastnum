use crate::bint::UInt;

impl<const N: usize> Default for UInt<N> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}
