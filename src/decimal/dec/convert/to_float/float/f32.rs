use crate::decimal::dec::convert::to_float::float::common::common_float_impl;

common_float_impl!(f32);

pub const MIN_10_EXP_REAL: i32 = -65;

/// Round-to-even only happens for negative values of q
/// when q ≥ −4 in the 64-bit case and when q ≥ −17 in
/// the 32-bitcase.
///
/// When q ≥ 0,we have that 5^q ≤ 2m+1. In the 64-bit case,we
/// have 5^q ≤ 2m+1 ≤ 2^54 or q ≤ 23. In the 32-bit case,we have
/// 5^q ≤ 2m+1 ≤ 2^25 or q ≤ 10.
///
/// When q < 0, we have w ≥ (2m+1)×5^−q. We must have that w < 2^64
/// so (2m+1)×5^−q < 2^64. We have that 2m+1 > 2^53 (64-bit case)
/// or 2m+1 > 2^24 (32-bit case). Hence,we must have 2^53×5^−q < 2^64
/// (64-bit) and 2^24×5^−q < 2^64 (32-bit). Hence we have 5^−q < 2^11
/// or q ≥ −4 (64-bit case) and 5^−q < 2^40 or q ≥ −17 (32-bitcase).
///
/// Thus we have that we only need to round ties to even when
/// we have that q ∈ [−4,23](in the 64-bit case) or q∈[−17,10]
/// (in the 32-bit case). In both cases,the power of five(5^|q|)
/// fits in a 64-bit word.
pub const MIN_EXPONENT_ROUND_TO_EVEN: i32 = -17;
pub const MAX_EXPONENT_ROUND_TO_EVEN: i32 = 10;

#[inline(always)]
pub const fn pow10_fast_path(exponent: usize) -> f32 {
    const TABLE: [f32; 16] = [
        1e0, 1e1, 1e2, 1e3, 1e4, 1e5, 1e6, 1e7, 1e8, 1e9, 1e10, 0., 0., 0., 0., 0.,
    ];
    TABLE[exponent & 15]
}

#[inline(always)]
pub const fn float(mantissa: u64, biased_exp: i32) -> f32 {
    let mut mantissa = ((mantissa & 0xFFFFFFFF) as u32) & MAN_MASK;
    mantissa |= (biased_exp as u32) << (MANTISSA_DIGITS - 1);
    f32::from_bits(mantissa)
}
