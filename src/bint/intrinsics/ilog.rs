use crate::bint::intrinsics::{_div_rem_64, DIGIT_POWERS_10};

#[inline(always)]
pub const fn _ilog10_64(n: u64) -> u32 {
    let res = ((n.ilog2() + 1) * 1233) >> 12;
    if n < DIGIT_POWERS_10[res as usize] {
        res.saturating_sub(1)
    } else {
        res
    }
}

#[inline(always)]
pub const fn _downscale_64(mut n: u64) -> (u64, u32) {
    let mut upscaled = 0;

    while n >= 10 {
        let (q, r) = _div_rem_64(n, 10);

        if r != 0 {
            break;
        }

        n = q;
        upscaled += 1;
    }

    (n, upscaled)
}
