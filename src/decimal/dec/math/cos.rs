use crate::decimal::{
    dec::math::{sin::sin, sub::sub},
    Decimal,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn cos<const N: usize>(x: D<N>) -> D<N> {
    sin(sub(D::FRAC_PI_2, x))
}
