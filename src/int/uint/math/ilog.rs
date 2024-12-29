use crate::int::{intrinsics::Intrinsics, UInt};

#[inline(always)]
/// Find integer log<sub>10</sub>(x) of an integer.
///
/// By the relationship:
/// _log<sub>10</sub>(x) = log<sub>2</sub>(x)/log<sub>2</sub>(10)_,
/// we can compute the _log<sub>10</sub>(x)_ as `ilog2(x)` multiplied by
///
/// _1/log<sub>2</sub>(10)_, which is approximately `1233/4096`, or `1233`
/// followed by a right shift of `12`.
///
/// _((`ilog2`(x) + 1) * 1233) >> 12_
///
/// Adding one is needed because the `ilog2()` rounds down. Finally, since the
/// resulting value is only an approximation that may be off by one, the exact
/// value is found by subtracting `1` if `x < PowersOf10[res]` (lookup table).
/// This method takes `6` more operations than `ilog2()`. It may be sped up (on
/// machines with fast memory access) by modifying the log base 2 table-lookup
/// method above so that the entries hold what is computed for t (that is, pre
/// `-add`, `-mulitply`, and `-shift`).
pub const fn ilog10<const N: usize>(n: UInt<N>) -> u32 {
    let res = ((n.ilog2() + 1) * 1233) >> 12;
    if n.lt(&Intrinsics::<N>::POWERS_OF_TEN.lookup(res)) {
        res.saturating_sub(1)
    } else {
        res
    }
}
