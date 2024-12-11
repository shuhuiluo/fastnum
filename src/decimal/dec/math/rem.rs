use crate::decimal::{dec::scale::with_scale, Context, Decimal, Signal};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn rem<const N: usize>(lhs: D<N>, rhs: D<N>, ctx: Context) -> D<N> {
    if lhs.is_nan() {
        return lhs.with_signals_from_and(&rhs, Signal::OP_INVALID);
    }

    if rhs.is_nan() {
        return rhs.with_signals_from_and(&lhs, Signal::OP_INVALID);
    }

    let scale = if lhs.scale >= rhs.scale {
        lhs.scale
    } else {
        rhs.scale
    };

    let num = with_scale(lhs, scale, ctx);
    let den = with_scale(rhs, scale, ctx);

    if num.scale != den.scale {
        // TODO
        return lhs.with_signals_from_and(&rhs, Signal::OP_OVERFLOW);
    }

    D::new(
        num.digits.rem(den.digits),
        scale,
        num.flags.combine(den.flags.abs()),
    )
}
