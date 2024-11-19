/// OverflowPolicy
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OverflowPolicy {
    /// Panic if overflow occurred.
    Strict,
    /// Saturating value if overflow occurred.
    Saturating,
}

impl OverflowPolicy {
    /// Returns th default `OverflowPolicy`.
    #[inline]
    pub const fn default() -> Self {
        #[cfg(debug_assertions)]
        return Self::Strict;

        #[cfg(not(debug_assertions))]
        Self::Saturating
    }
}
