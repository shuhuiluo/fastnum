use crate::decimal::{
    dec::math::{cos::cos, div::div, sin::sin},
    Decimal,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn tan<const N: usize>(x: D<N>) -> D<N> {
    div(sin(x), cos(x))
}
