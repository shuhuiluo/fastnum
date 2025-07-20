pub const BITS: u32 = 32;

/// Sign bit
pub const SIGN_MASK: u32 = 0x8000_0000;

/// Exponent mask
pub const EXP_MASK: u32 = 0x7f80_0000;

/// Mantissa mask
pub const MAN_MASK: u32 = 0x007f_ffff;

pub const MAN_MASK_NORMAL: u32 = 0x80_0000;

pub const MANTISSA_DIGITS: u32 = f32::MANTISSA_DIGITS;

pub const MAX_EXP: i32 = f32::MAX_EXP;

pub const MIN_EXP: i32 = f32::MIN_EXP;
