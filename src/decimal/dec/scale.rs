use crate::{
    decimal::{
        dec::math::utils::{overflow, underflow},
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

// TODO
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

#[inline]
pub(crate) const fn quantum<const N: usize>(exp: i32) -> D<N> {
    // Overflow exp > Emax
    if exp > D::<N>::E_MAX + (D::<N>::MAX_CLENGTH - 1) {
        return overflow(Flags::default());
    }

    if exp < D::<N>::E_MIN {
        return underflow(Flags::default());
    }

    if exp > D::<N>::E_MAX {
        let correct_exp = exp.abs_diff(D::<N>::E_MAX);
        return D::new(
            UInt::TEN.pow(correct_exp),
            i16::MIN,
            Flags::default()
                .raise_signal(Signal::OP_CLAMPED)
                .raise_signal(Signal::OP_ROUNDED),
        );
    }

    // TODO:
    // if exp < D::<N>::E_MIN + (D::<N>::MAX_CLENGTH - 1) {
    //     return D::new(
    //         UInt::ONE,
    //         -exp as i16,
    //         Flags::default().raise_signal(Signal::OP_SUBNORMAL),
    //     );
    // }

    D::new(UInt::ONE, -exp as i16, Flags::default())
}
