use core::cmp::Ordering;

use crate::decimal::{
    dec::math::{add::add, asin::asin, div::div, mul::mul, sqrt::sqrt},
    Decimal, Signal,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn atan<const N: usize>(x: D<N>) -> D<N> {
    if x.is_nan() {
        return x.raise_signal(Signal::OP_INVALID);
    }

    if x.is_zero() {
        return D::ZERO.with_ctx(x.context());
    }

    if x.is_infinite() {
        return x.signaling_nan();
    }

    match x.cmp(&D::ONE.neg()) {
        Ordering::Less => {
            return x.signaling_nan();
        }
        Ordering::Equal => return D::FRAC_PI_4.neg(),
        Ordering::Greater => {}
    }

    match x.cmp(&D::ONE) {
        Ordering::Less => {}
        Ordering::Equal => {
            return D::FRAC_PI_4;
        }
        Ordering::Greater => {
            return x.signaling_nan();
        }
    }

    let x2 = mul(x, x);
    asin(div(x, sqrt(add(x2, D::<N>::ONE))))
}
