use core::hash::{Hash, Hasher};

use crate::decimal::signed::Decimal;

impl<const N: usize> Hash for Decimal<N> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
        self.sign.hash(state);
    }
}
