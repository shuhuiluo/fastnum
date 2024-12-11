use crate::decimal::Decimal;

impl<const N: usize> zeroize::DefaultIsZeroes for Decimal<N> {}
