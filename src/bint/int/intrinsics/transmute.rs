use crate::bint::{intrinsics::_transmute, Int};

#[allow(unsafe_code)]
#[inline(always)]
pub const unsafe fn transmute<const N: usize, const M: usize>(v: Int<N>) -> Int<M> {
    let bits = v.to_bits();

    if N <= M {
        Int::from_digits(_transmute::<_, _, N>(bits.digits()))
    } else {
        debug_assert!(v.last_digit_index() < M);
        debug_assert!(v.bits() <= Int::<M>::BITS);
        Int::from_digits(_transmute::<_, _, M>(bits.digits()))
    }
}
