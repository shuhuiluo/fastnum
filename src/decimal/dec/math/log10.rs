use crate::decimal::{
    dec::math::{div::div, ln::ln},
    Decimal,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn log10<const N: usize>(d: D<N>) -> D<N> {
    div(ln(d), D::LN_10)
}
