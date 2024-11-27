use num_traits::{FromPrimitive, Num, One, Signed, ToPrimitive, Zero};
use std::ops::Neg;

use crate::decimal::{Decimal, ParseError, Sign};

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

impl<const N: usize> ToPrimitive for Decimal<N> {
    #[inline]
    fn to_isize(&self) -> Option<isize> {
        match self.sign {
            Sign::Minus => self
                .value
                .to_usize()
                .and_then(|n| 0_isize.checked_sub_unsigned(n)),
            Sign::NoSign | Sign::Plus => self.value.to_isize(),
        }
    }

    #[inline]
    fn to_i8(&self) -> Option<i8> {
        match self.sign {
            Sign::Minus => self
                .value
                .to_u8()
                .and_then(|n| 0_i8.checked_sub_unsigned(n)),
            Sign::NoSign | Sign::Plus => self.value.to_i8(),
        }
    }

    #[inline]
    fn to_i16(&self) -> Option<i16> {
        match self.sign {
            Sign::Minus => self
                .value
                .to_u16()
                .and_then(|n| 0_i16.checked_sub_unsigned(n)),
            Sign::NoSign | Sign::Plus => self.value.to_i16(),
        }
    }

    #[inline]
    fn to_i32(&self) -> Option<i32> {
        match self.sign {
            Sign::Minus => self
                .value
                .to_u32()
                .and_then(|n| 0_i32.checked_sub_unsigned(n)),
            Sign::NoSign | Sign::Plus => self.value.to_i32(),
        }
    }

    #[inline]
    fn to_i64(&self) -> Option<i64> {
        match self.sign {
            Sign::Minus => self
                .value
                .to_u64()
                .and_then(|n| 0_i64.checked_sub_unsigned(n)),
            Sign::NoSign | Sign::Plus => self.value.to_i64(),
        }
    }

    #[inline]
    fn to_i128(&self) -> Option<i128> {
        match self.sign {
            Sign::Minus => self
                .value
                .to_u128()
                .and_then(|n| 0_i128.checked_sub_unsigned(n)),
            Sign::NoSign | Sign::Plus => self.value.to_i128(),
        }
    }

    #[inline]
    fn to_usize(&self) -> Option<usize> {
        match self.sign {
            Sign::Minus => None,
            Sign::NoSign | Sign::Plus => self.value.to_usize(),
        }
    }

    #[inline]
    fn to_u8(&self) -> Option<u8> {
        match self.sign {
            Sign::Minus => None,
            Sign::NoSign | Sign::Plus => self.value.to_u8(),
        }
    }

    #[inline]
    fn to_u16(&self) -> Option<u16> {
        match self.sign {
            Sign::Minus => None,
            Sign::NoSign | Sign::Plus => self.value.to_u16(),
        }
    }

    #[inline]
    fn to_u32(&self) -> Option<u32> {
        match self.sign {
            Sign::Minus => None,
            Sign::NoSign | Sign::Plus => self.value.to_u32(),
        }
    }

    #[inline]
    fn to_u64(&self) -> Option<u64> {
        match self.sign {
            Sign::Minus => None,
            Sign::NoSign | Sign::Plus => self.value.to_u64(),
        }
    }

    #[inline]
    fn to_u128(&self) -> Option<u128> {
        match self.sign {
            Sign::Minus => None,
            Sign::NoSign | Sign::Plus => self.value.to_u128(),
        }
    }

    #[inline]
    fn to_f32(&self) -> Option<f32> {
        self.value.to_f32().map(|n| match self.sign {
            Sign::Minus => n.neg(),
            Sign::NoSign | Sign::Plus => n,
        })
    }

    #[inline]
    fn to_f64(&self) -> Option<f64> {
        self.value.to_f64().map(|n| match self.sign {
            Sign::Minus => n.neg(),
            Sign::NoSign | Sign::Plus => n,
        })
    }
}

impl<const N: usize> FromPrimitive for Decimal<N> {
    #[inline]
    fn from_isize(n: isize) -> Option<Self> {
        Some(Self::from(n))
    }

    #[inline]
    fn from_i8(n: i8) -> Option<Self> {
        Some(Self::from(n))
    }

    #[inline]
    fn from_i16(n: i16) -> Option<Self> {
        Some(Self::from(n))
    }

    #[inline]
    fn from_i32(n: i32) -> Option<Self> {
        Some(Self::from(n))
    }

    #[inline]
    fn from_i64(n: i64) -> Option<Self> {
        Some(Self::from(n))
    }

    #[inline]
    fn from_i128(n: i128) -> Option<Self> {
        Some(Self::from(n))
    }

    #[inline]
    fn from_usize(n: usize) -> Option<Self> {
        Some(Self::from(n))
    }

    #[inline]
    fn from_u8(n: u8) -> Option<Self> {
        Some(Self::from(n))
    }

    #[inline]
    fn from_u16(n: u16) -> Option<Self> {
        Some(Self::from(n))
    }

    #[inline]
    fn from_u32(n: u32) -> Option<Self> {
        Some(Self::from(n))
    }

    #[inline]
    fn from_u64(n: u64) -> Option<Self> {
        Some(Self::from(n))
    }

    #[inline]
    fn from_u128(n: u128) -> Option<Self> {
        Some(Self::from(n))
    }

    #[inline]
    fn from_f32(n: f32) -> Option<Self> {
        Self::try_from(n).ok()
    }

    #[inline]
    fn from_f64(n: f64) -> Option<Self> {
        Self::try_from(n).ok()
    }
}

impl<const N: usize> Signed for Decimal<N> {
    #[inline]
    fn abs(&self) -> Self {
        (*self).abs()
    }

    #[inline]
    fn abs_sub(&self, other: &Self) -> Self {
        if self.le(other) {
            Self::ZERO
        } else {
            *self - *other
        }
    }

    #[inline]
    fn signum(&self) -> Self {
        match self.sign {
            Sign::Plus => Self::ONE,
            Sign::NoSign => Self::ZERO,
            Sign::Minus => Self::ONE.neg(),
        }
    }

    #[inline]
    fn is_positive(&self) -> bool {
        self.is_positive()
    }

    #[inline]
    fn is_negative(&self) -> bool {
        self.is_negative()
    }
}
