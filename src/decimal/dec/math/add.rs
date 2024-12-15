use crate::{
    decimal::{
        dec::{
            math::sub::sub_abs,
            scale::{extend_scale_to, with_scale},
        },
        round::RoundConsts,
        Context, Decimal, Signal,
    },
    int::UInt,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn add<const N: usize>(lhs: D<N>, rhs: D<N>, ctx: Context) -> D<N> {
    if lhs.is_nan() {
        return lhs.with_signals_from_and(&rhs, Signal::OP_INVALID);
    }

    if rhs.is_nan() {
        return rhs.with_signals_from_and(&lhs, Signal::OP_INVALID);
    }

    match (lhs.is_negative(), rhs.is_negative()) {
        (false, false) => add_abs(lhs, rhs, ctx),
        (true, true) => add_abs(rhs.neg(), lhs.neg(), ctx).neg(),
        (false, true) => sub_abs(lhs, rhs.neg(), ctx),
        (true, false) => sub_abs(rhs, lhs.neg(), ctx),
    }
}

#[inline]
pub(crate) const fn add_abs<const N: usize>(lhs: D<N>, rhs: D<N>, ctx: Context) -> D<N> {
    debug_assert!(!lhs.is_negative() && !rhs.is_negative());

    if lhs.is_infinite() {
        return lhs.with_signals_from(&rhs);
    }

    if rhs.is_infinite() {
        return rhs.with_signals_from(&lhs);
    }
    
    if rhs.is_zero() {
        return extend_scale_to(lhs, rhs.scale, ctx).with_signals_from(&rhs);
    }

    if lhs.is_zero() {
        return extend_scale_to(rhs, lhs.scale, ctx).with_signals_from(&lhs);
    }
    
    if lhs.scale == rhs.scale {
        add_aligned(lhs, rhs, ctx)
    } else if lhs.scale < rhs.scale {
        add_rescale(lhs, rhs, ctx)
    } else {
        add_rescale(rhs, lhs, ctx)
    }
}

#[inline]
const fn add_rescale<const N: usize>(mut lhs: D<N>, mut rhs: D<N>, ctx: Context) -> D<N> {
    lhs = with_scale(lhs, rhs.scale, ctx);

    if lhs.flags.has_signal(Signal::OP_CLAMPED) {
        rhs = with_scale(rhs, lhs.scale, ctx);
        add_aligned(lhs, rhs, ctx)
    } else {
        add_aligned(lhs, rhs, ctx)
    }
}

#[inline]
const fn add_aligned<const N: usize>(mut lhs: D<N>, mut rhs: D<N>, ctx: Context) -> D<N> {
    debug_assert!(lhs.scale == rhs.scale);

    let mut overflow;

    (lhs.digits, overflow) = lhs.digits.overflowing_add(rhs.digits);

    if !overflow {
        lhs.with_signals_from(&rhs)
    } else {
        rhs.digits = RoundConsts::MAX;
        rhs.digits = rhs.digits.strict_add(UInt::ONE);
        (rhs.scale, overflow) = rhs.scale.overflowing_sub(1);

        if overflow {
            return lhs.with_signals_from_and(&rhs, Signal::OP_OVERFLOW);
        }

        let scale;
        (scale, overflow) = lhs.scale.overflowing_sub(1);

        if overflow {
            return lhs.with_signals_from_and(&rhs, Signal::OP_OVERFLOW);
        }

        lhs = with_scale(lhs, scale, ctx);
        add_aligned(lhs, rhs, ctx)
    }
}
