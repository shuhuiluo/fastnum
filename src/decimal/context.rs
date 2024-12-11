mod rounding_mode;
mod signal_traps;

pub use rounding_mode::RoundingMode;
pub use signal_traps::SignalsTraps;

use crate::decimal::{doc, Signal};

/// # Decimal Context
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

    #[inline]
    pub const fn default() -> Self {
        Self::DEFAULT
    }

    #[inline]
    pub const fn with_rounding_mode(mut self, rounding_mode: RoundingMode) -> Self {
        self.rounding_mode = rounding_mode;
        self
    }
    
    ///
    /// Method applies [SignalsTraps] to the given context.
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
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn with_signal_traps(mut self, traps: SignalsTraps) -> Self {
        self.signal_traps = traps;
        self
    }

    #[inline]
    pub const fn rounding_mode(&self) -> RoundingMode {
        self.rounding_mode
    }

    #[inline]
    pub const fn trap_signals(&self, signals: Signal) {
        self.signal_traps.trap(signals);
    }
}
