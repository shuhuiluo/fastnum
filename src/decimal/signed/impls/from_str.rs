use core::str::FromStr;

use crate::decimal::{signed::Decimal, ParseError};

impl<const N: usize> FromStr for Decimal<N> {
    type Err = ParseError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, ParseError> {
        Decimal::from_str(s)
    }
}
