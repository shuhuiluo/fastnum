use crate::bint::Int;

impl<const N: usize> zeroize::DefaultIsZeroes for Int<N> {}
