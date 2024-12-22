//! # Big Integers
//!
//! Under the hood [bnum](https://docs.rs/bnum/latest/bnum/) is currently used as the backend as most meeting the
//! requirements.
//! Subsequently, the implementation can be replaced in favor of its own
//! implementation, which enables `SIMD`.

#[macro_use]
mod macros;

#[cfg(debug_assertions)]
mod assertions;

mod doc;
mod error;
#[allow(clippy::module_inception)]
mod int;
mod uint;

#[doc(hidden)]
pub mod parse;

pub use int::*;
pub use uint::*;

pub use error::ParseError;
