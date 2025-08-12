use crate::bint::{Int, UInt};

pub use crate::bint::intrinsics::*;

pub struct Intrinsics<const N: usize>;

impl<const N: usize> Intrinsics<N> {
    pub const MAX_INT_AS_UINT: UInt<N> = Int::<N>::MAX.to_bits();
}
