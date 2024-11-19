//! # Decimal numbers

#[cfg(feature = "test-util")]
#[doc(hidden)]
pub mod extras;

#[cfg(not(feature = "test-util"))]
pub(crate) mod extras;

pub(crate) mod doc;
pub(crate) mod format;
pub(crate) mod math;
pub(crate) mod round;
pub(crate) mod signed;
pub(crate) mod unsigned;
pub(crate) mod utils;

mod error;
mod overflow;

#[macro_use]
mod macros;

pub use error::ParseError;
pub use math::{ArithmeticError, ArithmeticPolicy, DecimalResult};
pub use overflow::OverflowPolicy;
pub use round::{RoundingMode, RoundingPolicy};
pub use signed::{Decimal, Sign};
pub use unsigned::UnsignedDecimal;

use crate::decimal::doc::decimal_type_doc;

macro_rules! uint_types {
    ( $($bits: literal $u: ident $s: ident; ) *)  => {
        $(
            #[doc = decimal_type_doc!($bits, "unsigned")]
            pub type $u = unsigned::UnsignedDecimal::<{$bits / 64}>;

            #[doc = decimal_type_doc!($bits, "signed")]
            pub type $s = signed::Decimal::<{$bits / 64}>;
        )*
    };
}

uint_types!(
    128 UD128 D128;
    256 UD256 D256;
    512 UD512 D512;
    1024 UD1024 D1024;
    2048 UD2048 D2048;
    4096 UD4096 D4096;
    8192 UD8192 D8192;
);
