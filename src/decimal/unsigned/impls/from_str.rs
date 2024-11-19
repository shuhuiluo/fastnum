use core::str::FromStr;

use crate::decimal::{unsigned::UnsignedDecimal, ParseError};

impl<const N: usize> FromStr for UnsignedDecimal<N> {
    type Err = ParseError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, ParseError> {
        Self::from_str(s)
    }
}
