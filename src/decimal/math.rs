mod error;

pub use error::ArithmeticError;

use crate::{
    decimal::{OverflowPolicy, RoundingPolicy},
    utils::err_msg,
};

/// # Arithmetic Policy
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArithmeticPolicy {
    overflow_policy: OverflowPolicy,
    rounding_policy: RoundingPolicy,
}

impl ArithmeticPolicy {
    /// Creates new [`ArithmeticPolicy`](crate#arithmetic-policy) with given
    /// [OverflowPolicy] and [RoundingPolicy].
    #[inline]
    pub const fn new(overflow_policy: OverflowPolicy, rounding_policy: RoundingPolicy) -> Self {
        Self {
            overflow_policy,
            rounding_policy,
        }
    }

    /// Creates new
    /// [`DefaultArithmeticPolicy`](crate#default-arithmetic-policy).
    #[inline]
    pub const fn default() -> ArithmeticPolicy {
        ArithmeticPolicy {
            overflow_policy: OverflowPolicy::default(),
            rounding_policy: RoundingPolicy::default(),
        }
    }
}

use bitflags::bitflags;

bitflags! {
    #[derive(Copy, Clone)]
    pub(crate) struct Flags: u8 {
        const OVERFLOW = 0b00000001;
        const INEXACT = 0b00000010;
        const DIVIDE_BY_ZERO = 0b00000100;
        const NEGATIVE = 0b00001000;
    }
}

impl Flags {
    #[inline]
    pub const fn overflow_to_inexact(mut self) -> Self {
        if self.contains(Flags::OVERFLOW) {
            self = self
                .intersection(Flags::all().difference(Flags::OVERFLOW))
                .union(Flags::INEXACT);
        }
        self
    }
}

/// # Generic Decimal Result
///
/// Wraps the result of any mathematical operation over decimal type `T` with
/// set of emergency flags:
///
/// |  **Flag**        | Description                                                               |
/// |------------------|---------------------------------------------------------------------------|
/// | `OVERFLOW `      | Overflow occurred.                                                        |
/// | `INEXACT`        | Rounding was performed during the operation. The result may not be exact. |
/// | `DIVIDE_BY_ZERO` | Division by zero.                                                         |
/// | `NEGATIVE`       | The negative result cannot be represented by an unsigned type.            |
///
/// `DecimalResult` can be unwrapped into target type `T` with specific or
/// default [ArithmeticPolicy] (see methods [DecimalResult::unwrap] and
/// [DecimalResult::unwrap_with_policy]) or converted into [Option] or [Result].
///
/// For more information about flags and [ArithmeticPolicy] see:
/// [section](crate#arithmetic-result).
#[derive(Copy, Clone)]
pub struct DecimalResult<T> {
    flags: Flags,
    result: T,
}

impl<T: Copy> DecimalResult<T> {
    #[inline]
    pub(crate) const fn new(result: T) -> Self {
        Self {
            flags: Flags::empty(),
            result,
        }
    }

    #[inline]
    pub(crate) const fn split(self) -> (T, Flags) {
        (self.result, self.flags)
    }

    #[inline]
    pub(crate) const fn add_flags(mut self, flags: Flags) -> Self {
        self.flags = self.flags.union(flags);
        self
    }

    #[inline]
    pub(crate) const fn overflow_to_inexact(mut self) -> Self {
        self.flags = self.flags.overflow_to_inexact();
        self
    }

    #[inline]
    pub(crate) const fn negative(mut self) -> Self {
        self.flags = self.flags.union(Flags::NEGATIVE);
        self
    }

    #[inline]
    pub(crate) const fn overflow(mut self) -> Self {
        self.flags = self.flags.union(Flags::OVERFLOW);
        self
    }

    #[inline]
    pub(crate) const fn inexact(mut self) -> Self {
        self.flags = self.flags.union(Flags::INEXACT);
        self
    }

    #[inline]
    pub(crate) const fn div_by_zero(mut self) -> Self {
        self.flags = self.flags.union(Flags::DIVIDE_BY_ZERO);
        self
    }

    /// Returns `true` if the result is negative for unsigned target type.
    ///
    /// # Examples:
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::udec256;
    /// use fastnum::decimal::RoundingMode;
    ///
    /// let a = udec256!(1);
    /// let b = udec256!(2);
    ///
    /// let res = a.sub(b, RoundingMode::default());
    /// assert!(res.is_negative());
    /// ```
    ///
    /// /// For more information about flags and [ArithmeticPolicy] see:
    /// [section](crate#arithmetic-result).
    #[inline]
    pub const fn is_negative(&self) -> bool {
        self.flags.contains(Flags::NEGATIVE)
    }

    /// Returns `true` if overflow occurred during operation perform.
    ///
    /// # Examples:
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::UD256;
    /// use fastnum::decimal::RoundingMode;
    ///
    /// let a = UD256::MAX;
    /// let b = UD256::MAX;
    ///
    /// let res = a.mul(b, RoundingMode::default());
    /// assert!(res.is_overflow());
    /// ```
    ///
    /// /// For more information about flags and [ArithmeticPolicy] see:
    /// [section](crate#arithmetic-result).
    #[inline]
    pub const fn is_overflow(&self) -> bool {
        self.flags.contains(Flags::OVERFLOW)
    }

    /// Returns `true` if the result may be inexact.
    ///
    /// # Examples:
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::udec256;
    /// use fastnum::decimal::RoundingMode;
    ///
    /// let a = udec256!(1);
    /// let b = udec256!(3);
    ///
    /// let res = a.div(b, RoundingMode::default());
    /// assert!(res.is_inexact());
    /// ```
    ///
    /// /// For more information about flags and [ArithmeticPolicy] see:
    /// [section](crate#arithmetic-result).
    #[inline]
    pub const fn is_inexact(&self) -> bool {
        self.flags.contains(Flags::INEXACT)
    }

    /// Returns `true` if division by zero occurred during operation perform.
    ///
    /// # Examples:
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::udec256;
    /// use fastnum::decimal::RoundingMode;
    ///
    /// let a = udec256!(1);
    /// let b = udec256!(0);
    ///
    /// let res = a.div(b, RoundingMode::default());
    /// assert!(res.is_div_by_zero());
    /// ```
    ///
    /// /// For more information about flags and [ArithmeticPolicy] see:
    /// [section](crate#arithmetic-result).
    #[inline]
    pub const fn is_div_by_zero(&self) -> bool {
        self.flags.contains(Flags::DIVIDE_BY_ZERO)
    }

    /// Unwrap the `DecimalResult` into target type `T` using default
    /// [ArithmeticPolicy].
    ///
    /// This method is used in all [Rust operators
    /// overloads](crate#rust-operators-overloads) for decimal types.
    ///
    /// # Panics:
    ///
    /// This method will panic if operation performs with some emergency flags
    /// and default [ArithmeticPolicy] enjoin to panic when the corresponding
    /// flag occurs.
    ///
    /// For more information about flags and [ArithmeticPolicy] see:
    /// [section](crate#arithmetic-result).
    ///
    /// # Examples:
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::{udec256, UD256};
    /// use fastnum::decimal::RoundingMode;
    ///
    /// let a = UD256::ONE;
    /// let b = UD256::TWO;
    ///
    /// let c = a.add(b, RoundingMode::default()).unwrap();
    /// assert_eq!(c, udec256!(3));
    /// ```
    ///
    /// Should panic with default [ArithmeticPolicy]:
    ///
    /// ```should_panic
    /// use fastnum::{udec256, UD256};
    /// use fastnum::decimal::RoundingMode;
    ///
    /// let a = UD256::MAX;
    /// let b = UD256::MAX;
    ///
    /// let c = a.add(b, RoundingMode::default()).unwrap();
    /// ```
    #[track_caller]
    #[inline]
    pub const fn unwrap(self) -> T {
        const POLICY: ArithmeticPolicy = ArithmeticPolicy::default();
        self.unwrap_with_policy(&POLICY)
    }

    /// Unwrap the `DecimalResult` into target type `T` using specified
    /// [ArithmeticPolicy].
    ///
    /// # Panics:
    ///
    /// This method will panic if operation performs with some emergency flags
    /// and specified [ArithmeticPolicy] enjoin to panic when the corresponding
    /// flag occurs.
    ///
    /// For more information about flags and [ArithmeticPolicy] see:
    /// [section](crate#arithmetic-result).
    ///
    /// # Examples:
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::{udec256, UD256};
    /// use fastnum::decimal::{ArithmeticPolicy, OverflowPolicy, RoundingMode, RoundingPolicy};
    ///
    /// let a = UD256::ONE;
    /// let b = UD256::TWO;
    ///
    /// let policy = ArithmeticPolicy::new(OverflowPolicy::Strict, RoundingPolicy::Strict);
    ///
    /// let c = a.add(b, RoundingMode::default()).unwrap_with_policy(&policy);
    /// assert_eq!(c, udec256!(3));
    /// ```
    ///
    /// Saturate if overflowed:
    ///
    /// ```
    /// use fastnum::{udec256, UD256};
    /// use fastnum::decimal::{ArithmeticPolicy, OverflowPolicy, RoundingMode, RoundingPolicy};
    ///
    /// let a = UD256::MAX;
    /// let b = UD256::MAX;
    ///
    /// let policy = ArithmeticPolicy::new(OverflowPolicy::Saturating, RoundingPolicy::Strict);
    ///
    /// let c = a.add(b, RoundingMode::default()).unwrap_with_policy(&policy);
    /// assert_eq!(c, udec256!(115792089237316195423570985008687907853269984665640564039457584007913129639934e9223372036854775808));
    /// ```
    ///
    /// Should panic:
    ///
    /// ```should_panic
    /// use fastnum::{udec256, UD256};
    /// use fastnum::decimal::{ArithmeticPolicy, OverflowPolicy, RoundingMode, RoundingPolicy};
    ///
    /// let a = udec256!(1);
    /// let b = udec256!(3);
    ///
    /// let policy = ArithmeticPolicy::new(OverflowPolicy::Saturating, RoundingPolicy::Strict);
    ///
    /// let c = a.div(b, RoundingMode::default()).unwrap_with_policy(&policy);
    /// ```
    #[track_caller]
    #[inline]
    pub const fn unwrap_with_policy(self, policy: &ArithmeticPolicy) -> T {
        match self.ok_or_err_with_policy(policy) {
            Ok(value) => value,
            Err(e) => match e {
                ArithmeticError::DivideByZero => {
                    panic!(err_msg!("division by zero"));
                }
                ArithmeticError::Overflow => {
                    panic!(err_msg!("attempt to perform the operation with overflow"));
                }
                ArithmeticError::Inexact => {
                    panic!(err_msg!(
                        "not precise rounding is denied by default arithmetic policy"
                    ));
                }
                ArithmeticError::Signed => {
                    panic!(err_msg!("operation has negative result for unsigned type"));
                }
            },
        }
    }

    /// Converts the `DecimalResult` into [`Option<T>`].
    /// Returns
    /// - `Some(T)` if none of the emergency flags are set.
    /// - `None` if at least one emergency flag is set.
    #[inline]
    pub const fn ok(self) -> Option<T> {
        const POLICY: ArithmeticPolicy = ArithmeticPolicy::default();
        self.ok_with_policy(&POLICY)
    }

    /// Converts the `DecimalResult` into [`Option<T>`] using specified
    /// [ArithmeticPolicy].
    ///
    /// Returns:
    ///
    /// - `Some(T)` if result .
    /// - `None` if at least one emergency flag is set.
    #[inline]
    pub const fn ok_with_policy(self, policy: &ArithmeticPolicy) -> Option<T> {
        match self.ok_or_err_with_policy(policy) {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    }

    /// Converts the `DecimalResult` into [Result<T, ArithmeticError>].
    /// Returns
    /// - `Ok(T)` if none of the emergency flags are set.
    /// - `Err(ArithmeticError)` if at least one emergency flag is set.
    #[inline]
    pub const fn ok_or_err(self) -> Result<T, ArithmeticError> {
        const POLICY: ArithmeticPolicy = ArithmeticPolicy::default();
        self.ok_or_err_with_policy(&POLICY)
    }

    /// Converts the `DecimalResult` into [Result<T, ArithmeticError>].
    /// Returns
    /// - `Ok(T)` if none of the emergency flags are set.
    /// - `Err(ArithmeticError)` if at least one emergency flag is set.
    #[inline]
    pub const fn ok_or_err_with_policy(
        self,
        policy: &ArithmeticPolicy,
    ) -> Result<T, ArithmeticError> {
        if self.flags.contains(Flags::DIVIDE_BY_ZERO) {
            return Err(ArithmeticError::DivideByZero);
        }

        if self.flags.contains(Flags::NEGATIVE) {
            return Err(ArithmeticError::Signed);
        }

        if self.flags.contains(Flags::OVERFLOW) {
            match policy.overflow_policy {
                OverflowPolicy::Strict => {
                    return Err(ArithmeticError::Overflow);
                }
                OverflowPolicy::Saturating => {}
            }
        }

        if self.flags.contains(Flags::INEXACT) {
            match policy.rounding_policy {
                RoundingPolicy::Strict => {
                    return Err(ArithmeticError::Inexact);
                }
                RoundingPolicy::Round => {}
            }
        }

        Ok(self.result)
    }
}

macro_rules! result {
    ($value: expr) => {
        DecimalResult::new($value)
    };
}

pub(crate) use result;
