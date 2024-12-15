use core::cmp::Ordering;

use crate::{
    decimal::{
        dec::{
            math::add::add_abs,
            scale::{extend_scale_to, with_scale},
        },
        Context, Decimal, Signal,
    },
    int::UInt,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn sub<const N: usize>(lhs: D<N>, rhs: D<N>, ctx: Context) -> D<N> {
    if lhs.is_nan() {
        return lhs.with_signals_from_and(&rhs, Signal::OP_INVALID);
    }

    if rhs.is_nan() {
        return rhs.with_signals_from_and(&lhs, Signal::OP_INVALID);
    }

    match (lhs.is_negative(), rhs.is_negative()) {
        (false, false) => sub_abs(lhs, rhs, ctx),
        (true, true) => sub_abs(rhs.neg(), lhs.neg(), ctx),
        (false, true) => add_abs(lhs, rhs.neg(), ctx),
        (true, false) => add_abs(lhs.neg(), rhs, ctx).neg(),
    }
}

#[inline]
pub(crate) const fn sub_abs<const N: usize>(mut lhs: D<N>, mut rhs: D<N>, ctx: Context) -> D<N> {
    debug_assert!(!lhs.is_negative() && !rhs.is_negative());

    if lhs.is_infinite() && rhs.is_infinite() {
        return D::NAN
            .with_signals_from(&lhs)
            .with_signals_from_and(&rhs, Signal::OP_INVALID);
    } else if lhs.is_infinite() {
        return lhs.with_signals_from(&rhs);
    } else if rhs.is_infinite() {
        return rhs.with_signals_from(&lhs);
    }
    
    if rhs.is_zero() {
        return extend_scale_to(lhs.with_signals_from(&rhs), rhs.scale, ctx);
    }

    if lhs.is_zero() {
        return extend_scale_to(rhs.with_signals_from(&lhs), lhs.scale, ctx).neg();
    }
    
    if lhs.scale == rhs.scale {
        sub_aligned(lhs, rhs)
    } else if lhs.scale < rhs.scale {
        lhs = with_scale(lhs, rhs.scale, ctx);

        if lhs.flags.has_signal(Signal::OP_CLAMPED) {
            rhs = with_scale(rhs, lhs.scale, ctx);
            sub_aligned(lhs, rhs)
        } else {
            sub_aligned(lhs, rhs)
        }
    } else {
        rhs = with_scale(rhs, lhs.scale, ctx);

        if rhs.flags.has_signal(Signal::OP_CLAMPED) {
            lhs = with_scale(lhs, rhs.scale, ctx);
            sub_aligned(lhs, rhs)
        } else {
            sub_aligned(lhs, rhs)
        }
    }
}

#[inline]
const fn sub_aligned<const N: usize>(lhs: D<N>, rhs: D<N>) -> D<N> {
    debug_assert!(lhs.scale == rhs.scale);

    match lhs.digits.cmp(&rhs.digits) {
        Ordering::Less => D::new(
            rhs.digits.strict_sub(lhs.digits),
            rhs.scale,
            rhs.flags.combine(lhs.flags),
        )
        .neg(),
        Ordering::Equal => D::new(UInt::ZERO, rhs.scale, lhs.flags.combine(rhs.flags)),
        Ordering::Greater => D::new(
            lhs.digits.strict_sub(rhs.digits),
            lhs.scale,
            lhs.flags.combine(rhs.flags),
        ),
    }
}
