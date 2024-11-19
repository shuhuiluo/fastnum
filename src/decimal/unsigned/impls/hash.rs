use core::hash::{Hash, Hasher};

use crate::decimal::unsigned::UnsignedDecimal;

impl<const N: usize> Hash for UnsignedDecimal<N> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        let a = self.normalized();
        a.value.hash(state);
        a.scale.hash(state);
    }
}
