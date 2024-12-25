use crate::{
    decimal::{
        dec::{construct::construct, ControlBlock},
        round::scale_round,
        Context, Decimal, Signal,
    },
    int::{math::div_rem, UInt},
};
use crate::decimal::dec::intrinsics::Intrinsics;

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn extend_scale_to<const N: usize>(d: D<N>, new_scale: i16) -> D<N> {
    if new_scale > d.scale {
        rescale(d, new_scale)
    } else {
        d
    }
}

#[inline]
pub(crate) const fn rescale<const N: usize>(mut d: D<N>, new_scale: i16) -> D<N> {
    if d.cb.is_special() {
        return d.signaling_nan();
    }

    if d.digits.is_zero() {
        d.scale = new_scale;
        return d;
    }

    if new_scale == d.scale {
        d
    } else if new_scale > d.scale {
        // increase the number of zeros if it possible
        while new_scale > d.scale {
            if d.digits.gt(&Intrinsics::<N>::COEFF_MEDIUM) {
                return d.raise_signal(Signal::OP_CLAMPED);
            } else {
                d.digits = d.digits.strict_mul(UInt::<N>::TEN);
                d.scale += 1;
            }
        }
        d
    } else {
        // round
        d.cb = d.cb.raise_signal(Signal::OP_ROUNDED);
        let mut is_inexact;
        while new_scale < d.scale {
            (d.digits, is_inexact) = scale_round(d.digits, d.cb.context());
            d.scale -= 1;

            if is_inexact {
                d.cb = d.cb.raise_signal(Signal::OP_INEXACT);
            }

            if d.digits.is_zero() {
                d.scale = new_scale;
                return d;
            }
        }
        d
    }
}

#[inline]
pub(crate) const fn quantum<const N: usize>(exp: i32, ctx: Context) -> D<N> {
    let cb = ControlBlock::default().set_context(ctx);
    construct(UInt::ONE, exp, cb)
}

#[inline]
pub(crate) const fn reduce<const N: usize>(mut d: D<N>) -> D<N> {
    if d.cb.is_special() {
        return d.raise_signal(Signal::OP_INVALID);
    }

    if d.digits.is_zero() {
        d.scale = 0;
    } else {
        let mut digits;
        let mut remainder;
        while !d.digits.is_zero() {
            (digits, remainder) = div_rem(d.digits, UInt::TEN);
            if remainder.is_zero() {
                if d.scale > i16::MIN {
                    d.digits = digits;
                    d.scale -= 1;
                } else {
                    return d.raise_signal(Signal::OP_SUBNORMAL);
                }
            } else {
                break;
            }
        }
    }

    d
}

#[inline]
pub(crate) const fn quantize<const N: usize>(d: D<N>, other: D<N>) -> D<N> {
    if d.is_infinite() && other.is_infinite() {
        d
    } else if d.cb.is_special() || other.cb.is_special() {
        d.signaling_nan()
    } else {
        let res = rescale(d, other.scale).quiet_signal(Signal::OP_UNDERFLOW);

        if res.scale != other.scale {
            d.signaling_nan()
        } else {
            res
        }
    }
}
