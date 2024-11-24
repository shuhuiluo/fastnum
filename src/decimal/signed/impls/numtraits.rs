use num_traits::{Num, One, Zero};

use crate::decimal::{Decimal, ParseError};

impl<const N: usize> One for Decimal<N> {
    #[inline]
    fn one() -> Self {
        Self::ONE
    }
}

impl<const N: usize> Zero for Decimal<N> {
    #[inline]
    fn zero() -> Self {
        Self::ZERO
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.is_zero()
    }
}

impl<const N: usize> Num for Decimal<N> {
    type FromStrRadixErr = ParseError;

    #[inline]
    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        if radix != 10 {
            return Err(ParseError::InvalidRadix);
        }
        Self::from_str(str)
    }
}
