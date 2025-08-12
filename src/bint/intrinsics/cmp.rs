#[inline(always)]
pub const fn min(lhs: u32, rhs: u32) -> u32 {
    if lhs < rhs {
        lhs
    } else {
        rhs
    }
}

#[allow(dead_code)]
#[inline(always)]
pub const fn max(lhs: u32, rhs: u32) -> u32 {
    if lhs > rhs {
        lhs
    } else {
        rhs
    }
}
