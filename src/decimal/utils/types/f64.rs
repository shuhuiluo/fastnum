use crate::int::UInt;

/// Sign bit
pub const SIGN_MASK: u64 = 0x8000_0000_0000_0000;

/// Exponent mask
pub const EXP_MASK: u64 = 0x7ff0_0000_0000_0000;

/// Mantissa mask
pub const MAN_MASK: u64 = 0x000f_ffff_ffff_ffff;

pub const MAN_MASK_NORMAL: u64 = 0x0010_0000_0000_0000;

pub const MANTISSA_DIGITS: u32 = f64::MANTISSA_DIGITS;

pub const MAX_EXP: i32 = f64::MAX_EXP;

#[inline]
pub const fn to_bits(n: f64) -> u64 {
    #[allow(unsafe_code)]
    unsafe {
        core::mem::transmute(n)
    }
}

#[inline]
#[allow(clippy::eq_op)]
pub const fn is_nan(n: f64) -> bool {
    n != n
}

pub struct Subnormal<const N: usize> {}

impl<const N: usize> Subnormal<N> {
    pub const POW: u32 = (MAX_EXP - 2) as u32 + (MANTISSA_DIGITS - 1);
    pub const SUBNORMAL_BASE: Option<UInt<N>> = UInt::<N>::from_digit(5).checked_pow(Self::POW);
}

#[inline]
pub const fn uint<const N: usize>(digit: u64) -> UInt<N> {
    UInt::from_digit(digit)
}

#[cfg(not(any(feature = "std", feature = "libm")))]
core::compile_error!("Either feature \"std\" or \"libm\" must be enabled for this crate.");

#[cfg(all(not(feature = "std"), feature = "libm"))]
pub fn powi(x: f64, n: i32) -> f64 {
    libm::pow(x, n as f64)
}

#[cfg(feature = "std")]
pub fn powi(x: f64, n: i32) -> f64 {
    x.powi(n)
}
