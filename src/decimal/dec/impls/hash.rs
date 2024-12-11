use core::hash::{Hash, Hasher};

use crate::decimal::{Context, Decimal};

impl<const N: usize> Hash for Decimal<N> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        let normalized = self.normalized(Context::default());
        normalized.digits.hash(state);
        normalized.scale.hash(state);
        normalized.flags.hash(state);
    }
}
