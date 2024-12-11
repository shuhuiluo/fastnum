use core::hash::{Hash, Hasher};

use crate::decimal::UnsignedDecimal;

impl<const N: usize> Hash for UnsignedDecimal<N> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
