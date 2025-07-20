use crate::decimal::dec::convert::to_float::lemire::table::{
    LARGEST_POWER_OF_FIVE, POWER_OF_FIVE_128, SMALLEST_POWER_OF_FIVE,
};

#[inline]
pub(super) const fn full_multiplication(a: u64, b: u64) -> (u64, u64) {
    let r = (a as u128) * (b as u128);
    (r as u64, (r >> 64) as u64)
}

// This will compute or rather approximate w * 5**q and return a pair of 64-bit
// words approximating the result, with the "high" part corresponding to the
// most significant bits and the low part corresponding to the least significant
// bits.
pub(super) const fn compute_product_approx(q: i32, w: u64, precision: usize) -> (u64, u64) {
    debug_assert!(q >= SMALLEST_POWER_OF_FIVE);
    debug_assert!(q <= LARGEST_POWER_OF_FIVE);
    debug_assert!(precision <= 64);

    let mask = if precision < 64 {
        0xFFFF_FFFF_FFFF_FFFF_u64 >> precision
    } else {
        0xFFFF_FFFF_FFFF_FFFF_u64
    };

    // 5^q < 2^64, then the multiplication always provides an exact value.
    // That means whenever we need to round ties to even, we always have
    // an exact value.
    let index = (q - SMALLEST_POWER_OF_FIVE) as usize;
    let (lo5, hi5) = POWER_OF_FIVE_128[index];
    // Only need one multiplication as long as there is 1 zero but
    // in the explicit mantissa bits, +1 for the hidden bit, +1 to
    // determine the rounding direction, +1 for if the computed
    // product has a leading zero.
    let (mut first_lo, mut first_hi) = full_multiplication(w, lo5);
    if first_hi & mask == mask {
        // Need to do a second multiplication to get better precision
        // for the lower product. This will always be exact
        // where q is < 55, since 5^55 < 2^128. If this wraps,
        // then we need to round up the hi product.
        let (_, second_hi) = full_multiplication(w, hi5);
        first_lo = first_lo.wrapping_add(second_hi);
        if second_hi > first_lo {
            first_hi += 1;
        }
    }
    (first_lo, first_hi)
}
