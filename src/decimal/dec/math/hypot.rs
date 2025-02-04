use crate::decimal::{
    dec::math::{add::add, mul::mul, sqrt::sqrt},
    Decimal,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn hypot<const N: usize>(x: D<N>, y: D<N>) -> D<N> {
    sqrt(add(mul(x, x), mul(y, y)))
}
