use bnum::BInt;
use core::str::from_utf8_unchecked;

use crate::bint::{error::from_int_error_kind, convert::from_str::from_str_impl, ParseError, Int, doc};

from_str_impl!(Int, I, BInt);