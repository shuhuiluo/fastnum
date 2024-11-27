use crate::int::UInt;

/// Sign bit
pub const SIGN_MASK: u32 = 0x8000_0000;

/// Exponent mask
pub const EXP_MASK: u32 = 0x7f80_0000;

/// Mantissa mask
pub const MAN_MASK: u32 = 0x007f_ffff;

pub const MAN_MASK_NORMAL: u32 = 0x80_0000;

pub const MANTISSA_DIGITS: u32 = f32::MANTISSA_DIGITS;

pub const MAX_EXP: i64 = f32::MAX_EXP as i64;

#[inline]
pub const fn to_bits(n: f32) -> u32 {
    #[allow(unsafe_code)]
    unsafe {
        core::mem::transmute(n)
    }
}

#[inline]
#[allow(clippy::eq_op)]
pub const fn is_nan(n: f32) -> bool {
    n != n
}

pub struct Subnormal<const N: usize> {}

impl<const N: usize> Subnormal<N> {
    pub const POW: u32 = (MAX_EXP - 2) as u32 + (MANTISSA_DIGITS - 1);
    pub const SUBNORMAL_BASE: Option<UInt<N>> = UInt::<N>::from_digit(5).checked_pow(Self::POW);
}

#[inline]
pub const fn uint<const N: usize>(digit: u32) -> UInt<N> {
    UInt::from_digit(digit as u64)
}

#[cfg(not(any(feature = "std", feature = "libm")))]
core::compile_error!("Either feature \"std\" or \"libm\" must be enabled for this crate.");

#[cfg(all(not(feature = "std"), feature = "libm"))]
pub fn powi(x: f32, n: i32) -> f32 {
    libm::pow(x as f64, n as f64) as f32
}

#[cfg(feature = "std")]
pub fn powi(x: f32, n: i32) -> f32 {
    x.powi(n)
}