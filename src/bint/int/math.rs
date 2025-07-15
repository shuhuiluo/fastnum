use crate::bint::Int;

#[inline]
pub const fn div_rem<const N: usize>(dividend: Int<N>, divisor: Int<N>) -> (Int<N>, Int<N>) {
    // TODO
    (dividend.div(divisor), dividend.rem(divisor))
}
