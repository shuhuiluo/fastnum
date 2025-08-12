#[inline(always)]
pub const fn _widening_mul_64(a: u64, b: u64) -> (u64, u64) {
    let result = (a as u128) * (b as u128);
    (result as u64, (result >> 64) as u64)
}

#[allow(unsafe_code)]
#[inline(always)]
pub const unsafe fn _unchecked_mul_64(a: u64, b: u64) -> u64 {
    a.unchecked_mul(b)
}

#[inline(always)]
pub const fn _carrying_mul_64(multiplier: u64, multiplicand: u64, carry: u64) -> (u64, u64) {
    _carrying_mul_add_64(multiplier, multiplicand, carry, 0)
}

// TODO
#[inline]
pub const fn _carrying_mul_add_64(
    multiplier: u64,
    multiplicand: u64,
    addend: u64,
    carry: u64,
) -> (u64, u64) {
    let prod = carry as u128 + addend as u128 + (multiplier as u128) * (multiplicand as u128);
    (prod as u64, (prod >> u64::BITS) as u64)
}
