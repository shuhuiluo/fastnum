pub const BITS: u32 = 64;

/// Sign bit
pub const SIGN_MASK: u64 = 0x8000_0000_0000_0000;

/// Exponent mask
pub const EXP_MASK: u64 = 0x7ff0_0000_0000_0000;

/// Mantissa mask
pub const MAN_MASK: u64 = 0x000f_ffff_ffff_ffff;

pub const MAN_MASK_NORMAL: u64 = 0x0010_0000_0000_0000;

pub const MANTISSA_DIGITS: u32 = f64::MANTISSA_DIGITS;

pub const MAX_EXP: i32 = f64::MAX_EXP;

pub const MIN_EXP: i32 = f64::MIN_EXP;

pub const BIAS_EXP: i16 = MAX_EXP as i16 - 1;

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
    let i = MAGIC_U64 - (n.to_bits() >> 1);
    let y = f64::from_bits(i);

    y * (THREE_HALFS - (x2 * y * y))
}
