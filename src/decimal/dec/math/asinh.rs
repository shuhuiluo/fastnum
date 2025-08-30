use crate::decimal::{
    dec::math::{add::add, log::ln, mul::mul, sqrt::sqrt},
    Decimal,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn asinh<const N: usize>(x: D<N>) -> D<N> {
    ln(add(x, sqrt(add(D::ONE, mul(x, x)))))
}
