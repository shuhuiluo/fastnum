use crate::decimal::{
    dec::math::{add::add, div::div, exp::exp, mul::mul},
    Decimal,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn cosh<const N: usize>(x: D<N>) -> D<N> {
    let e = exp(x);
    mul(D::HALF, add(e, div(D::ONE, e)))
}
