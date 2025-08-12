use crate::bint::{doc, uint::math, widening::widening_impl, UInt};

widening_impl!(UInt, U);

impl<const N: usize> UInt<N> {
    #[doc = doc::widening::widening_mul!(U 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn widening_mul(self, rhs: Self) -> (Self, Self) {
        math::mul::widening_mul(self, rhs)
    }
}
