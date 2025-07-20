use bnum::BInt;
use core::str::from_utf8_unchecked;

use crate::bint::{
    convert::from_str::from_str_impl, doc, error::from_int_error_kind, Int, ParseError,
};

from_str_impl!(Int, I, BInt);
