use crate::doc;

/// Unified trait for conversions between fastnum numeric types.
///
/// Cast is a total conversion: it never fails and does not return an error.
/// Prefer `Cast` when the conversion is guaranteed to be correct by definition
/// (e.g., widening with preserved value or sign/zero extension that cannot
/// overflow).
///
/// If the conversion may go out of range or lose information (sign/high bits),
/// use [`TryCast`] instead.
///
/// # Examples:
///
/// ```
/// use fastnum::*;
///
/// let a = u128!(123);
///
/// // Lossless widening
/// let b: U256 = a.cast();
/// assert_eq!(u256!(123), b);
/// ```
///
/// See also: [`TryCast`] for fallible conversions.
pub trait Cast<T> {
    /// Performs an infallible, value-preserving conversion.
    ///
    /// This method never fails. Use it for conversions that are known to be
    /// safe and cannot overflow or lose information (e.g., widening).
    ///
    /// # Examples:
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// let x = u128!(123);
    /// let y: U256 = x.cast();
    /// assert_eq!(u256!(123), y);
    /// ```
    #[must_use = doc::must_use_op!()]
    fn cast(self) -> T;
}

/// Fallible conversion between fastnum numeric types (akin to `TryFrom`, but
/// for fastnum).
///
/// TryCast returns `Result<T, Error>`, where `Error` describes why the
/// conversion failed (e.g., out-of-range for the target type or invalid sign
/// conversion).
///
/// Use `TryCast` when:
/// - the value may not fit into the target type (overflow/underflow);
/// - information could be lost (e.g., negative to unsigned);
/// - you need explicit error handling rather than silent truncation.
///
/// # Examples:
///
/// ```
/// use fastnum::*;
///
/// // Successful conversion (value fits in the target type)
/// let x = u256!(42);
/// let y: U128 = x.try_cast().unwrap();
/// assert_eq!(u128!(42), y);
///
/// // Failing conversion (negative to unsigned)
/// let neg = i256!(-1);
/// assert!(<I256 as TryCast<U64>>::try_cast(neg).is_err());
/// ```
///
/// Tips:
/// - If the conversion is always valid by design — prefer [`Cast`].
/// - If range/sign must be checked — prefer `TryCast`.
pub trait TryCast<T> {
    /// The type returned in the event of a conversion error.
    type Error;

    /// Attempts to convert `self` into `T`, returning an error on failure.
    ///
    /// Use this method when the conversion may be lossy or out-of-range.
    ///
    /// # Examples:
    ///
    /// ```
    /// use fastnum::*;
    ///
    /// // In-range: succeeds
    /// let a = u256!(18446744073709551615);
    /// let b: U64 = a.try_cast().unwrap();
    /// assert_eq!(u64!(18446744073709551615), b);
    ///
    /// // Out-of-range: fails with an error
    /// let big = u256!(18446744073709551616);
    /// let _ = <U256 as TryCast<U64>>::try_cast(big).unwrap_err();
    /// ```
    fn try_cast(self) -> Result<T, Self::Error>;
}
