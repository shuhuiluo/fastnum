use crate::decimal::{
    dec::{
        intrinsics::Intrinsics,
        math::sub::sub_abs,
        scale::{extend_scale_to, rescale},
    },
    Decimal, Signal,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn add<const N: usize>(lhs: D<N>, rhs: D<N>) -> D<N> {
    if lhs.is_nan() {
        return lhs.compound_and_raise(&rhs, Signal::OP_INVALID);
    }

    if rhs.is_nan() {
        return rhs.compound_and_raise(&lhs, Signal::OP_INVALID);
    }

    match (lhs.is_negative(), rhs.is_negative()) {
        (false, false) => add_abs(lhs, rhs),
        (true, true) => add_abs(rhs.neg(), lhs.neg()).neg(),
        (false, true) => sub_abs(lhs, rhs.neg()),
        (true, false) => sub_abs(rhs, lhs.neg()),
    }
}

#[inline]
pub(crate) const fn add_abs<const N: usize>(lhs: D<N>, rhs: D<N>) -> D<N> {
    debug_assert!(!lhs.is_negative() && !rhs.is_negative());

    if lhs.is_infinite() {
        return lhs.compound(&rhs);
    }

    if rhs.is_infinite() {
        return rhs.compound(&lhs);
    }

    if rhs.is_zero() {
        return extend_scale_to(lhs, rhs.scale).compound(&rhs);
    }

    if lhs.is_zero() {
        return extend_scale_to(rhs, lhs.scale).compound(&lhs);
    }

    if lhs.scale == rhs.scale {
        add_aligned(lhs, rhs)
    } else if lhs.scale < rhs.scale {
        add_rescale(lhs, rhs)
    } else {
        add_rescale(rhs, lhs)
    }
}

#[inline]
const fn add_rescale<const N: usize>(mut lhs: D<N>, mut rhs: D<N>) -> D<N> {
    lhs = rescale(lhs, rhs.scale);

    if lhs.is_op_clamped() {
        rhs = rescale(rhs, lhs.scale);
        add_aligned(lhs, rhs)
    } else {
        add_aligned(lhs, rhs)
    }
}

#[inline]
const fn add_aligned<const N: usize>(mut lhs: D<N>, mut rhs: D<N>) -> D<N> {
    debug_assert!(lhs.scale == rhs.scale);

    let mut overflow;

    (lhs.digits, overflow) = lhs.digits.overflowing_add(rhs.digits);

    if !overflow {
        lhs.compound(&rhs)
    } else {
        // TODO: forward to round
        rhs.digits = Intrinsics::<N>::COEFF_MEDIUM_PLUS_ONE;
        (rhs.scale, overflow) = rhs.scale.overflowing_sub(1);

        if overflow {
            return lhs.compound_and_raise(&rhs, Signal::OP_OVERFLOW);
        }

        let scale;
        (scale, overflow) = lhs.scale.overflowing_sub(1);

        if overflow {
            return lhs.compound_and_raise(&rhs, Signal::OP_OVERFLOW);
        }

        lhs = rescale(lhs, scale);
        add_aligned(lhs, rhs)
    }
}
