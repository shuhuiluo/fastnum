use crate::{
    decimal::{dec::math, Decimal},
    int::UInt,
};

/// Sign bit
pub const SIGN_MASK: u64 = 0x8000_0000_0000_0000;

/// Exponent mask
pub const EXP_MASK: u64 = 0x7ff0_0000_0000_0000;

/// Mantissa mask
pub const MAN_MASK: u64 = 0x000f_ffff_ffff_ffff;

pub const MAN_MASK_NORMAL: u64 = 0x0010_0000_0000_0000;

pub const MANTISSA_DIGITS: u32 = f64::MANTISSA_DIGITS;

pub const MAX_EXP: i32 = f64::MAX_EXP;

pub const BIAS_EXP: i16 = MAX_EXP as i16 - 1;

#[inline]
pub const fn to_bits(n: f64) -> u64 {
    #[allow(unsafe_code)]
    unsafe {
        core::mem::transmute(n)
    }
}

#[inline]
pub const fn from_bits(v: u64) -> f64 {
    #[allow(unsafe_code)]
    unsafe {
        core::mem::transmute(v)
    }
}

#[inline]
#[allow(clippy::eq_op)]
pub const fn is_nan(n: f64) -> bool {
    n != n
}

pub struct FloatConsts<const N: usize>;

impl<const N: usize> FloatConsts<N> {
    pub const MAX: Decimal<N> = math::consts::Consts::MAX_F64;
}

#[inline]
pub const fn uint<const N: usize>(digit: u64) -> UInt<N> {
    UInt::from_digit(digit)
}

#[inline]
pub const fn sqrt(n: f64) -> f64 {
    1. / inv_sqrt(n)
}

#[inline]
const fn inv_sqrt(n: f64) -> f64 {
    if n.is_sign_negative() {
        return f64::NAN;
    } else if n == f64::INFINITY {
        return 0.0;
    } else if n < f64::MIN_POSITIVE {
        return f64::INFINITY;
    }

    const MAGIC_U64: u64 = 0x5fe6ec85e7de30da;
    const THREE_HALFS: f64 = 1.5;
    let x2 = n * 0.5;
    #[allow(unsafe_code)]
    let i = MAGIC_U64 - (unsafe { core::mem::transmute::<f64, u64>(n) } >> 1);
    #[allow(unsafe_code)]
    let y: f64 = unsafe { core::mem::transmute(i) };

    y * (THREE_HALFS - (x2 * y * y))
}
