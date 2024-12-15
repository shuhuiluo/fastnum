mod rounding_mode;
mod signal_traps;

pub use rounding_mode::RoundingMode;
pub use signal_traps::SignalsTraps;

use crate::decimal::Signal;

/// # Decimal Context
///
/// The context represents the user-selectable parameters and rules which govern
/// the results of arithmetic operations (for example, the rounding mode when
/// rounding occurs).
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Context {
    rounding_mode: RoundingMode,
    signal_traps: SignalsTraps,
}

impl Context {
    const DEFAULT: Self = Self {
        rounding_mode: RoundingMode::default(),
        signal_traps: SignalsTraps::default(),
    };

    /// Returns the [Default Decimal Context](#crate::default-decimal-context).
    #[inline]
    pub const fn default() -> Self {
        Self::DEFAULT
    }

    /// Apply the given [RoundingMode] to the `Context`.
    #[must_use]
    #[inline]
    pub const fn with_rounding_mode(mut self, rounding_mode: RoundingMode) -> Self {
        self.rounding_mode = rounding_mode;
        self
    }

    /// Method applies specified [SignalsTraps] to the given context.
    /// # Examples
    ///
    /// ```
    /// use fastnum::{dec256, decimal::{Context, SignalsTraps}};
    ///
    /// let traps = SignalsTraps::empty();
    /// let ctx = Context::default().with_signal_traps(traps);
    ///
    /// // No panic! We can divide by zero!
    /// let res = dec256!(1.0).div(dec256!(0), ctx);
    ///
    /// assert!(res.is_infinite());
    /// assert!(res.is_op_div_by_zero());
    /// assert!(res.is_op_invalid());
    /// ```
    #[must_use]
    #[inline]
    pub const fn with_signal_traps(mut self, traps: SignalsTraps) -> Self {
        self.signal_traps = traps;
        self
    }

    /// Get [RoundingMode] of given `Context`.
    #[must_use]
    #[inline]
    pub const fn rounding_mode(&self) -> RoundingMode {
        self.rounding_mode
    }

    #[allow(dead_code)]
    #[inline]
    pub(crate) const fn trap_signals(&self, signals: Signal) {
        self.signal_traps.trap(signals);
    }
}
