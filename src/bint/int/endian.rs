use bnum::BInt;

use crate::bint::{endian::endian_impl, Int, doc};

endian_impl!(Int, I, BInt);