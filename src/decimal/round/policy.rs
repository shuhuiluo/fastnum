/// Rounding policy
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoundingPolicy {
    /// No rounding. Panic if performing an operation follows a loss of
    /// accuracy.
    Strict,
    /// Rounded result can be used as the result of the operation, may be
    /// inexact.
    Round,
}

impl RoundingPolicy {
    /// Returns default `Rounding policy`.
    pub const fn default() -> Self {
        Self::Round
    }
}
