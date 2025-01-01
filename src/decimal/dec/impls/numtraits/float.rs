use core::num::FpCategory;
use num_traits::{Float, ToPrimitive};

use crate::decimal::{Decimal, RoundingMode};

impl<const N: usize> Float for Decimal<N> {
    #[inline]
    fn nan() -> Self {
        Self::NAN
    }

    #[inline]
    fn infinity() -> Self {
        Self::INFINITY
    }

    #[inline]
    fn neg_infinity() -> Self {
        Self::NEG_INFINITY
    }

    #[inline]
    fn neg_zero() -> Self {
        Self::ZERO.neg()
    }

    #[inline]
    fn min_value() -> Self {
        Self::MIN
    }

    #[inline]
    fn min_positive_value() -> Self {
        Self::MIN_POSITIVE
    }

    #[inline]
    fn epsilon() -> Self {
        Self::EPSILON
    }

    #[inline]
    fn max_value() -> Self {
        Self::MAX
    }

    #[inline]
    fn is_nan(self) -> bool {
        Self::is_nan(&self)
    }

    #[inline]
    fn is_infinite(self) -> bool {
        Self::is_infinite(&self)
    }

    #[inline]
    fn is_finite(self) -> bool {
        Self::is_finite(&self)
    }

    #[inline]
    fn is_normal(self) -> bool {
        Self::is_normal(&self)
    }

    #[inline]
    fn is_subnormal(self) -> bool {
        Self::is_subnormal(&self)
    }

    #[inline]
    fn classify(self) -> FpCategory {
        Self::classify(&self)
    }

    #[inline]
    fn floor(self) -> Self {
        self.with_rounding_mode(RoundingMode::Floor).round(0)
    }

    #[inline]
    fn ceil(self) -> Self {
        self.with_rounding_mode(RoundingMode::Ceiling).round(0)
    }

    #[inline]
    fn round(self) -> Self {
        self.round(0)
    }

    #[inline]
    fn trunc(self) -> Self {
        self.with_rounding_mode(RoundingMode::Down).round(0)
    }

    #[inline]
    fn fract(self) -> Self {
        self - self.trunc()
    }

    #[inline]
    fn abs(self) -> Self {
        self.abs()
    }

    #[inline]
    fn signum(self) -> Self {
        Self::signum(&self)
    }

    #[inline]
    fn is_sign_positive(self) -> bool {
        Self::is_sign_positive(&self)
    }

    #[inline]
    fn is_sign_negative(self) -> bool {
        Self::is_sign_negative(&self)
    }

    #[inline]
    fn mul_add(self, a: Self, b: Self) -> Self {
        todo!()
    }

    #[inline]
    fn recip(self) -> Self {
        self.recip()
    }

    #[inline]
    fn powi(self, n: i32) -> Self {
        self.powi(n)
    }

    #[inline]
    fn powf(self, n: Self) -> Self {
        todo!()
    }

    #[inline]
    fn sqrt(self) -> Self {
        todo!()
    }

    #[inline]
    fn exp(self) -> Self {
        todo!()
    }

    #[inline]
    fn exp2(self) -> Self {
        todo!()
    }

    #[inline]
    fn ln(self) -> Self {
        todo!()
    }

    #[inline]
    fn log(self, base: Self) -> Self {
        todo!()
    }

    #[inline]
    fn log2(self) -> Self {
        todo!()
    }

    #[inline]
    fn log10(self) -> Self {
        todo!()
    }

    #[inline]
    fn to_degrees(self) -> Self {
        self.to_degrees()
    }

    #[inline]
    fn to_radians(self) -> Self {
        self.to_radians()
    }

    #[inline]
    fn max(self, other: Self) -> Self {
        self.max(other)
    }

    #[inline]
    fn min(self, other: Self) -> Self {
        self.min(other)
    }

    #[inline]
    fn clamp(self, min: Self, max: Self) -> Self {
        self.clamp(min, max)
    }

    #[inline]
    fn abs_sub(self, other: Self) -> Self {
        todo!()
    }

    #[inline]
    fn cbrt(self) -> Self {
        todo!()
    }

    #[inline]
    fn hypot(self, other: Self) -> Self {
        todo!()
    }

    #[inline]
    fn sin(self) -> Self {
        todo!()
    }

    #[inline]
    fn cos(self) -> Self {
        todo!()
    }

    #[inline]
    fn tan(self) -> Self {
        todo!()
    }

    #[inline]
    fn asin(self) -> Self {
        todo!()
    }

    #[inline]
    fn acos(self) -> Self {
        todo!()
    }

    #[inline]
    fn atan(self) -> Self {
        todo!()
    }

    #[inline]
    fn atan2(self, other: Self) -> Self {
        todo!()
    }

    #[inline]
    fn sin_cos(self) -> (Self, Self) {
        todo!()
    }

    #[inline]
    fn exp_m1(self) -> Self {
        todo!()
    }

    #[inline]
    fn ln_1p(self) -> Self {
        todo!()
    }

    #[inline]
    fn sinh(self) -> Self {
        todo!()
    }

    #[inline]
    fn cosh(self) -> Self {
        todo!()
    }

    #[inline]
    fn tanh(self) -> Self {
        todo!()
    }

    #[inline]
    fn asinh(self) -> Self {
        todo!()
    }

    #[inline]
    fn acosh(self) -> Self {
        todo!()
    }

    #[inline]
    fn atanh(self) -> Self {
        todo!()
    }

    #[inline]
    fn integer_decode(self) -> (u64, i16, i8) {
        self.to_f64().unwrap_or(f64::NAN).integer_decode()
    }

    #[inline]
    fn copysign(self, sign: Self) -> Self {
        todo!()
    }
}
