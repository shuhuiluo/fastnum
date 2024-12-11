#![allow(clippy::excessive_precision)]
#![allow(clippy::approx_constant)]
#![allow(clippy::zero_prefixed_literal)]

pub(crate) mod common;

mod assertions;
mod cmp;
mod extras;
mod format;
mod from;
mod hash;
mod math;
mod parse;
mod round;
mod scale;
mod smoke;

#[cfg(feature = "numtraits")]
pub(crate) mod numtraits;
