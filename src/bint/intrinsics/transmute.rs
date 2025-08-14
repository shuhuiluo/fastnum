use crate::bint::intrinsics::Digits;

// TODO: low-level performance optimization
#[allow(unsafe_code)]
#[inline(always)]
pub const unsafe fn _transmute<const N: usize, const M: usize, const V: usize>(
    digits: &Digits<N>,
) -> Digits<M> {
    let mut out = [0; M];
    let mut i = 0;

    while i < V {
        out[i] = digits[i];
        i += 1;
    }

    out
}
