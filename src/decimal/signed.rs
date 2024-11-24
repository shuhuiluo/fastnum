//! # Signed Decimal
//!
//! `fastnum` provides a several signed decimal numbers suitable for financial
//! calculations that require significant integral and fractional digits with no
//! round-off errors (such as 0.1 + 0.2 ≠ 0.3).
//!
//! | Decimal             | Integer | Bits | Max significant | Helper macro                     |
//! |---------------------|---------|------|-----------------|----------------------------------|
//! | [D128](crate::D128) | [U128]  | 128  | 2<sup>128</sup> | [`dec128!(0.1)`](crate::dec128!) |
//! | [D256](crate::D256) | [U256]  | 256  | 2<sup>256</sup> | [`dec256!(0.1)`](crate::dec256!) |
//! | [D512](crate::D512) | [U512]  | 512  | 2<sup>512</sup> | [`dec512!(0.1)`](crate::dec512!) |

mod doc;
mod extras;
mod impls;
mod parse;
mod sign;

pub use sign::Sign;

use impls::decimal::consts::consts_impl;

use core::{cmp::Ordering, fmt};

use crate::{
    decimal::{
        math::result,
        signed::sign::signify_result,
        unsigned::{round, UnsignedDecimal},
        DecimalResult, ParseError, RoundingMode,
    },
    int::UInt,
};

/// # Signed Decimal
///
/// Generic signed N-bits decimal number.
/// Consists of N-bit unsigned [UnsignedDecimal], paired with a [Sign].
#[derive(Copy, Clone)]
pub struct Decimal<const N: usize> {
    /// An N-bit unsigned decimal.
    value: UnsignedDecimal<N>,

    /// Sign
    sign: Sign,
}

consts_impl!();

impl<const N: usize> Decimal<N> {
    #[inline]
    pub(crate) const fn new(value: UnsignedDecimal<N>, sign: Sign) -> Self {
        Self { value, sign }
    }

    /// Creates and initializes a Decimal from string.
    #[inline]
    pub const fn parse_str(s: &str) -> Self {
        match Self::from_str(s) {
            Ok(n) => n,
            Err(e) => {
                panic!("{}", e.description())
            }
        }
    }

    /// Creates and initializes a Decimal from string.
    pub const fn from_str(s: &str) -> Result<Self, ParseError> {
        parse::from_str(s)
    }

    /// Returns the internal big integer, representing the significant
    /// decimal digits of a `Decimal`, including significant trailing
    /// zeros.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::{dec256, u256};
    ///
    /// let a = dec256!(-123.45);
    /// assert_eq!(a.decimal_digits(), u256!(12345));
    ///
    /// let b = dec256!(-1.0);
    /// assert_eq!(b.decimal_digits(), u256!(10));
    /// ```
    #[inline]
    pub const fn decimal_digits(&self) -> UInt<N> {
        self.value.decimal_digits()
    }

    /// Returns the count of digits in the non-scaled integer representation
    #[inline]
    pub const fn decimal_digits_count(&self) -> usize {
        self.value.decimal_digits_count()
    }

    /// Returns the scale of the `Decimal`, the total number of
    /// digits to the right of the decimal point (including insignificant
    /// leading zeros).
    ///
    /// # Examples:
    ///
    /// ```
    /// use fastnum::dec256;
    ///
    /// let a = dec256!(12345);  // No fractional part
    /// let b = dec256!(123.45);  // Fractional part
    /// let c = dec256!(0.0000012345);  // Completely fractional part
    /// let d = dec256!(500000000);  // No fractional part
    /// let e = dec256!(5e9);  // Negative-fractional part
    ///
    /// assert_eq!(a.fractional_digits_count(), 0);
    /// assert_eq!(b.fractional_digits_count(), 2);
    /// assert_eq!(c.fractional_digits_count(), 10);
    /// assert_eq!(d.fractional_digits_count(), 0);
    /// assert_eq!(e.fractional_digits_count(), -9);
    /// ```
    #[inline]
    pub const fn fractional_digits_count(&self) -> i64 {
        self.value.fractional_digits_count()
    }

    /// Return the sign of the `Decimal` as [Sign].
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::{decimal::Sign, dec256};
    ///
    /// assert_eq!(dec256!(-1.0).sign(), Sign::Minus);
    /// assert_eq!(dec256!(0.0).sign(),  Sign::NoSign);
    /// assert_eq!(dec256!(+1.0).sign(),  Sign::Plus);
    /// ```
    #[inline]
    pub const fn sign(&self) -> Sign {
        self.sign
    }

    /// Invert sign of given decimal.
    #[inline]
    pub const fn neg(self) -> Self {
        Self::new(self.value, self.sign.not())
    }

    /// Get the absolute value of the decimal (non-negative sign).
    #[inline]
    pub const fn abs(&self) -> UnsignedDecimal<N> {
        self.value
    }

    /// Initialize decimal with `1 * 10`<sup>exp</sup> value.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::{UD256, udec256};
    ///
    /// assert_eq!(UD256::from_scale(0), udec256!(1));
    /// assert_eq!(UD256::from_scale(-0), udec256!(1));
    /// assert_eq!(UD256::from_scale(-3), udec256!(0.001));
    /// assert_eq!(UD256::from_scale(3), udec256!(1000));
    /// ```
    #[inline]
    pub const fn from_scale(exp: i64) -> Self {
        Self::new(UnsignedDecimal::<N>::from_scale(exp), Sign::NoSign)
    }

    /// __Normalize__ this decimal moving all significant trailing zeros into
    /// the exponent.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::{dec256, u256};
    ///
    /// let a = dec256!(-1234500);
    /// assert_eq!(a.decimal_digits(), u256!(1234500));
    /// assert_eq!(a.fractional_digits_count(), 0);
    ///
    /// let b = a.normalized();
    /// assert_eq!(b.decimal_digits(), u256!(12345));
    /// assert_eq!(b.fractional_digits_count(), -2);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn normalized(self) -> Self {
        Self::new(UnsignedDecimal::<N>::normalized(self.value), self.sign)
    }

    /// Tests for `self` and `other` values to be equal, and is used by `==`
    /// operator.
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn eq(&self, rhs: &Self) -> bool {
        self.sign.eq(&rhs.sign) && self.value.eq(&rhs.value)
    }

    /// Compares and returns the maximum of two signed decimal values.
    ///
    /// Returns the second argument if the comparison determines them to be
    /// equal.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::{dec256};
    ///
    /// assert_eq!(dec256!(1).max(dec256!(2)), dec256!(2));
    /// assert_eq!(dec256!(2).max(dec256!(2)), dec256!(2));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn max(self, other: Self) -> Self {
        match self.cmp(&other) {
            Ordering::Less | Ordering::Equal => other,
            _ => self,
        }
    }

    /// Compares and returns the minimum of two signed decimal values.
    ///
    /// Returns the first argument if the comparison determines them to be
    /// equal.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::dec256;
    ///
    /// assert_eq!(dec256!(1).min(dec256!(2)), dec256!(1));
    /// assert_eq!(dec256!(2).min(dec256!(2)), dec256!(2));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn min(self, other: Self) -> Self {
        match self.cmp(&other) {
            Ordering::Less | Ordering::Equal => self,
            _ => other,
        }
    }

    /// Restrict a signed decimal value to a certain interval.
    ///
    /// Returns `max` if `self` is greater than `max`, and `min` if `self` is
    /// less than `min`. Otherwise, this returns `self`.
    ///
    /// # Panics
    ///
    /// Panics if `min > max`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::dec256;
    ///
    /// assert_eq!(dec256!(-3).clamp(dec256!(-2), dec256!(1)), dec256!(-2));
    /// assert_eq!(dec256!(0).clamp(dec256!(-2), dec256!(1)), dec256!(0));
    /// assert_eq!(dec256!(2).clamp(dec256!(-2), dec256!(1)), dec256!(1));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn clamp(self, min: Self, max: Self) -> Self {
        assert!(min.le(&max));
        if let Ordering::Less = self.cmp(&min) {
            min
        } else if let Ordering::Greater = self.cmp(&max) {
            max
        } else {
            self
        }
    }

    /// Tests signed decimal `self` less than `other` and is used by the `<`
    /// operator.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::dec256;
    ///
    /// assert_eq!(dec256!(1.0).lt(&dec256!(1.0)), false);
    /// assert_eq!(dec256!(1.0).lt(&dec256!(2.0)), true);
    /// assert_eq!(dec256!(2.0).lt(&dec256!(1.0)), false);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn lt(&self, other: &Self) -> bool {
        #[allow(clippy::match_like_matches_macro)]
        match self.cmp(other) {
            Ordering::Less => true,
            _ => false,
        }
    }

    /// Tests signed decimal `self` less than or equal to `other` and is used by
    /// the `<=` operator.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::dec256;
    ///
    /// assert_eq!(dec256!(1.0).le(&dec256!(1.0)), true);
    /// assert_eq!(dec256!(1.0).le(&dec256!(2.0)), true);
    /// assert_eq!(dec256!(2.0).le(&dec256!(1.0)), false);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn le(&self, other: &Self) -> bool {
        #[allow(clippy::match_like_matches_macro)]
        match self.cmp(other) {
            Ordering::Less | Ordering::Equal => true,
            _ => false,
        }
    }

    /// Tests signed decimal `self` greater than `other` and is used by the `>`
    /// operator.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::dec256;
    ///
    /// assert_eq!(dec256!(1.0).gt(&dec256!(1.0)), false);
    /// assert_eq!(dec256!(1.0).gt(&dec256!(2.0)), false);
    /// assert_eq!(dec256!(2.0).gt(&dec256!(1.0)), true);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn gt(&self, other: &Self) -> bool {
        #[allow(clippy::match_like_matches_macro)]
        match self.cmp(other) {
            Ordering::Greater => true,
            _ => false,
        }
    }

    /// Tests signed decimal `self` greater than or equal to `other` and is used
    /// by the `>=` operator.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::dec256;
    ///
    /// assert_eq!(dec256!(1.0).ge(&dec256!(1.0)), true);
    /// assert_eq!(dec256!(1.0).ge(&dec256!(2.0)), false);
    /// assert_eq!(dec256!(2.0).ge(&dec256!(1.0)), true);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn ge(&self, other: &Self) -> bool {
        #[allow(clippy::match_like_matches_macro)]
        match self.cmp(other) {
            Ordering::Greater | Ordering::Equal => true,
            _ => false,
        }
    }

    /// This method returns an [`Ordering`] between `self` and `other`.
    ///
    /// By convention, `self.cmp(&other)` returns the ordering matching the
    /// expression `self <operator> other` if true.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::dec256;
    /// use std::cmp::Ordering;
    ///
    /// assert_eq!(dec256!(5).cmp(&dec256!(10)), Ordering::Less);
    /// assert_eq!(dec256!(10).cmp(&dec256!(5)), Ordering::Greater);
    /// assert_eq!(dec256!(5).cmp(&dec256!(5)), Ordering::Equal);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn cmp(&self, rhs: &Self) -> Ordering {
        match (self.sign, rhs.sign) {
            (Sign::NoSign, Sign::Minus) | (Sign::Plus, Sign::Minus) => Ordering::Greater,
            (Sign::Minus, Sign::NoSign) | (Sign::Minus, Sign::Plus) => Ordering::Less,
            (Sign::Minus, Sign::Minus) => self.value.cmp(&rhs.value).reverse(),
            (Sign::Plus, Sign::NoSign) => self.value.cmp(&rhs.value).then(Ordering::Greater),
            (Sign::NoSign, Sign::Plus) => self.value.cmp(&rhs.value).then(Ordering::Less),
            (Sign::NoSign, Sign::NoSign) | (Sign::Plus, Sign::Plus) => self.value.cmp(&rhs.value),
        }
    }

    /// Calculates `self` + `rhs`.
    ///
    /// Returns [DecimalResult] with result of addition and [emergency
    /// flags](crate#arithmetic-result). Is internally used by the `+`
    /// operator.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::{dec256, D256};
    /// use fastnum::decimal::RoundingMode;
    ///
    /// let a = D256::ONE;
    /// let b = D256::TWO;
    ///
    /// let c = a.add(b, RoundingMode::default()).unwrap();
    /// assert_eq!(c, dec256!(3));
    /// ```
    ///
    /// ```should_panic
    /// use fastnum::{dec256, D256};
    /// use fastnum::decimal::RoundingMode;
    ///
    /// let a = D256::MAX;
    /// let b = D256::MAX;
    ///
    /// let c = a + b;
    /// ```
    ///
    /// For more information about flags and [crate::decimal::ArithmeticPolicy]
    /// see: [section](crate#arithmetic-result).
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn add(self, rhs: Self, rounding_mode: RoundingMode) -> DecimalResult<Self> {
        match (self.sign, rhs.sign) {
            // same sign => keep the sign with the sum of magnitudes
            (Sign::NoSign, Sign::Plus)
            | (Sign::Plus, Sign::NoSign)
            | (Sign::NoSign, Sign::NoSign)
            | (Sign::Plus, Sign::Plus)
            | (Sign::Minus, Sign::Minus) => {
                signify_result(self.value.add(rhs.value, rounding_mode), self.sign)
            }
            // opposite signs => keep the sign of the larger with the difference of magnitudes
            (_, _) => match self.value.cmp(&rhs.value) {
                Ordering::Less => {
                    signify_result(rhs.value.sub(self.value, rounding_mode), rhs.sign)
                }
                Ordering::Greater => {
                    signify_result(self.value.sub(rhs.value, rounding_mode), self.sign)
                }
                Ordering::Equal => result!(Self::ZERO),
            },
        }
    }

    /// Calculates `self` - `rhs`.
    ///
    /// Returns [DecimalResult] with result of subtraction and [emergency
    /// flags](crate#arithmetic-result). Is internally used by the `-`
    /// operator.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::{dec256, D256};
    /// use fastnum::decimal::RoundingMode;
    ///
    /// let a = D256::ONE;
    /// let b = D256::TWO;
    ///
    /// let c = a.sub(b, RoundingMode::default()).unwrap();
    /// assert_eq!(c, dec256!(-1));
    /// ```
    ///
    /// For more information about flags and [crate::decimal::ArithmeticPolicy]
    /// see: [section](crate#arithmetic-result).
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn sub(self, rhs: Self, rounding_mode: RoundingMode) -> DecimalResult<Self> {
        match (self.sign, rhs.sign) {
            // same sign => keep or toggle the sign of the left with the difference of magnitudes
            (Sign::NoSign, Sign::Plus)
            | (Sign::Plus, Sign::NoSign)
            | (Sign::NoSign, Sign::NoSign)
            | (Sign::Plus, Sign::Plus)
            | (Sign::Minus, Sign::Minus) => match self.value.cmp(&rhs.value) {
                Ordering::Less => {
                    signify_result(rhs.value.sub(self.value, rounding_mode), self.sign.not())
                }
                Ordering::Greater => {
                    signify_result(self.value.sub(rhs.value, rounding_mode), self.sign)
                }
                Ordering::Equal => result!(Self::ZERO),
            },
            // opposite signs => keep the sign of the left with the sum of magnitudes
            (_, _) => signify_result(self.value.add(rhs.value, rounding_mode), self.sign),
        }
    }

    /// Calculates `self` × `rhs`.
    ///
    /// Returns [DecimalResult] with result of multiplication and [emergency
    /// flags](crate#arithmetic-result). Is internally used by the `*`
    /// operator.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::{dec256, D256};
    /// use fastnum::decimal::RoundingMode;
    ///
    /// let a = D256::FIVE;
    /// let b = D256::TWO;
    ///
    /// let c = a.mul(b, RoundingMode::default()).unwrap();
    /// assert_eq!(c, dec256!(10));
    /// ```
    ///
    /// ```should_panic
    /// use fastnum::{dec256, D256};
    /// use fastnum::decimal::RoundingMode;
    ///
    /// let a = D256::MAX;
    /// let b = D256::MAX;
    ///
    /// let c = a * b;
    /// ```
    ///
    /// For more information about flags and [crate::decimal::ArithmeticPolicy]
    /// see: [section](crate#arithmetic-result).
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn mul(self, rhs: Self, rounding_mode: RoundingMode) -> DecimalResult<Self> {
        signify_result(
            self.value.mul(rhs.value, rounding_mode),
            self.sign.mul(rhs.sign),
        )
    }

    /// Calculates `self` ÷ `rhs`.
    ///
    /// Returns [DecimalResult] with result of division and [emergency
    /// flags](crate#arithmetic-result). Is internally used by the `/`
    /// operator.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::{dec256, D256};
    /// use fastnum::decimal::RoundingMode;
    ///
    /// let a = D256::FIVE;
    /// let b = D256::TWO;
    ///
    /// let c = a.neg().div(b, RoundingMode::default()).unwrap();
    /// assert_eq!(c, dec256!(-2.5));
    /// ```
    ///
    /// ```should_panic
    /// use fastnum::{dec256, D256};
    /// use fastnum::decimal::RoundingMode;
    ///
    /// let a = D256::ONE;
    /// let b = D256::ZERO;
    ///
    /// let c = a / b;
    /// ```
    ///
    /// For more information about flags and [crate::decimal::ArithmeticPolicy]
    /// see: [section](crate#arithmetic-result).
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn div(self, rhs: Self, rounding_mode: RoundingMode) -> DecimalResult<Self> {
        signify_result(
            self.value.div(rhs.value, rounding_mode),
            self.sign.div(rhs.sign),
        )
    }

    /// Calculates `self` % `rhs`.
    ///
    /// Returns [DecimalResult] with result of division reminder and [emergency
    /// flags](crate#arithmetic-result). Is internally used by the `%`
    /// operator.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::{dec256, D256};
    /// use fastnum::decimal::RoundingMode;
    ///
    /// let a = D256::FIVE;
    /// let b = D256::TWO;
    ///
    /// let c = a.rem(b.neg(), RoundingMode::default()).unwrap();
    /// assert_eq!(c, dec256!(1));
    /// ```
    ///
    /// For more information about flags and [crate::decimal::ArithmeticPolicy]
    /// see: [section](crate#arithmetic-result).
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn rem(self, rhs: Self, rounding_mode: RoundingMode) -> DecimalResult<Self> {
        signify_result(self.value.rem(rhs.value, rounding_mode), self.sign)
    }

    /// Return given decimal number rounded to 'digits' precision after the
    /// decimal point, using given [RoundingMode] unwrapped with default
    /// rounding and overflow policy.
    ///
    /// # Panics:
    ///
    /// This method will panic if round operation (up-scale or down-scale)
    /// performs with some emergency flags and specified
    /// [crate::decimal::ArithmeticPolicy] enjoin to panic when the
    /// corresponding flag occurs.
    ///
    /// For more information about flags and [crate::decimal::ArithmeticPolicy]
    /// see: [section](crate#arithmetic-result).
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::{dec256, decimal::RoundingMode};
    ///
    /// let n = dec256!(129.41675);
    ///
    /// assert_eq!(n.round(2, RoundingMode::Up),  dec256!(129.42));
    /// assert_eq!(n.round(-1, RoundingMode::Down),  dec256!(120));
    /// assert_eq!(n.round(4, RoundingMode::HalfEven),  dec256!(129.4168));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn round(self, digits: i64, rounding_mode: RoundingMode) -> Self {
        self.with_scale(digits, rounding_mode).unwrap()
    }

    /// Returns [DecimalResult] with result of round given decimal number
    /// to 'digits' precision after the decimal point using given
    /// [RoundingMode].
    ///
    /// For more information about flags and [crate::decimal::ArithmeticPolicy]
    /// see: [section](crate#arithmetic-result).
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::{dec256, decimal::RoundingMode};
    ///
    /// let n = dec256!(129.41675);
    ///
    /// assert_eq!(n.with_scale(2, RoundingMode::Up).unwrap(),  dec256!(129.42));
    /// assert_eq!(n.with_scale(-1, RoundingMode::Down).unwrap(),  dec256!(120));
    /// assert_eq!(n.with_scale(4, RoundingMode::HalfEven).unwrap(),  dec256!(129.4168));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn with_scale(
        self,
        new_scale: i64,
        rounding_mode: RoundingMode,
    ) -> DecimalResult<Self> {
        match (rounding_mode, self.sign) {
            (RoundingMode::Floor, Sign::Minus) => signify_result(
                round::with_scale(self.value, new_scale, RoundingMode::Up),
                self.sign,
            ),
            (RoundingMode::Ceiling, Sign::Minus) => signify_result(
                round::with_scale(self.value, new_scale, RoundingMode::Down),
                self.sign,
            ),
            (_, _) => signify_result(
                round::with_scale(self.value, new_scale, rounding_mode),
                self.sign,
            ),
        }
    }

    /// Create string of this decimal in scientific notation.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::dec256;
    ///
    /// let n = dec256!(-12345678);
    /// assert_eq!(&n.to_scientific_notation(), "-1.2345678e7");
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub fn to_scientific_notation(&self) -> String {
        let mut output = String::new();
        self.write_scientific_notation(&mut output)
            .expect("Could not write to string");
        output
    }

    /// Create string of this decimal in engineering notation.
    ///
    /// Engineering notation is scientific notation with the exponent
    /// coerced to a multiple of three
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::dec256;
    ///
    /// let n = dec256!(-12345678);
    /// assert_eq!(&n.to_engineering_notation(), "-12.345678e6");
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub fn to_engineering_notation(&self) -> String {
        let mut output = String::new();
        self.write_engineering_notation(&mut output)
            .expect("Could not write to string");
        output
    }
}

#[doc(hidden)]
impl<const N: usize> Decimal<N> {
    #[inline]
    pub(crate) fn type_name() -> String {
        format!("D{}", N * 64)
    }

    #[inline]
    pub(crate) fn write_scientific_notation<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        write!(w, "{}", self.sign)?;
        self.value.write_scientific_notation(w)
    }

    #[inline]
    pub(crate) fn write_engineering_notation<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        write!(w, "{}", self.sign)?;
        self.value.write_engineering_notation(w)
    }
}
