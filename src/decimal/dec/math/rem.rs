use crate::decimal::{dec::scale::rescale, Decimal, Signal};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn rem<const N: usize>(lhs: D<N>, rhs: D<N>) -> D<N> {
    if lhs.is_nan() {
        return lhs.compound_and_raise(&rhs, Signal::OP_INVALID);
    }

    if rhs.is_nan() {
        return rhs.compound_and_raise(&lhs, Signal::OP_INVALID);
    }

    let scale = if lhs.scale >= rhs.scale {
        lhs.scale
    } else {
        rhs.scale
    };

    let num = rescale(lhs, scale);
    let den = rescale(rhs, scale);

    if num.scale != den.scale {
        // TODO
        return lhs.compound_and_raise(&rhs, Signal::OP_OVERFLOW);
    }

    D::new(
        num.digits.rem(den.digits),
        scale,
        num.cb.combine(den.cb.abs()),
    )
}
