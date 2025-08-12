//! Division historically is the most complex operation on CPUs and all
//! guidelines suggest avoiding the division at all costs.
//!
//! Division u128 by 64-bit integer use `__udivti3` function.
//!
//! `udivti3` is an unsigned division of TI (128 bits) integers, last ‘3′ means
//! that it has 3 arguments including the return value. Also, there is a
//! function `__udivmodti4` which computes the divisor and the remainder
//! (division and modulo operation) and it has 4 arguments including the
//! returning value. These functions are a part of runtime libraries which
//! compiler provide by default. In LLVM, `__udivti3` is a simple alias to
//! `__udivmodti4`.
//!
//! Unfortunately native dividing the 128-bit number by a 64-bit number is very
//! slow. About 90-150 nanoseconds O_O !
//!
//! On ARM64 everything is very bad:
//! https://www.reddit.com/r/rust/comments/1gy6tae/rust_slower_on_m4_pro/
//!
//! Links:
//!
//! https://danlark.org/2020/06/14/128-bit-division/
//!
//! https://github.com/ridiculousfish/libdivide
//!
//! https://ridiculousfish.com/blog/

#[inline(always)]
pub const fn _div_64(dividend: u64, divisor: u64) -> u64 {
    debug_assert!(divisor != 0);

    if dividend < divisor {
        0
    } else if divisor == 1 {
        dividend
    } else if divisor == dividend {
        1
    } else {
        dividend / divisor
    }
}

#[inline(always)]
pub const fn _div_rem_64(dividend: u64, divisor: u64) -> (u64, u64) {
    debug_assert!(divisor != 0);

    if dividend < divisor {
        (0, dividend)
    } else if divisor == 1 {
        (dividend, 0)
    } else if divisor == dividend {
        (1, 0)
    } else {
        let q = dividend / divisor;
        let r = dividend % divisor;
        (q, r)
    }
}

/// Divides a 128-bit uint {low: u64, high: u64} by a 64-bit uint {den}.
/// The result must fit in 64 bits.
///
/// Returns the (quotient, remainder).
///
/// This is a port of `libdivide_128_div_64_to_64`
/// from [`libdivide`](https://github.com/ridiculousfish/libdivide).
#[inline]
#[allow(clippy::needless_late_init)]
pub const fn _div_rem_128_64(mut low: u64, mut high: u64, mut den: u64) -> (u64, u64) {
    debug_assert!(high < den);

    // We work in base 2^32.
    // u32 holds a single digit.
    // u64 holds two digits.
    // Our numerator is conceptually [num3, num2, num1, num0].
    // Our denominator is [den1, den0].
    const B: u64 = 1 << 32;

    // The high and low digits of our computed quotient.
    let q1: u32;
    let q0: u32;

    // The normalization shift factor.
    let shift: u32;

    // The high and low digits of our denominator (after normalizing).
    // Also, the low 2 digits of our numerator (after normalizing).
    let den1: u32;
    let den0: u32;
    let num1: u32;
    let num0: u32;

    // A partial remainder.
    let rem;

    // The estimated quotient, and its corresponding remainder (unrelated to true
    // remainder).
    let mut qhat: u64;
    let mut rhat: u64;

    // Variables used to correct the estimated quotient.
    let mut c1: u64;
    let mut c2: u64;

    // Determine the normalization factor. We multiply den by this, so that its
    // leading digit is at least half b. In binary this means just shifting left
    // by the number of leading zeros, so that there's a 1 in the MSB.
    // We also shift numer by the same amount. This cannot overflow because `high` <
    // den. The expression (-shift & 63) is the same as (64 - shift), except it
    // avoids the UB of shifting by 64. The funny bitwise 'and' ensures that
    // `low` does not get shifted into `high` if shift is 0.
    shift = den.leading_zeros();
    den <<= shift;
    high <<= shift;
    high |= (low >> (-(shift as i32) & 63)) & (-(shift as i64) >> 63) as u64;
    low <<= shift;

    // Extract the low digits of the numerator and both digits of the denominator.
    num1 = (low >> 32) as u32;
    num0 = (low & 0xFFFFFFFF) as u32;
    den1 = (den >> 32) as u32;
    den0 = (den & 0xFFFFFFFF) as u32;

    // We wish to compute q1 = [n3 n2 n1] / [d1 d0].
    // Estimate q1 as [n3 n2] / [d1], and then correct it.
    // Note while qhat may be 2 digits, q1 is always 1 digit.
    qhat = high / den1 as u64;
    rhat = high % den1 as u64;
    c1 = qhat * den0 as u64;
    c2 = rhat * B + num1 as u64;
    if c1 > c2 {
        if c1 - c2 > den {
            qhat -= 2;
        } else {
            qhat -= 1;
        }
    }

    q1 = qhat as u32;

    // Compute the true (partial) remainder.
    rem = high
        .wrapping_mul(B)
        .wrapping_add(num1 as u64)
        .wrapping_sub((q1 as u64).wrapping_mul(den));

    // We wish to compute q0 = [rem1 rem0 n0] / [d1 d0].
    // Estimate q0 as [rem1 rem0] / [d1] and correct it.
    qhat = rem / den1 as u64;
    rhat = rem % den1 as u64;
    c1 = qhat * den0 as u64;
    c2 = rhat * B + num0 as u64;
    if c1 > c2 {
        if c1 - c2 > den {
            qhat -= 2;
        } else {
            qhat -= 1;
        }
    }
    q0 = qhat as u32;

    (
        ((q1 as u64) << 32) | q0 as u64,
        (rem.wrapping_mul(B)
            .wrapping_add(num0 as u64)
            .wrapping_sub((q0 as u64).wrapping_mul(den)))
            >> shift,
    )
}
