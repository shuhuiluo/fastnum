mod doc;
mod extras;
mod impls;

use impls::decimal::{consts::consts_impl, ops::ops_impl};

pub(crate) mod math;
pub(crate) mod parse;
pub(crate) mod round;

use core::{cmp::Ordering, fmt};

use crate::{
    decimal::{
        format, math::DecimalResult, signed::Decimal, ParseError, RoundingMode,
        Sign,
    },
    int::{math::div_rem, UInt},
};

/// # Unsigned Decimal
///
/// Generic unsigned N-bits decimal number.
/// Consists of N-bit big unsigned integer, paired with a 64-bit signed
/// integer scaling factor which determines the position of the decimal point.
#[derive(Copy, Clone)]
pub struct UnsignedDecimal<const N: usize> {
    /// Unsigned integer for significant digits of a decimal number.
    value: UInt<N>,

    /// A positive scale means a negative power of 10.
    scale: i64,
}

consts_impl!();
ops_impl!();

impl<const N: usize> UnsignedDecimal<N> {
    #[inline]
    pub(crate) const fn new(value: UInt<N>, scale: i64) -> Self {
        Self { value, scale }
    }

    /// Creates and initializes an unsigned decimal from string.
    ///
    /// # Panics
    ///
    /// This function will panic if `UnsignedDecimal<N>` can not be constructed
    /// from given string.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::{UD256, udec256};
    ///
    /// assert_eq!(UD256::parse_str("1.2345"), udec256!(1.2345));
    /// ```
    #[track_caller]
    #[inline]
    pub const fn parse_str(s: &str) -> Self {
        match Self::from_str(s) {
            Ok(n) => n,
            Err(e) => panic!("{}", e.description()),
        }
    }

    /// Creates and initializes a Decimal from string.
    #[track_caller]
    #[inline]
    pub const fn from_str(s: &str) -> Result<Self, ParseError> {
        parse::from_str(s)
    }

    /// Returns the internal big integer, representing the significant
    /// decimal digits of a `UnsignedDecimal`, including significant trailing
    /// zeros.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::{udec256, u256};
    ///
    /// let a = udec256!(123.45);
    /// assert_eq!(a.decimal_digits(), u256!(12345));
    ///
    /// let b = udec256!(1.0);
    /// assert_eq!(b.decimal_digits(), u256!(10));
    /// ```
    #[inline]
    pub const fn decimal_digits(&self) -> UInt<N> {
        self.value
    }

    /// Returns the count of digits in the non-scaled integer representation
    #[inline]
    pub const fn decimal_digits_count(&self) -> usize {
        if self.is_zero() {
            return 1;
        }
        self.value.ilog10() as usize + 1
    }

    /// Returns the scale of the `UnsignedDecimal`, the total number of
    /// digits to the right of the decimal point (including insignificant
    /// leading zeros).
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::udec256;
    ///
    /// let a = udec256!(12345);  // No fractional part
    /// let b = udec256!(123.45);  // Fractional part
    /// let c = udec256!(0.0000012345);  // Completely fractional part
    /// let d = udec256!(500000000);  // No fractional part
    /// let e = udec256!(5e9);  // Negative-fractional part
    ///
    /// assert_eq!(a.fractional_digits_count(), 0);
    /// assert_eq!(b.fractional_digits_count(), 2);
    /// assert_eq!(c.fractional_digits_count(), 10);
    /// assert_eq!(d.fractional_digits_count(), 0);
    /// assert_eq!(e.fractional_digits_count(), -9);
    /// ```
    #[inline]
    pub const fn fractional_digits_count(&self) -> i64 {
        self.scale
    }

    /// Initialize unsigned decimal with `1 * 10`<sup>exp</sup> value.
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
        Self::new(UInt::ONE, -exp)
    }

    /// __Normalize__ this unsigned decimal moving all significant trailing
    /// zeros into the exponent.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::{udec256, u256};
    ///
    /// let a = udec256!(1234500);
    /// assert_eq!(a.decimal_digits(), u256!(1234500));
    /// assert_eq!(a.fractional_digits_count(), 0);
    ///
    /// let b = a.normalized();
    /// assert_eq!(b.decimal_digits(), u256!(12345));
    /// assert_eq!(b.fractional_digits_count(), -2);
    /// ```
    #[must_use = doc::must_use_op!()]
    pub const fn normalized(mut self) -> Self {
        if self.value.is_zero() {
            self.scale = 0;
        } else {
            let mut value;
            let mut remainder;
            while !self.value.is_zero() && self.scale > i64::MIN {
                (value, remainder) = div_rem(self.value, UInt::TEN);
                if remainder.is_zero() {
                    self.value = value;
                    self.scale -= 1;
                } else {
                    break;
                }
            }
        }
        self
    }

    /// Invert sign of given decimal.
    #[inline]
    pub const fn neg(self) -> Decimal<N> {
        Decimal::new(self, Sign::Minus)
    }

    /// Tests for `self` and `other` values to be equal, and is used by `==`
    /// operator.
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn eq(&self, rhs: &Self) -> bool {
        // TODO: performance optimization & some extra checks when normalize is partial
        let a = self.normalized();
        let b = rhs.normalized();
        (a.scale == b.scale) && (a.value.eq(&b.value))
    }

    /// Compares and returns the maximum of two unsigned decimal values.
    ///
    /// Returns the second argument if the comparison determines them to be
    /// equal.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::{udec256};
    ///
    /// assert_eq!(udec256!(1).max(udec256!(2)), udec256!(2));
    /// assert_eq!(udec256!(2).max(udec256!(2)), udec256!(2));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn max(self, other: Self) -> Self {
        match self.cmp(&other) {
            Ordering::Less | Ordering::Equal => other,
            _ => self,
        }
    }

    /// Compares and returns the minimum of two undecimal values.
    ///
    /// Returns the first argument if the comparison determines them to be
    /// equal.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::udec256;
    ///
    /// assert_eq!(udec256!(1).min(udec256!(2)), udec256!(1));
    /// assert_eq!(udec256!(2).min(udec256!(2)), udec256!(2));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn min(self, other: Self) -> Self {
        match self.cmp(&other) {
            Ordering::Less | Ordering::Equal => self,
            _ => other,
        }
    }

    /// Restrict an unsigned decimal value to a certain interval.
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
    /// use fastnum::udec256;
    ///
    /// assert_eq!(udec256!(0).clamp(udec256!(3), udec256!(5)), udec256!(3));
    /// assert_eq!(udec256!(3).clamp(udec256!(1), udec256!(5)), udec256!(3));
    /// assert_eq!(udec256!(6).clamp(udec256!(1), udec256!(5)), udec256!(5));
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

    /// Tests unsigned decimal `self` less than `other` and is used by the `<`
    /// operator.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::udec256;
    ///
    /// assert_eq!(udec256!(1.0).lt(&udec256!(1.0)), false);
    /// assert_eq!(udec256!(1.0).lt(&udec256!(2.0)), true);
    /// assert_eq!(udec256!(2.0).lt(&udec256!(1.0)), false);
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

    /// Tests unsigned decimal `self` less than or equal to `other` and is used
    /// by the `<=` operator.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::udec256;
    ///
    /// assert_eq!(udec256!(1.0).le(&udec256!(1.0)), true);
    /// assert_eq!(udec256!(1.0).le(&udec256!(2.0)), true);
    /// assert_eq!(udec256!(2.0).le(&udec256!(1.0)), false);
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

    /// Tests unsigned decimal `self` greater than `other` and is used by the
    /// `>` operator.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::udec256;
    ///
    /// assert_eq!(udec256!(1.0).gt(&udec256!(1.0)), false);
    /// assert_eq!(udec256!(1.0).gt(&udec256!(2.0)), false);
    /// assert_eq!(udec256!(2.0).gt(&udec256!(1.0)), true);
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

    /// Tests unsigned decimal `self` greater than or equal to `other` and is
    /// used by the `>=` operator.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::udec256;
    ///
    /// assert_eq!(udec256!(1.0).ge(&udec256!(1.0)), true);
    /// assert_eq!(udec256!(1.0).ge(&udec256!(2.0)), false);
    /// assert_eq!(udec256!(2.0).ge(&udec256!(1.0)), true);
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
    /// use fastnum::udec256;
    /// use std::cmp::Ordering;
    ///
    /// assert_eq!(udec256!(5).cmp(&udec256!(10)), Ordering::Less);
    /// assert_eq!(udec256!(10).cmp(&udec256!(5)), Ordering::Greater);
    /// assert_eq!(udec256!(5).cmp(&udec256!(5)), Ordering::Equal);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn cmp(&self, rhs: &Self) -> Ordering {
        match (self.is_zero(), rhs.is_zero()) {
            (true, true) => {
                return Ordering::Equal;
            }
            (true, false) => {
                return Ordering::Less;
            }
            (false, true) => {
                return Ordering::Greater;
            }
            (_, _) => {}
        }

        let a = self.normalized();
        let b = rhs.normalized();

        if a.scale == b.scale {
            return a.value.cmp(&b.value);
        }

        let a_exp = a.value.ilog10() as i64 - a.scale;
        let b_exp = b.value.ilog10() as i64 - b.scale;

        if a_exp == b_exp {
            if a.scale > b.scale {
                let (mul, false) = UInt::TEN.overflowing_pow((a.scale - b.scale) as u32) else {
                    return Ordering::Less;
                };

                let (value, false) = b.value.overflowing_mul(mul) else {
                    return Ordering::Less;
                };

                a.value.cmp(&value)
            } else {
                let (mul, false) = UInt::TEN.overflowing_pow((b.scale - a.scale) as u32) else {
                    return Ordering::Less;
                };

                let (value, false) = a.value.overflowing_mul(mul) else {
                    return Ordering::Less;
                };

                value.cmp(&b.value)
            }
        } else if a_exp > b_exp {
            Ordering::Greater
        } else {
            Ordering::Less
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
    /// ```should_panic
    /// use fastnum::{udec256, UD256};
    /// use fastnum::decimal::RoundingMode;
    ///
    /// let a = UD256::MAX;
    /// let b = UD256::MAX;
    ///
    /// let c = a + b;
    /// ```
    ///
    /// For more information about flags and [crate::decimal::ArithmeticPolicy] see:
    /// [section](crate#arithmetic-result).
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn add(self, rhs: Self, rounding_mode: RoundingMode) -> DecimalResult<Self> {
        math::add(self, rhs, rounding_mode)
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
    /// use fastnum::{udec256, UD256};
    /// use fastnum::decimal::RoundingMode;
    ///
    /// let a = UD256::FIVE;
    /// let b = UD256::TWO;
    ///
    /// let c = a.sub(b, RoundingMode::default()).unwrap();
    /// assert_eq!(c, udec256!(3));
    /// ```
    ///
    /// ```should_panic
    /// use fastnum::{udec256, UD256};
    /// use fastnum::decimal::RoundingMode;
    ///
    /// let a = UD256::ZERO;
    /// let b = UD256::ONE;
    ///
    /// let c = a - b;
    /// ```
    ///
    /// For more information about flags and [crate::decimal::ArithmeticPolicy] see:
    /// [section](crate#arithmetic-result).
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn sub(self, rhs: Self, rounding_mode: RoundingMode) -> DecimalResult<Self> {
        math::sub(self, rhs, rounding_mode)
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
    /// use fastnum::{udec256, UD256};
    /// use fastnum::decimal::RoundingMode;
    ///
    /// let a = UD256::FIVE;
    /// let b = UD256::TWO;
    ///
    /// let c = a.div(b, RoundingMode::default()).unwrap();
    /// assert_eq!(c, udec256!(2.5));
    /// ```
    ///
    /// ```should_panic
    /// use fastnum::{udec256, UD256};
    /// use fastnum::decimal::RoundingMode;
    ///
    /// let a = UD256::ONE;
    /// let b = UD256::ZERO;
    ///
    /// let c = a / b;
    /// ```
    ///
    /// For more information about flags and [crate::decimal::ArithmeticPolicy] see:
    /// [section](crate#arithmetic-result).
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn div(self, rhs: Self, rounding_mode: RoundingMode) -> DecimalResult<Self> {
        math::div(self, rhs, rounding_mode)
    }

    /// Return given decimal number rounded to 'digits' precision after the
    /// decimal point, using given [RoundingMode] unwrapped with default
    /// rounding and overflow policy.
    ///
    /// # Panics:
    ///
    /// This method will panic if round operation (up-scale or down-scale)
    /// performs with some emergency flags and specified [crate::decimal::ArithmeticPolicy]
    /// enjoin to panic when the corresponding flag occurs.
    ///
    /// For more information about flags and [crate::decimal::ArithmeticPolicy] see:
    /// [section](crate#arithmetic-result).
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::{udec256, decimal::RoundingMode};
    ///
    /// let n = udec256!(129.41675);
    ///
    /// assert_eq!(n.round(2, RoundingMode::Up),  udec256!(129.42));
    /// assert_eq!(n.round(-1, RoundingMode::Down),  udec256!(120));
    /// assert_eq!(n.round(4, RoundingMode::HalfEven),  udec256!(129.4168));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn round(self, digits: i64, rounding_mode: RoundingMode) -> Self {
        self.with_scale(digits, rounding_mode).unwrap()
    }

    /// Returns [DecimalResult] with result of round given decimal number
    /// to 'digits' precision after the decimal point using given [RoundingMode].
    ///
    /// For more information about flags and [crate::decimal::ArithmeticPolicy] see:
    /// [section](crate#arithmetic-result).
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::{udec256, decimal::RoundingMode};
    ///
    /// let n = udec256!(129.41675);
    ///
    /// assert_eq!(n.with_scale(2, RoundingMode::Up).unwrap(),  udec256!(129.42));
    /// assert_eq!(n.with_scale(-1, RoundingMode::Down).unwrap(),  udec256!(120));
    /// assert_eq!(n.with_scale(4, RoundingMode::HalfEven).unwrap(),  udec256!(129.4168));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn with_scale(
        self,
        new_scale: i64,
        rounding_mode: RoundingMode,
    ) -> DecimalResult<Self> {
        round::with_scale(self, new_scale, rounding_mode)
    }

    /// Create string of this unsigned decimal in scientific notation.
    ///
    /// # Examples
    /// 
    /// ```
    /// use fastnum::udec256;
    ///
    /// let n = udec256!(12345678);
    /// assert_eq!(&n.to_scientific_notation(), "1.2345678e7");
    /// ```
    pub fn to_scientific_notation(&self) -> String {
        let mut output = String::new();
        self.write_scientific_notation(&mut output)
            .expect("Could not write to string");
        output
    }

    /// Create string of this unsigned decimal in engineering notation.
    ///
    /// Engineering notation is scientific notation with the exponent
    /// coerced to a multiple of three.
    /// 
    /// # Examples
    ///
    /// ```
    /// use fastnum::udec256;
    /// 
    /// let n = udec256!(12345678);
    /// assert_eq!(&n.to_engineering_notation(), "12.345678e6");
    /// ```
    pub fn to_engineering_notation(&self) -> String {
        let mut output = String::new();
        self.write_engineering_notation(&mut output)
            .expect("Could not write to string");
        output
    }
}

#[doc(hidden)]
impl<const N: usize> UnsignedDecimal<N> {
    /// Write unsigned decimal in scientific notation to writer `w`.
    #[inline]
    pub(crate) fn write_scientific_notation<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        if self.is_zero() {
            return w.write_str("0e0");
        }
        let digits = self.value.to_str_radix(10);
        let scale = self.scale;
        format::write_scientific_notation(digits, scale, w)
    }

    /// Write unsigned decimal in engineering notation to writer `w`.
    pub(crate) fn write_engineering_notation<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        if self.is_zero() {
            return w.write_str("0e0");
        }
        let digits = self.value.to_str_radix(10);
        let scale = self.scale;
        format::write_engineering_notation(digits, scale, w)
    }
}
