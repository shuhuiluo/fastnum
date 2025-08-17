use crate::bint::{
    intrinsics::{DIGIT_POWERS_10, DIGIT_POWER_10},
    UInt,
};

type U<const N: usize> = UInt<N>;

#[derive(Copy, Clone)]
pub(super) struct TruncatedResult {
    pub(super) digits: u64,
    pub(super) exp: i32,
    pub(super) is_inexact: bool,
}

#[inline]
pub(super) const fn truncate<const N: usize>(mut digits: U<N>, exp: i32) -> TruncatedResult {
    let mut result = TruncatedResult {
        digits: 0,
        exp,
        is_inexact: false,
    };
    let mut extra_digit;

    while digits.bits() > u64::BITS {
        let gap = digits.decimal_digits() - DIGIT_POWER_10;
        let pow10 = if gap > DIGIT_POWER_10 {
            DIGIT_POWER_10
        } else {
            gap
        } as usize;
        debug_assert!(pow10 > 0 && pow10 <= DIGIT_POWER_10 as usize);

        (digits, extra_digit) = digits.div_rem_digit(DIGIT_POWERS_10[pow10]);

        if extra_digit != 0 {
            result.is_inexact = true;
        }

        result.exp += pow10 as i32;
    }

    result.digits = digits.digits()[0];
    result
}

/// Calculate a base 2 exponent from a decimal exponent.
/// This uses a pre-computed integer approximation for
/// log2(10), where 217706 / 2^16 is accurate for the
/// entire range of non-finite decimal exponents.
#[inline]
pub(super) const fn power(q: i32) -> i32 {
    (q.wrapping_mul(152_170 + 65536) >> 16) + 63
}
