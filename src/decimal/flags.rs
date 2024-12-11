use core::fmt::{Debug, Display, Formatter};

use crate::decimal::Signal;

/// Operation flags
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct Flags {
    flags: FlagsInt,
    signals: Signal,
}

impl Flags {
    pub const EMPTY: Self = Self {
        flags: FlagsInt::EMPTY,
        signals: Signal::EMPTY,
    };

    pub const NAN: Self = Self {
        flags: FlagsInt::NAN,
        signals: Signal::EMPTY,
    };
    pub const INFINITY: Self = Self {
        flags: FlagsInt::INFINITY,
        signals: Signal::EMPTY,
    };
    pub const NEG_INFINITY: Self = Self {
        flags: FlagsInt::NEG_INFINITY,
        signals: Signal::EMPTY,
    };
    pub const NEG: Self = Self {
        flags: FlagsInt::SIGN,
        signals: Signal::EMPTY,
    };

    #[inline(always)]
    pub(crate) const fn default() -> Self {
        Self::EMPTY
    }

    #[inline(always)]
    pub const fn mul(mut self, other: Self) -> Self {
        self.flags = self.flags.mul(other.flags);
        self.signals = self.signals.combine(other.signals);
        self
    }

    #[inline(always)]
    pub const fn neg(mut self) -> Self {
        self.flags = self.flags.neg();
        self
    }

    #[inline(always)]
    pub const fn abs(mut self) -> Self {
        self.flags = self.flags.abs();
        self
    }

    #[inline(always)]
    pub const fn raise_signal(mut self, signal: Signal) -> Self {
        self.signals = self.signals.combine(signal);
        self
    }

    #[inline(always)]
    pub(crate) const fn with_signals_from(mut self, other: Self) -> Self {
        self.signals = self.signals.combine(other.signals);
        self
    }

    #[inline(always)]
    pub(crate) const fn with_signals_from_and(mut self, other: Self, signal: Signal) -> Self {
        self.signals = self.signals.combine(other.signals.combine(signal));
        self
    }

    #[inline(always)]
    pub const fn combine(mut self, other: Self) -> Self {
        self.flags = self.flags.combine(other.flags);
        self.signals = self.signals.combine(other.signals);
        self
    }

    #[inline(always)]
    pub const fn signals(&self) -> Signal {
        self.signals
    }

    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.flags.is_empty() && self.signals.is_empty()
    }

    #[inline(always)]
    pub const fn is_negative(&self) -> bool {
        self.flags.is_negative()
    }

    #[inline(always)]
    pub const fn is_nan(&self) -> bool {
        self.flags.is_nan()
    }

    #[inline(always)]
    pub const fn is_infinity(&self) -> bool {
        self.flags.is_infinity()
    }

    #[inline(always)]
    pub const fn is_special(&self) -> bool {
        self.flags.is_special()
    }

    #[inline(always)]
    pub const fn has_signal(&self, signal: Signal) -> bool {
        self.signals.is_raised(signal)
    }

    #[inline(always)]
    pub const fn has_signals(&self) -> bool {
        !self.signals.is_empty()
    }
}

/// Operation flags
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct FlagsInt(u8);

impl FlagsInt {
    pub const EMPTY: Self = Self(0b0000_0000);

    /// Sign bit. More about Sign:
    pub const SIGN: Self = Self(0b0000_0001);

    /// NAN bit.
    pub const NAN: Self = Self(0b0000_0010);

    pub const INFINITY: Self = Self(0b0000_0100);

    pub const NEG_INFINITY: Self = Self(0b0000_0101);

    const MASK_IS_SPECIAL: u8 = Self::NAN.0 | Self::INFINITY.0;

    #[inline(always)]
    pub const fn combine(self, other: Self) -> Self {
        self.set(other)
    }

    #[inline(always)]
    pub const fn set(mut self, other: Self) -> Self {
        self.0 |= other.0;
        self
    }
    #[inline(always)]
    pub const fn unset(mut self, other: Self) -> Self {
        self.0 &= !other.0;
        self
    }
    
    #[inline(always)]
    pub const fn toggle(mut self, other: Self) -> Self {
        self.0 ^= other.0;
        self
    }
    
    #[inline(always)]
    pub const fn neg(self) -> Self {
        self.toggle(Self::SIGN)
    }

    #[inline(always)]
    pub const fn abs(self) -> Self {
        self.unset(Self::SIGN)
    }

    #[inline(always)]
    pub const fn mul(mut self, other: Self) -> Self {
        self.0 ^= other.0 & Self::SIGN.0;
        self
    }

    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.0 == Self::EMPTY.0
    }

    #[inline(always)]
    pub const fn is_negative(&self) -> bool {
        self.0 & Self::SIGN.0 != 0
    }

    #[inline(always)]
    pub const fn is_nan(&self) -> bool {
        self.0 & Self::NAN.0 != 0
    }

    #[inline(always)]
    pub const fn is_infinity(&self) -> bool {
        self.0 & Self::INFINITY.0 != 0
    }

    #[inline(always)]
    pub const fn is_special(&self) -> bool {
        self.0 & Self::MASK_IS_SPECIAL != 0
    }
}

macro_rules! delimiter {
    ($delimiter: ident, $f: ident) => {
        #[allow(unused_assignments)]
        match $delimiter {
            true => {
                write!($f, ", ")?;
            }
            false => {
                $delimiter = true;
            }
        }
    };
}

impl Display for Flags {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let mut delimiter = false;
        if self.is_empty() {
            return Ok(());
        }

        if self.is_nan() {
            write!(f, "NAN")?;
            delimiter = true;
        }

        if self.is_negative() {
            delimiter!(delimiter, f);
            write!(f, "S")?;
        }

        if self.is_infinity() {
            delimiter!(delimiter, f);
            write!(f, "INF")?;
        }
        
        Ok(())
    }
}

impl Debug for Flags {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self)
    }
}
