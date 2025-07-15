use crate::bint::UInt;

impl<const N: usize> zeroize::DefaultIsZeroes for UInt<N> {}
