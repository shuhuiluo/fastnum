pub use crate::bint::intrinsics::*;
use crate::bint::{Int, UInt};

// struct IntHelper<const N: usize>;
//
// impl<const N: usize> IntHelper<N> {
//     const MAX_INT: bnum::BUint<N> = bnum::BInt::MAX.to_bits();
//     const MAX_INT_PLUS_1 : bnum::BUint<N> =
// Self::MAX_INT.strict_add(bnum::BUint::ONE); }

pub struct Intrinsics<const N: usize>;

impl<const N: usize> Intrinsics<N> {
    pub const MAX_INT_AS_UINT: UInt<N> = Int::<N>::MAX.to_bits();
}
