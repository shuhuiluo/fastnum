use crate::decimal::{
    dec::math::{add::add, div::div, exp::exp, mul::mul, sub::sub},
    Decimal,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn tanh<const N: usize>(x: D<N>) -> D<N> {
    let e2x = exp(mul(x, D::TWO));
    div(sub(e2x, D::ONE), add(e2x, D::ONE))
}
