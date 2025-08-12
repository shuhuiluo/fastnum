use crate::bint::{
    intrinsics::{ExpType, _U128},
    UInt,
};

type U<const N: usize> = UInt<N>;

#[inline(always)]
pub const fn as_u64<const N: usize>(n: U<N>) -> u64 {
    debug_assert!(N == 1);
    n.digits()[0]
}

#[inline(always)]
pub const fn as_u64_ref<const N: usize>(n: &U<N>) -> u64 {
    debug_assert!(N == 1);
    n.digits()[0]
}

#[inline(always)]
pub const fn as_u128_ref<const N: usize>(n: &U<N>) -> _U128 {
    debug_assert!(N == 2);
    _U128 {
        low: n.digits()[0],
        high: n.digits()[1],
    }
}

#[inline(always)]
pub const fn as_u128<const N: usize>(n: U<N>) -> _U128 {
    debug_assert!(N == 2);

    // #[allow(unsafe_code)]
    // unsafe {
    //     core::mem::transmute(*(n.digits().as_ptr() as *const _U128))
    // }

    _U128 {
        low: n.digits()[0],
        high: n.digits()[1],
    }
}

#[inline(always)]
pub const fn uint64<const N: usize>(v: u64) -> U<N> {
    debug_assert!(N == 1);
    U::from_digit(v)
}

#[inline(always)]
pub const fn uint128<const N: usize>(v: _U128) -> U<N> {
    debug_assert!(N == 2);
    let mut out = [0; N];
    out[0] = v.low;
    out[1] = v.high;
    U::from_digits(out)
}

#[inline(always)]
pub const fn uint_pair64<const N: usize>((v1, v2): (u64, u64)) -> (U<N>, U<N>) {
    debug_assert!(N == 1);
    (uint64(v1), uint64(v2))
}

#[inline]
pub const fn wide_shl<const N: usize>(mut low: U<N>, mut high: U<N>, shl: ExpType) -> (U<N>, U<N>) {
    let shr = U::<N>::BITS - shl;
    high = high.strict_shl(shl).strict_add(low.strict_shr(shr));
    low = low.strict_shl(shl);

    (low, high)
}
