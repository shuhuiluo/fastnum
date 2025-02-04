use crate::{
    decimal::{dec::math, Decimal},
    int::UInt,
};

/// Sign bit
pub const SIGN_MASK: u32 = 0x8000_0000;

/// Exponent mask
pub const EXP_MASK: u32 = 0x7f80_0000;

/// Mantissa mask
pub const MAN_MASK: u32 = 0x007f_ffff;

pub const MAN_MASK_NORMAL: u32 = 0x80_0000;

pub const MANTISSA_DIGITS: u32 = f32::MANTISSA_DIGITS;

pub const MAX_EXP: i32 = f32::MAX_EXP;

#[inline]
pub const fn to_bits(n: f32) -> u32 {
    #[allow(unsafe_code)]
    unsafe {
        core::mem::transmute(n)
    }
}

#[inline]
pub const fn from_bits(v: u32) -> f32 {
    #[allow(unsafe_code)]
    unsafe {
        core::mem::transmute(v)
    }
}

#[inline]
#[allow(clippy::eq_op)]
pub const fn is_nan(n: f32) -> bool {
    n != n
}

pub struct FloatConsts<const N: usize>;

impl<const N: usize> FloatConsts<N> {
    pub const MAX: Decimal<N> = math::consts::Consts::MAX_F32;
}

#[inline]
pub const fn uint<const N: usize>(digit: u32) -> UInt<N> {
    UInt::from_digit(digit as u64)
}
