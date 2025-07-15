use crate::bint::{carrying::carrying_impl, doc, UInt};

carrying_impl!(UInt, U);

impl<const N: usize> UInt<N> {
    #[doc = doc::carrying::carrying_mul!(U 256)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn carrying_mul(self, rhs: Self, carry: Self) -> (Self, Self) {
        let (low, high) = self.0.carrying_mul(rhs.0, carry.0);
        (Self(low), Self(high))
    }
}
