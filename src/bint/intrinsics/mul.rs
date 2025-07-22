#[inline]
pub const fn carrying_mul(multiplier: u64, multiplicand: u64, carry: u64) -> (u64, u64) {
    carrying_mul_add(multiplier, multiplicand, carry, 0)
}

// TODO
#[inline]
pub const fn carrying_mul_add(
    multiplier: u64,
    multiplicand: u64,
    addend: u64,
    carry: u64,
) -> (u64, u64) {
    let prod = carry as u128 + addend as u128 + (multiplier as u128) * (multiplicand as u128);
    (prod as u64, (prod >> u64::BITS) as u64)
}
