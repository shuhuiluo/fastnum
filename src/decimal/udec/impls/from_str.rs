use core::str::FromStr;

use crate::decimal::{Context, ParseError, UnsignedDecimal};

impl<const N: usize> FromStr for UnsignedDecimal<N> {
    type Err = ParseError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, ParseError> {
        Self::from_str(s, Context::default())
    }
}
