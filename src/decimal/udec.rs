mod extras;
mod impls;

use impls::consts::consts_impl;

use core::cmp::Ordering;

use crate::{
    decimal::{doc, Category, Context, Decimal, Flags, ParseError, RoundingMode},
    int::UInt,
    utils::err_msg,
};

/// # Unsigned Decimal
///
/// Generic unsigned N-bits decimal number.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct UnsignedDecimal<const N: usize>(Decimal<N>);

consts_impl!();

impl<const N: usize> UnsignedDecimal<N> {
    /// Creates and initializes an unsigned decimal from string.
    #[track_caller]
    #[inline]
    pub const fn from_str(s: &str) -> Result<Self, ParseError> {
        match Decimal::<N>::from_str(s) {
            Ok(d) => {
                if d.is_negative() {
                    Err(ParseError::Signed)
                } else {
                    Ok(Self::new(d))
                }
            }
            Err(e) => Err(e),
        }
    }

    /// Creates and initializes an unsigned decimal from string.
    ///
    /// # Panics
    ///
    /// This function will panic if `UnsignedDecimal<N>` cannot be constructed
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
    /// assert_eq!(a.digits(), u256!(12345));
    ///
    /// let b = udec256!(1.0);
    /// assert_eq!(b.digits(), u256!(10));
    /// ```
    #[inline]
    pub const fn digits(&self) -> UInt<N> {
        self.0.digits()
    }

    /// Returns the count of digits in the non-scaled integer representation
    #[inline]
    pub const fn digits_count(&self) -> usize {
        self.0.digits_count()
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
    pub const fn fractional_digits_count(&self) -> i16 {
        self.0.fractional_digits_count()
    }

    /// Return if the referenced unsigned decimal is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::{udec256};
    ///
    /// let a = udec256!(0);
    /// assert!(a.is_zero());
    ///
    /// let b = udec256!(0.0);
    /// assert!(b.is_zero());
    ///
    /// let c = udec256!(0.00);
    /// assert!(c.is_zero());
    ///
    /// let d = udec256!(0.1);
    /// assert!(!d.is_zero());
    /// ```
    #[inline]
    pub const fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    /// Return if the referenced unsigned decimal is strictly [Self::ONE].
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::{udec256};
    ///
    /// let a = udec256!(1);
    /// assert!(a.is_one());
    ///
    /// let b = udec256!(10e-1);
    /// assert!(!b.is_one());
    /// ```
    #[inline]
    pub const fn is_one(&self) -> bool {
        self.0.is_one()
    }

    #[inline]
    pub const fn is_op_div_by_zero(&self) -> bool {
        self.0.is_op_div_by_zero()
    }

    #[inline]
    pub const fn is_op_invalid(&self) -> bool {
        self.0.is_op_invalid()
    }

    #[must_use]
    #[inline]
    pub const fn is_op_subnormal(&self) -> bool {
        self.0.is_op_subnormal()
    }

    #[inline]
    pub const fn is_op_inexact(&self) -> bool {
        self.0.is_op_inexact()
    }

    #[inline]
    pub const fn is_op_rounded(&self) -> bool {
        self.0.is_op_rounded()
    }

    #[inline]
    pub const fn is_op_clamped(&self) -> bool {
        self.0.is_op_clamped()
    }

    #[inline]
    pub const fn is_op_ok(&self) -> bool {
        self.0.is_op_ok()
    }

    #[inline]
    pub const fn classify(&self) -> Category {
        self.0.classify()
    }

    #[inline]
    pub const fn is_normal(self) -> bool {
        self.0.is_normal()
    }

    #[inline]
    pub const fn is_subnormal(self) -> bool {
        self.0.is_subnormal()
    }

    #[inline]
    pub const fn is_finite(self) -> bool {
        self.0.is_finite()
    }

    #[inline]
    pub const fn is_infinite(self) -> bool {
        self.0.is_infinite()
    }

    #[inline]
    pub const fn is_nan(self) -> bool {
        self.0.is_nan()
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
    pub const fn from_scale(exp: i16) -> Self {
        Self::new(Decimal::<N>::from_scale(exp))
    }

    /// __Normalize__ this unsigned decimal moving all significant trailing
    /// zeros into the exponent.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::{udec256, u256, decimal::Context};
    ///
    /// let a = udec256!(1234500);
    /// assert_eq!(a.digits(), u256!(1234500));
    /// assert_eq!(a.fractional_digits_count(), 0);
    ///
    /// let b = a.normalized(Context::default());
    /// assert_eq!(b.digits(), u256!(12345));
    /// assert_eq!(b.fractional_digits_count(), -2);
    /// ```
    #[must_use = doc::must_use_op!()]
    pub const fn normalized(self, ctx: Context) -> Self {
        Self::new(self.0.normalized(ctx))
    }

    /// Invert sign of given decimal.
    #[inline]
    pub const fn neg(self) -> Decimal<N> {
        self.0.neg()
    }

    /// Tests for `self` and `other` values to be equal, and is used by `==`
    /// operator.
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }

    /// Tests for `self` and `other` values to be equal, and is used by `==`
    /// operator.
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn ne(&self, other: &Self) -> bool {
        self.0.ne(&other.0)
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
    pub const fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }

    /// Calculates `self` + `rhs`.
    ///
    /// Returns the result of addition and [emergency
    /// flags](crate#arithmetic-result). Is internally used by the `+`
    /// operator.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::{udec256, UD256, decimal::Context};
    ///
    /// let a = UD256::ONE;
    /// let b = UD256::TWO;
    ///
    /// let c = a.add(b, Context::default());
    /// assert_eq!(c, udec256!(3));
    /// ```
    ///
    /// ```should_panic
    /// use fastnum::{udec256, UD256};
    ///
    /// let a = UD256::MAX;
    /// let b = UD256::MAX;
    ///
    /// let c = a + b;
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn add(self, rhs: Self, ctx: Context) -> Self {
        Self::new(self.0.add(rhs.0, ctx))
    }

    /// Calculates `self` - `rhs`.
    ///
    /// Returns the result of subtraction and [emergency
    /// flags](crate#arithmetic-result). Is internally used by the `-`
    /// operator.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::{udec256, UD256, decimal::{Context, RoundingMode}};
    ///
    /// let a = UD256::FIVE;
    /// let b = UD256::TWO;
    ///
    /// let c = a.sub(b, Context::default());
    /// assert_eq!(c, udec256!(3));
    /// ```
    ///
    /// ```should_panic
    /// use fastnum::{udec256, UD256};
    ///
    /// let a = UD256::ZERO;
    /// let b = UD256::ONE;
    ///
    /// let c = a - b;
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn sub(self, rhs: Self, ctx: Context) -> Self {
        let res = self.0.sub(rhs.0, ctx);
        Self::from_signed(res, ctx)
    }

    /// Calculates `self` × `rhs`.
    ///
    /// Returns the result of multiplication and [emergency
    /// flags](crate#arithmetic-result). Is internally used by the `*`
    /// operator.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::{udec256, UD256, decimal::{Context, RoundingMode}};
    ///
    /// let a = UD256::FIVE;
    /// let b = UD256::TWO;
    ///
    /// let c = a.mul(b, Context::default());
    /// assert_eq!(c, udec256!(10));
    /// ```
    ///
    /// ```should_panic
    /// use fastnum::{udec256, UD256};
    ///
    /// let a = UD256::MAX;
    /// let b = UD256::MAX;
    ///
    /// let c = a * b;
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn mul(self, rhs: Self, ctx: Context) -> Self {
        Self::new(self.0.mul(rhs.0, ctx))
    }

    /// Calculates `self` ÷ `rhs`.
    ///
    /// Returns the result of division and [emergency
    /// flags](crate#arithmetic-result). Is internally used by the `/`
    /// operator.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::{udec256, UD256, decimal::{Context, RoundingMode}};
    ///
    /// let a = UD256::FIVE;
    /// let b = UD256::TWO;
    ///
    /// let c = a.div(b, Context::default());
    /// assert_eq!(c, udec256!(2.5));
    /// ```
    ///
    /// ```should_panic
    /// use fastnum::{udec256, UD256};
    ///
    /// let a = UD256::ONE;
    /// let b = UD256::ZERO;
    ///
    /// let c = a / b;
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn div(self, rhs: Self, ctx: Context) -> Self {
        Self::new(self.0.div(rhs.0, ctx))
    }

    /// Calculates `self` % `rhs`.
    ///
    /// Returns the result of division reminder and [emergency
    /// flags](crate#arithmetic-result). Is internally used by the `%`
    /// operator.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use fastnum::{udec256, UD256, decimal::{Context, RoundingMode}};
    ///
    /// let a = UD256::FIVE;
    /// let b = UD256::TWO;
    ///
    /// let c = a.rem(b, Context::default());
    /// assert_eq!(c, udec256!(1));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn rem(self, rhs: Self, ctx: Context) -> Self {
        Self::new(self.0.rem(rhs.0, ctx))
    }

    /// Return given decimal number rounded to 'digits' precision after the
    /// decimal point, using given [RoundingMode] unwrapped with default
    /// rounding and overflow policy.
    ///
    /// # Panics:
    ///
    /// This method will panic if round operation (up-scale or down-scale)
    /// performs with some emergency flags and specified
    /// [crate::decimal::Context] enjoin to panic when the
    /// corresponding flag occurs.
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
    pub const fn round(self, digits: i16, rounding_mode: RoundingMode) -> Self {
        Self::new(self.0.round(digits, rounding_mode))
    }

    /// Returns the result of rounding given decimal number
    /// to 'digits' precision after the decimal point using given
    /// [RoundingMode].
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::{udec256, decimal::{RoundingMode, Context}};
    ///
    /// let n = udec256!(129.41675);
    ///
    /// assert_eq!(n.with_scale(2, Context::default().with_rounding_mode(RoundingMode::Up)),  udec256!(129.42));
    /// assert_eq!(n.with_scale(-1, Context::default().with_rounding_mode(RoundingMode::Down)),  udec256!(120));
    /// assert_eq!(n.with_scale(4, Context::default().with_rounding_mode(RoundingMode::HalfEven)),  udec256!(129.4168));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn with_scale(self, new_scale: i16, ctx: Context) -> Self {
        Self::new(self.0.with_scale(new_scale, ctx))
    }

    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn ok(self) -> Option<Self> {
        if self.flags().is_special() || self.flags().has_signals() {
            None
        } else {
            Some(self)
        }
    }

    /// Create a string of this unsigned decimal in scientific notation.
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
        self.0.to_scientific_notation()
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
        self.0.to_engineering_notation()
    }
}

#[doc(hidden)]
impl<const N: usize> UnsignedDecimal<N> {
    #[inline]
    pub(crate) const fn new(dec: Decimal<N>) -> Self {
        debug_assert!(!dec.is_negative());
        Self(dec)
    }

    #[inline]
    pub(crate) const fn from_signed(dec: Decimal<N>, _ctx: Context) -> Self {
        if dec.is_negative() {
            #[cfg(debug_assertions)]
            panic!(err_msg!("operation has negative result for unsigned type"));
            #[cfg(not(debug_assertions))]
            Self::new(Decimal::NAN.with_signals_from_and(&dec, Signal::OP_INVALID))
        } else {
            Self::new(dec)
        }
    }

    #[inline]
    pub(crate) fn type_name() -> String {
        format!("UD{}", N * 64)
    }

    #[inline]
    pub(crate) const fn flags(&self) -> Flags {
        self.0.flags()
    }
}
