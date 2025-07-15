use core::str::FromStr;

use crate::bint::{UInt, ParseError};

impl<const N: usize> FromStr for UInt<N> {
    type Err = ParseError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, ParseError> {
        UInt::from_str(s)
    }
}
