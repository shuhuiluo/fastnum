//! # Big Integers
//!
//! For big integers this crate provides integer types of arbitrary fixed size
//! which behave exactly like Rust's internal primitive integer types (`u8`,
//! `i8`, `u16`, `i16`, etc.):
//!
//! | Unit                  | Bits | Signed | Min               | Max                | Helper Macro                |
//! |-----------------------|------|--------|-------------------|--------------------|-----------------------------|
//! | [I128](crate::I128)   | 128  |   +    | -2<sup>127</sup>  | 2<sup>127</sup>-1  | [`i128!(1)`](crate::i128)   |
//! | [U128](crate::U128)   | 128  |        | 0                 | 2<sup>128</sup>    | [`u128!(1)`](crate::u128)   |
//! | [I256](crate::I256)   | 256  |   +    | -2<sup>255</sup>  | 2<sup>255</sup>-1  | [`i256!(1)`](crate::i256)   |
//! | [U256](crate::U256)   | 256  |        | 0                 | 2<sup>256</sup>    | [`u256!(1)`](crate::u256)   |
//! | [I512](crate::I512)   | 512  |   +    | -2<sup>511</sup>  | 2<sup>511</sup>-1  | [`i512!(1)`](crate::i512)   |
//! | [U512](crate::U512)   | 512  |        | 0                 | 2<sup>512</sup>    | [`u512!(1)`](crate::u512)   |
//! | [I1024](crate::I1024) | 1024 |   +    | -2<sup>1023</sup> | 2<sup>1023</sup>-1 | [`i1024!(1)`](crate::i1024) |
//! | [U1024](crate::U1024) | 1024 |        | 0                 | 2<sup>1024</sup>   | [`u1024!(1)`](crate::u1024) |
//! | [I2048](crate::I2048) | 2048 |   +    | -2<sup>2047</sup> | 2<sup>2047</sup>-1 | [`i2048!(1)`](crate::i2048) |
//! | [U2048](crate::U2048) | 2048 |        | 0                 | 2<sup>2048</sup>   | [`u2048!(1)`](crate::u2048) |
//! | [I4096](crate::I4096) | 4096 |   +    | -2<sup>4095</sup> | 2<sup>4095</sup>-1 | [`i4096!(1)`](crate::i4096) |
//! | [U4096](crate::U4096) | 4096 |        | 0                 | 2<sup>4096</sup>   | [`u4096!(1)`](crate::u4096) |
//! | [I8192](crate::I8192) | 8192 |   +    | -2<sup>8191</sup> | 2<sup>8191</sup>-1 | [`i8192!(1)`](crate::i8192) |
//! | [U8192](crate::U8192) | 8192 |        | 0                 | 2<sup>8192</sup>   | [`u8192!(1)`](crate::u8192) |
//!
//! Nearly all methods defined on Rust's signed and unsigned primitive integers
//! are defined `fastnum`'s signed and unsigned integers.
//!
//! Under the hood [bnum](https://docs.rs/bnum/latest/bnum/) is currently used as the backend as most meeting the
//! requirements.
//! Subsequently, the implementation can be replaced in favor of its own
//! implementation, which enables `SIMD`.
//!
//! Unsigned integers are stored as an array of digits (primitive unsigned
//! integers) of length `N`. This means all `fastnum` integers can be stored on
//! the stack, as they are fixed size. Signed integers are simply stored as an
//! unsigned integer in two's complement.
//!
//! ## Example
//!
//! ```
//! // Calculate the `n`th Fibonacci number, using the type alias `U512`.
//!
//! use fastnum::U512; // `U512` is a type alias for a `BUint` which contains 8 `u64` digits
//!
//! // Calculate the nth Fibonacci number
//! fn fibonacci(n: usize) -> U512 {
//!     let mut f_n: U512 = U512::ZERO; // or `U512::from(0u8)`
//!     let mut f_n_next: U512 = U512::ONE; // or `U512::from(1u8)`
//!
//!     for _ in 0..n {
//!         let temp = f_n_next;
//!         f_n_next += f_n;
//!         f_n = temp;
//!     }
//!
//!     f_n
//! }
//!
//! let n = 100;
//! let f_n = fibonacci(n);
//!
//! println!("The {}th Fibonacci number is {}", n, f_n);
//! // Prints "The 100th Fibonacci number is 354224848179261915075"
//!
//! assert_eq!(f_n, U512::from_str_radix("354224848179261915075", 10).unwrap());
//! ```

mod error;

#[macro_use]
mod macros;

#[doc(hidden)]
pub mod parse;

pub use bnum::types::*;
pub use error::ParseError;
