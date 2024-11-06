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
//!

mod extras;
mod impls;
mod sign;

pub use sign::Sign;

#[doc(hidden)]
pub mod parse;

use core::fmt;

use crate::{decimal::unsigned::UnsignedDecimal, U128, U256, U512};

/// # Signed Decimal
///
/// Generic signed N-bits decimal number.
/// Consists of N-bit unsigned [UnsignedDecimal], paired with a [Sign].
#[derive(Copy, Clone)]
pub struct Decimal<UINT> {
    /// A 256-bit decimal.
    value: UnsignedDecimal<UINT>,

    /// Sign
    sign: Sign,
}

impl<UINT> Decimal<UINT> {
    #[inline]
    pub(crate) const fn new(value: UnsignedDecimal<UINT>, sign: Sign) -> Self {
        Self { value, sign }
    }

    /// Returns the scale of the `Decimal`, the total number of
    /// digits to the right of the decimal point (including insignificant
    /// leading zeros).
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::dec256;
    /// use std::str::FromStr;
    ///
    /// let a = dec256!(12345);  // No fractional part
    /// let b = dec256!(123.45);  // Fractional part
    /// let c = dec256!(0.0000012345);  // Completely fractional part
    /// let d = dec256!(500000000);  // Negative-fractional part
    ///
    /// assert_eq!(a.fractional_digit_count(), 0);
    /// assert_eq!(b.fractional_digit_count(), 2);
    /// assert_eq!(c.fractional_digit_count(), 10);
    /// assert_eq!(d.fractional_digit_count(), -9);
    /// ```
    #[inline]
    pub const fn fractional_digit_count(&self) -> i64 {
        self.value.fractional_digit_count()
    }

    /// Return the sign of the `Decimal` as [Sign].
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::{decimal::signed::Sign, dec256};
    ///
    /// assert_eq!(dec256!(-1.0).sign(), Sign::Minus);
    /// assert_eq!(dec256!(0.0).sign(),  Sign::NoSign);
    /// assert_eq!(dec256!(+1.0).sign(),  Sign::Plus);
    /// ```
    #[inline]
    pub const fn sign(&self) -> Sign {
        self.sign
    }
}

impl<UINT: Copy> Decimal<UINT> {
    /// Invert sign of given decimal.
    #[inline]
    pub const fn negative(self) -> Self {
        Self::new(self.value, self.sign.not())
    }

    /// Take absolute value of the decimal (non-negative sign)
    #[inline]
    pub const fn abs(&self) -> UnsignedDecimal<UINT> {
        self.value
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
    /// assert_eq!(a.significant_digits(), u256!(12345));
    ///
    /// let b = dec256!(-1.0);
    /// assert_eq!(b.significant_digits(), u256!(10));
    /// ```
    #[inline]
    pub const fn significant_digits(&self) -> UINT {
        self.value.significant_digits()
    }
}

macro_rules! pos_const {
    ($UINT: ident, $bits: literal, $($name: ident $num: literal), *) => {
        $(
            #[doc = concat!("The value of `", $num, "` represented by ", $bits, "-bits `Decimal`.")]
            pub const $name: Self = Self::new(UnsignedDecimal::<$UINT>::$name, Sign::NoSign);
        )*
    }
}

macro_rules! macro_impl {
    ($UINT: ident, $bits: literal) => {
        /// Unsigned decimal number with $bits-bit integer for significant digits.
        impl Decimal<$UINT> {

            pos_const!($UINT, $bits, ZERO 0, ONE 1, TWO 2, THREE 3, FOUR 4, FIVE 5, SIX 6, SEVEN 7, EIGHT 8, NINE 9, TEN 10);

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
                Self::new(UnsignedDecimal::<$UINT>::from_scale(exp), Sign::NoSign)
            }

            /// __Normalize__ this decimal moving all significant trailing zeros into the exponent.
            ///
            /// # Examples
            ///
            /// ```
            /// use fastnum::{dec256, u256};
            ///
            /// let a = dec256!(-1234500);
            /// assert_eq!(a.significant_digits(), u256!(1234500));
            /// assert_eq!(a.fractional_digit_count(), 0);
            ///
            /// let b = a.normalized();
            /// assert_eq!(b.significant_digits(), u256!(12345));
            /// assert_eq!(b.fractional_digit_count(), -2);
            /// ```
            #[inline]
            pub const fn normalized(self) -> Self {
                Self::new(UnsignedDecimal::<$UINT>::normalized(self.value), self.sign)
            }

            ///
            /// Create string of this decimal in scientific notation.
            ///
            /// ```
            /// use fastnum::dec256;
            ///
            /// let n = dec256!(-12345678);
            /// assert_eq!(&n.to_scientific_notation(), "-1.2345678e7");
            /// ```
            #[inline]
            pub fn to_scientific_notation(&self) -> String {
                let mut output = String::new();
                self.write_scientific_notation(&mut output)
                    .expect("Could not write to string");
                output
            }

            ///
            /// Create string of this decimal in engineering notation.
            ///
            /// Engineering notation is scientific notation with the exponent
            /// coerced to a multiple of three
            ///
            /// ```
            /// use fastnum::dec256;
            /// let n = dec256!(-12345678);
            /// assert_eq!(&n.to_engineering_notation(), "-12.345678e6");
            /// ```
            #[inline]
            pub fn to_engineering_notation(&self) -> String {
                let mut output = String::new();
                self.write_engineering_notation(&mut output)
                    .expect("Could not write to string");
                output
            }
        }

        #[doc(hidden)]
        impl Decimal<$UINT> {
            #[inline]
            pub(crate) fn write_scientific_notation<W: fmt::Write>(
                &self,
                w: &mut W,
            ) -> fmt::Result {
                write!(w, "{}", self.sign)?;
                self.value.write_scientific_notation(w)
            }

            #[inline]
            pub(crate) fn write_engineering_notation<W: fmt::Write>(
                &self,
                w: &mut W,
            ) -> fmt::Result {
                write!(w, "{}", self.sign)?;
                self.value.write_engineering_notation(w)
            }
        }
    };
}

macro_impl!(U128, 128);
macro_impl!(U256, 256);
macro_impl!(U512, 512);
