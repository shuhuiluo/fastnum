use crate::{
    decimal::{
        round::{scale_round, RoundConsts},
        Context, Decimal, Flags, Signal,
    },
    int::UInt,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn extend_scale_to<const N: usize>(d: D<N>, new_scale: i16, ctx: Context) -> D<N> {
    if new_scale > d.scale {
        with_scale(d, new_scale, ctx)
    } else {
        d
    }
}

#[inline]
pub(crate) const fn with_scale<const N: usize>(mut d: D<N>, new_scale: i16, ctx: Context) -> D<N> {
    if d.flags.is_special() {
        return d.raise_signal(Signal::OP_INVALID);
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
            if d.digits.gt(&RoundConsts::<N>::MAX) {
                return d.raise_signal(Signal::OP_CLAMPED);
            } else {
                d.digits = d.digits.strict_mul(UInt::<N>::TEN);
                d.scale += 1;
            }
        }
        d
    } else {
        // round
        let mut flags = Flags::default();
        let mut is_rounded;
        while new_scale < d.scale {
            (d.digits, is_rounded) = scale_round(d.digits, ctx);
            d.scale -= 1;

            if is_rounded {
                flags = flags
                    .raise_signal(Signal::OP_ROUNDED)
                    .raise_signal(Signal::OP_INEXACT);
            }

            if d.digits.is_zero() {
                d.scale = new_scale;
                return d.with_flags(flags);
            }
        }
        d.with_flags(flags)
    }
}
