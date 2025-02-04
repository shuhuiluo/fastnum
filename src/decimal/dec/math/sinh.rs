use crate::decimal::{
    dec::math::{div::div, exp::exp, mul::mul, sub::sub},
    Decimal,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn sinh<const N: usize>(x: D<N>) -> D<N> {
    let e = exp(x);
    mul(D::HALF, sub(e, div(D::ONE, e)))
}
