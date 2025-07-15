mod float_const;

use num_traits::{ConstOne, ConstZero, FromPrimitive, Num, One, ToPrimitive, Zero};

use crate::decimal::{impls::numtraits::from_primitive_impl, Context, ParseError, UnsignedDecimal};

impl<const N: usize> One for UnsignedDecimal<N> {
    #[inline]
    fn one() -> Self {
        Self::ONE
    }
}

impl<const N: usize> ConstOne for UnsignedDecimal<N> {
    const ONE: Self = Self::ONE;
}

impl<const N: usize> Zero for UnsignedDecimal<N> {
    #[inline]
    fn zero() -> Self {
        Self::ZERO
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.is_zero()
    }
}

impl<const N: usize> ConstZero for UnsignedDecimal<N> {
    const ZERO: Self = Self::ZERO;
}

impl<const N: usize> Num for UnsignedDecimal<N> {
    type FromStrRadixErr = ParseError;

    #[inline]
    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        if radix != 10 {
            return Err(ParseError::InvalidRadix);
        }
        Self::from_str(str, Context::default())
    }
}

impl<const N: usize> ToPrimitive for UnsignedDecimal<N> {
    #[inline]
    fn to_isize(&self) -> Option<isize> {
        self.0.to_isize().ok()
    }

    #[inline]
    fn to_i8(&self) -> Option<i8> {
        self.0.to_i8().ok()
    }

    #[inline]
    fn to_i16(&self) -> Option<i16> {
        self.0.to_i16().ok()
    }

    #[inline]
    fn to_i32(&self) -> Option<i32> {
        self.0.to_i32().ok()
    }

    #[inline]
    fn to_i64(&self) -> Option<i64> {
        self.0.to_i64().ok()
    }

    #[inline]
    fn to_i128(&self) -> Option<i128> {
        self.0.to_i128().ok()
    }

    #[inline]
    fn to_usize(&self) -> Option<usize> {
        self.0.to_usize().ok()
    }

    #[inline]
    fn to_u8(&self) -> Option<u8> {
        self.0.to_u8().ok()
    }

    #[inline]
    fn to_u16(&self) -> Option<u16> {
        self.0.to_u16().ok()
    }

    #[inline]
    fn to_u32(&self) -> Option<u32> {
        self.0.to_u32().ok()
    }

    #[inline]
    fn to_u64(&self) -> Option<u64> {
        self.0.to_u64().ok()
    }

    #[inline]
    fn to_u128(&self) -> Option<u128> {
        self.0.to_u128().ok()
    }

    #[inline]
    fn to_f32(&self) -> Option<f32> {
        Some(self.0.to_f32())
    }

    #[inline]
    fn to_f64(&self) -> Option<f64> {
        Some(self.0.to_f64())
    }
}

from_primitive_impl!(
    UnsignedDecimal, U,
    from_u8 <- u8,
    from_u16 <- u16,
    from_u32 <- u32,
    from_u64 <- u64,
    from_usize <- usize,
    from_u128 <- u128 #TRY,

    from_i8 <- i8 #TRY,
    from_i16 <- i16 #TRY,
    from_i32 <- i32 #TRY,
    from_i64 <- i64 #TRY,
    from_isize <- isize #TRY,
    from_i128 <- i128 #TRY,

    from_f32 <- f32 #TRY,
    from_f64 <- f64 #TRY
);
