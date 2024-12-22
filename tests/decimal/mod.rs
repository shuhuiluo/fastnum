#![allow(clippy::excessive_precision)]
#![allow(clippy::approx_constant)]
#![allow(clippy::zero_prefixed_literal)]

pub(crate) mod common;

mod cmp;
mod extras;
mod format;
mod from;
mod hash;
mod math;
mod parse;
mod quantize;
mod quantum;
mod round;
mod smoke;
mod signals;

#[cfg(feature = "numtraits")]
pub(crate) mod numtraits;
