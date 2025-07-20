use crate::bint::{uint::intrinsics::Intrinsics, UInt};

#[inline(always)]
pub const fn ilog10<const N: usize>(n: UInt<N>) -> u32 {
    let res = ((n.ilog2() + 1) * 1233) >> 12;
    if n.lt(&Intrinsics::<N>::unchecked_power_of_ten(res)) {
        res.saturating_sub(1)
    } else {
        res
    }
}
