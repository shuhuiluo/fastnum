use bnum::BUint;

use crate::bint::{doc, endian::endian_impl, UInt};

endian_impl!(UInt, U, BUint);
