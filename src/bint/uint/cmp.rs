use crate::bint::{cmp::cmp_impl, doc, UInt};

cmp_impl!(UInt, U);

impl<const N: usize> UInt<N> {
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub(crate) const fn is_max(&self) -> bool {
        // TODO
        self.0.eq(&bnum::BUint::MAX)
    }
}
