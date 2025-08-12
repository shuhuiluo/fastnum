use crate::bint::{
    intrinsics::_ilog10_64,
    uint::{
        intrinsics::Intrinsics,
        math::utils::{as_u128, as_u64},
    },
    UInt,
};

#[inline(always)]
pub const fn ilog10<const N: usize>(n: UInt<N>) -> u32 {
    match N {
        1 => _ilog10_64(as_u64(n)),
        2 => as_u128(n).ilog10(),
        _ => ilog10_long(n),
    }
}

#[inline(always)]
const fn ilog10_long<const N: usize>(n: UInt<N>) -> u32 {
    let res = ((n.ilog2() + 1) * 1233) >> 12;
    if n.lt(&Intrinsics::<N>::unchecked_power_of_ten(res)) {
        res.saturating_sub(1)
    } else {
        res
    }
}
