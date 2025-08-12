//! # Big Integers
//!
//! Under the hood [bnum](https://docs.rs/bnum/latest/bnum/) is currently used as the backend as most meeting the
//! requirements.
//! Subsequently, the implementation can be replaced in favor of its own
//! implementation, which enables `SIMD`.

// TODO
pub(crate) mod bits;
pub(crate) mod carrying;
pub(crate) mod checked;
pub(crate) mod cmp;
pub(crate) mod consts;
pub(crate) mod convert;
pub(crate) mod doc;
pub(crate) mod endian;
pub(crate) mod impls;
pub(crate) mod intrinsics;
pub(crate) mod math;
pub(crate) mod num;
pub(crate) mod overflowing;
pub(crate) mod saturating;
pub(crate) mod strict;
pub(crate) mod utils;
pub(crate) mod widening;
pub(crate) mod wrapping;

#[macro_use]
mod macros;

#[cfg(debug_assertions)]
mod assertions;

mod error;
mod int;
mod uint;

pub use error::ParseError;
pub use int::Int;
pub use uint::UInt;

use crate::bint::doc::int_type_doc;

macro_rules! int_types {
    ( $($bits: literal $u: ident $s: ident; ) *)  => {
        $(
            #[doc = int_type_doc!($bits, "unsigned")]
            pub type $u = UInt::<{$bits / 64}>;

            #[doc = int_type_doc!($bits, "signed")]
            pub type $s = Int::<{$bits / 64}>;
        )*
    };
}

int_types!(
    64 U64 I64;
    128 U128 I128;
    256 U256 I256;
    512 U512 I512;
    1024 U1024 I1024;
    // 2048 U2048 I2048;
    // 4096 U4096 I4096;
    // 8192 U8192 I8192;
);
