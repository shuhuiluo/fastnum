use crate::bint::UInt;

#[inline(always)]
pub const fn uint<const N: usize>(digit: u32) -> UInt<N> {
    UInt::from_digit(digit as u64)
}
