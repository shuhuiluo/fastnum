use crate::decimal::{
    dec::{
        convert::to_i32,
        math::{exp::exp, ln::ln, mul::mul, powi::powi},
        ControlBlock,
    },
    Decimal, Flags, Signal,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn pow<const N: usize>(d: D<N>, n: D<N>) -> D<N> {
    if n.is_integral() {
        if let Some(n) = to_i32(n) {
            return powi(d, n);
        }
    }

    if d.is_nan() {
        return d.raise_signal(Signal::OP_INVALID);
    }

    let flags = if d.is_negative() && n.is_even() {
        Flags::default().neg()
    } else {
        Flags::default()
    };

    if d.is_infinite() {
        return if n.is_zero() {
            D::ONE
        } else if n.is_negative() {
            D::ZERO.with_cb(ControlBlock::default().with_flags(flags))
        } else if flags.is_negative() ^ d.is_negative() {
            d.neg()
        } else {
            d
        };
    }

    if n.is_zero() {
        return if d.is_zero() {
            d.signaling_nan()
        } else {
            D::ONE
        };
    }

    if d.is_zero() {
        return if n.is_negative() {
            D::INFINITY.with_cb(ControlBlock::default().with_flags(flags))
        } else {
            D::ZERO.with_cb(ControlBlock::default().with_flags(flags))
        };
    }

    powf(d, n)
}

#[inline]
const fn powf<const N: usize>(d: D<N>, n: D<N>) -> D<N> {
    debug_assert!(!d.is_negative());
    debug_assert!(!d.is_zero());
    exp(mul(ln(d), n))
}
