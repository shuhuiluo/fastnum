use bnum::BInt;

use crate::bint::{doc, endian::endian_impl, Int};

endian_impl!(Int, I, BInt);
