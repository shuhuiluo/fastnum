use crate::bint::UInt;

#[inline(always)]
pub const fn uint<const N: usize>(digit: u64) -> UInt<N> {
    UInt::from_digit(digit)
}
