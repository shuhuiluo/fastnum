use crate::{decimal::Signal, utils::err_msg};

/// # SignalsTraps
///
/// `SignalsTraps` is a list of set trap enablers for signals.
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct SignalsTraps(Signal);

impl SignalsTraps {
    const EMPTY: Self = Self(Signal::EMPTY);
    const DEFAULT: Self = Self(Signal::DEFAULT_TRAPS);

    /// Returns the empty list of signal traps.
    #[must_use]
    #[inline(always)]
    pub const fn empty() -> Self {
        Self::EMPTY
    }

    /// Returns the default set of signal traps.
    #[must_use]
    #[inline(always)]
    pub const fn default() -> Self {
        Self::DEFAULT
    }

    /// Adds the signal trap for the given signal.
    #[must_use]
    #[inline(always)]
    pub const fn set(mut self, signal: Signal) -> Self {
        self.0 = self.0.set(signal);
        self
    }

    #[inline]
    pub(crate) const fn trap(&self, signals: Signal) {
        let signaled = self.0.intersect(signals);
        if signaled.is_empty() {
            return;
        }

        if signaled.is_raised(Signal::OP_DIV_BY_ZERO) {
            panic!(err_msg!("division by zero"));
        }

        if signaled.is_raised(Signal::OP_INVALID) {
            panic!(err_msg!("invalid operation"));
        }

        if signaled.is_raised(Signal::OP_OVERFLOW) {
            panic!(err_msg!(
                "overflow was occurred while performing arithmetic operation"
            ));
        }

        if signaled.is_raised(Signal::OP_UNDERFLOW) {
            panic!(err_msg!(
                "underflow was occurred while performing arithmetic operation"
            ));
        }

        if signaled.is_raised(Signal::OP_INEXACT) {
            panic!(err_msg!("result may be inexact"));
        }

        if signaled.is_raised(Signal::OP_ROUNDED) {
            panic!(err_msg!("result is rounded"));
        }

        if signaled.is_raised(Signal::OP_SUBNORMAL) {
            panic!(err_msg!("result is subnormal"));
        }

        if signaled.is_raised(Signal::OP_CLAMPED) {
            panic!(err_msg!("result clamped"));
        }
    }
}
