//! # Unsigned Decimal
//!
//! `fastnum` provides a several unsigned decimal numbers suitable for financial
//! calculations that require significant integral and fractional digits with no
//! round-off errors (such as 0.1 + 0.2 ≠ 0.3).
//!
//! | Decimal               | Integer | Bits | Max significant integer | Helper macro               |
//! |-----------------------|---------|------|-----------------|------------------------------------|
//! | [UD128](crate::UD128) | [U128]  | 128  | 2<sup>128</sup> | [`udec128!(0.1)`](crate::udec128!) |
//! | [UD256](crate::UD256) | [U256]  | 256  | 2<sup>256</sup> | [`udec256!(0.1)`](crate::udec256!) |
//! | [UD512](crate::UD512) | [U512]  | 512  | 2<sup>512</sup> | [`udec512!(0.1)`](crate::udec512!) |
//!
//!
//! ## Example
//!
//! ```
//! use fastnum::udec256;
//!
//! println!("Float with decimal: {} vs {}", 0.1_f32, udec256!(0.1));
//! ```

mod extras;
mod impls;

#[doc(hidden)]
pub mod parse;

use std::fmt;

use crate::{decimal::format, U128, U256, U512};

/// # Unsigned Decimal
///
/// Generic unsigned N-bits decimal number.
/// Consists of N-bit big unsigned integer, paired with a 64-bit signed
/// integer scaling factor which determines the position of the decimal point.
#[derive(Copy, Clone)]
pub struct UnsignedDecimal<UINT> {
    /// Unsigned integer for significant digits of a decimal number.
    value: UINT,

    /// A positive scale means a negative power of 10.
    scale: i64,
}

impl<UINT> UnsignedDecimal<UINT> {
    #[inline]
    pub(crate) const fn new(value: UINT, scale: i64) -> Self {
        Self { value, scale }
    }

    /// Returns the scale of the `UnsignedDecimal`, the total number of
    /// digits to the right of the decimal point (including insignificant
    /// leading zeros).
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::udec256;
    /// use std::str::FromStr;
    ///
    /// let a = udec256!(12345);  // No fractional part
    /// let b = udec256!(123.45);  // Fractional part
    /// let c = udec256!(0.0000012345);  // Completely fractional part
    /// let d = udec256!(500000000);  // Negative-fractional part
    ///
    /// assert_eq!(a.fractional_digit_count(), 0);
    /// assert_eq!(b.fractional_digit_count(), 2);
    /// assert_eq!(c.fractional_digit_count(), 10);
    /// assert_eq!(d.fractional_digit_count(), -9);
    /// ```
    #[inline]
    pub const fn fractional_digit_count(&self) -> i64 {
        self.scale
    }
}

impl<UINT: Copy> UnsignedDecimal<UINT> {
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
    /// assert_eq!(a.significant_digits(), u256!(12345));
    ///
    /// let b = udec256!(1.0);
    /// assert_eq!(b.significant_digits(), u256!(10));
    /// ```
    #[inline]
    pub const fn significant_digits(&self) -> UINT {
        self.value
    }
}

macro_rules! pos_const {
    ($UINT: ident, $bits: literal, $($name: ident $num: literal), *) => {
        $(
            #[doc = concat!("The value of `", $num, "` represented by ", $bits, "-bits `UnsignedDecimal`.")]
            pub const $name: Self = Self::new($UINT::$name, 0);
        )*
    }
}

macro_rules! macro_impl {
    ($UINT: ident, $bits: literal) => {
        #[doc = concat!("UnsignedDecimal with ", $bits, "-bit significant integer part.")]
        impl UnsignedDecimal<$UINT> {
            /// The maximum value that this type can represent.
            pub const MAX: Self = Self::new($UINT::MAX, i64::MIN);

            /// The minimum value that this type can represent.
            ///
            /// # Example
            ///
            /// ```
            #[doc = concat!("use fastnum::UD", stringify!($bits), ";")]
            ///
            #[doc = concat!("assert_eq!(UD", stringify!($bits), "::MIN, UD", stringify!($bits), "::ZERO);")]
            /// ```
            pub const MIN: Self = Self::new($UINT::MIN, 0);

            pos_const!($UINT, $bits, ZERO 0, ONE 1, TWO 2, THREE 3, FOUR 4, FIVE 5, SIX 6, SEVEN 7, EIGHT 8, NINE 9, TEN 10);

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
                self.value.is_zero()
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
                Self::new($UINT::ONE, -exp)
            }

            /// __Normalize__ this unsigned decimal moving all significant trailing zeros into the exponent.
            ///
            /// # Examples
            ///
            /// ```
            /// use fastnum::{udec256, u256};
            ///
            /// let a = udec256!(1234500);
            /// assert_eq!(a.significant_digits(), u256!(1234500));
            /// assert_eq!(a.fractional_digit_count(), 0);
            ///
            /// let b = a.normalized();
            /// assert_eq!(b.significant_digits(), u256!(12345));
            /// assert_eq!(b.fractional_digit_count(), -2);
            /// ```
            pub const fn normalized(mut self) -> Self {
                if self.value.is_zero() {
                    self.scale = 0;
                } else {
                    while !self.value.is_zero() && self.value.rem($UINT::TEN).is_zero() {
                        self.value = self.value.div($UINT::TEN);
                        self.scale -= 1;
                    }
                }
                self
            }

            ///
            /// Create string of this unsigned decimal in scientific notation.
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

            /// Write unsigned decimal in scientific notation to writer `w`.
            #[inline]
            pub(crate) fn write_scientific_notation<W: fmt::Write>(
                &self,
                w: &mut W,
            ) -> fmt::Result {
                if self.is_zero() {
                    return w.write_str("0e0");
                }
                let digits = self.value.to_str_radix(10);
                let scale = self.scale;
                format::write_scientific_notation(digits, scale, w)
            }

            ///
            /// Create string of this unsigned decimal in engineering notation.
            ///
            /// Engineering notation is scientific notation with the exponent
            /// coerced to a multiple of three
            ///
            /// ```
            /// use fastnum::udec256;
            /// let n = udec256!(12345678);
            /// assert_eq!(&n.to_engineering_notation(), "12.345678e6");
            /// ```
            pub fn to_engineering_notation(&self) -> String {
                let mut output = String::new();
                self.write_engineering_notation(&mut output)
                    .expect("Could not write to string");
                output
            }

            /// Write unsigned decimal in engineering notation to writer `w`.
            pub(crate) fn write_engineering_notation<W: fmt::Write>(
                &self,
                w: &mut W,
            ) -> fmt::Result {
                if self.is_zero() {
                    return w.write_str("0e0");
                }
                let digits = self.value.to_str_radix(10);
                let scale = self.scale;
                format::write_engineering_notation(digits, scale, w)
            }
        }

        impl Default for UnsignedDecimal<$UINT> {
            #[inline]
            fn default() -> Self {
                Self::ZERO
            }
        }
    };
}

macro_impl!(U128, 128);
macro_impl!(U256, 256);
macro_impl!(U512, 512);
