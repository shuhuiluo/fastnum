//! 
//! # Decimal
//!
//! `fastnum` provides a several decimal numbers suitable for financial
//! calculations that require significant integral and fractional digits with no
//! round-off errors (such as 0.1 + 0.2 ≠ 0.3).
//!
//! | Decimal               | Integer             | Bits | Signed | Max significant | Helper macro                       |
//! |-----------------------|---------------------|------|--------|-----------------|------------------------------------|
//! | [D128](crate::D128)   | [U128](crate::U128) | 128  | +      | 2<sup>128</sup> | [`dec128!(0.1)`](crate::dec128!)   |
//! | [UD128](crate::UD128) | [U128](crate::U128) | 128  |        | 2<sup>128</sup> | [`udec128!(0.1)`](crate::udec128!) |
//! | [D256](crate::D256)   | [U256](crate::U256) | 256  | +      | 2<sup>256</sup> | [`dec256!(0.1)`](crate::dec256!)   |
//! | [UD256](crate::UD256) | [U256](crate::U256) | 256  |        | 2<sup>256</sup> | [`udec256!(0.1)`](crate::udec256!) |
//! | [D512](crate::D512)   | [U512](crate::U512) | 512  | +      | 2<sup>512</sup> | [`dec512!(0.1)`](crate::dec512!)   |
//! | [UD512](crate::UD512) | [U512](crate::U512) | 512  |        | 2<sup>512</sup> | [`udec512!(0.1)`](crate::udec512!) |
//!
//!
//! Under the hood any `[D|UD]N` decimal type consists of N-bit big unsigned
//! integer, paired with a 64-bit signed integer scaling factor which determines
//! the position of the decimal point and sign (for signed types only).
//! Therefore, the precision is not actually arbitrary, but limited to
//! 2<sup>63</sup> decimal places. Because of this representation,
//! trailing zeros are preserved and may be exposed when in string form. These
//! can be truncated using the normalize or round_dp functions.
//!
//! Thus, fixed-point numbers are trivially copyable and do not require any
//! dynamic allocation. This allows you to get additional performance gains by
//! eliminating not only dynamic allocation, like such, but also will get rid of
//! one indirect addressing, which improves cache-friendliness and reduces the
//! CPU load.
//!
//! Common numerical operations are overloaded, so we can treat them
//! the same way we treat other numbers.
//!
//! It is not recommended to convert a floating point number to a decimal
//! directly, as the floating point representation may be unexpected.
//!
//! # Example
//!
//! ```
//! use fastnum::UD256;
//! use std::str::FromStr;
//!
//! let input = "0.1";
//! let dec = UD256::from_str(&input).unwrap();
//! let float = f32::from_str(&input).unwrap();
//!
//! println!("Input ({}) with decimals: {} vs {})", input, dec, float);
//! ```

pub mod signed;
pub mod unsigned;

#[cfg(feature = "test-util")]
#[doc(hidden)]
pub mod extras;

#[cfg(not(feature = "test-util"))]
pub(crate) mod extras;

pub(crate) mod format;
pub(crate) mod math;

mod error;
mod rounding;

#[macro_use]
mod macros;

pub use error::ParseError;
pub use rounding::RoundingMode;
