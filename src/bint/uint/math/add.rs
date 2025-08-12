use crate::bint::{
    intrinsics::Digit,
    uint::math::utils::{as_u128, as_u64, uint128, uint64},
    UInt,
};

type U<const N: usize> = UInt<N>;

#[allow(unsafe_code)]
#[inline(always)]
pub const unsafe fn unchecked_add_digit<const N: usize>(value: U<N>, rhs: Digit) -> U<N> {
    match N {
        1 => uint64(as_u64(value).unchecked_add(rhs)),
        2 => uint128(as_u128(value).unchecked_add_u64(rhs)),
        // TODO
        _ => add_digit(value, rhs).0,
    }
}

#[inline(always)]
pub const fn add_digit<const N: usize>(value: U<N>, rhs: Digit) -> (U<N>, bool) {
    let mut out = *value.digits();
    let mut carry;

    (out[0], carry) = out[0].overflowing_add(rhs);

    let mut i = 1;
    while i < N && carry {
        (out[i], carry) = out[i].overflowing_add(1);
        i += 1;
    }

    (U::from_digits(out), carry)
}
