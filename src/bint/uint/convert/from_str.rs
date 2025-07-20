use bnum::BUint;
use core::str::from_utf8_unchecked;

use crate::bint::{
    convert::from_str::from_str_impl, doc, error::from_int_error_kind, ParseError, UInt,
};

from_str_impl!(UInt, U, BUint);
