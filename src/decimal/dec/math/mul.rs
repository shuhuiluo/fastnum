use crate::{
    decimal::{
        dec::{math::utils::overflow_scale, scale::extend_scale_to},
        round::round,
        Context, Decimal, Signal,
    },
    int::{math::div_rem_wide, UInt},
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn mul<const N: usize>(lhs: D<N>, rhs: D<N>, ctx: Context) -> D<N> {
    if lhs.is_nan() {
        return lhs.with_signals_from_and(&rhs, Signal::OP_INVALID);
    }

    if rhs.is_nan() {
        return rhs.with_signals_from_and(&lhs, Signal::OP_INVALID);
    }

    let mut flags = lhs.flags.mul(rhs.flags);

    if lhs.is_infinite() || rhs.is_infinite() {
        return D::INFINITY.with_flags(flags);
    }

    if lhs.is_zero() {
        return extend_scale_to(
            lhs.with_flags(flags),
            rhs.scale.saturating_add(lhs.scale),
            ctx,
        );
    }

    if rhs.is_zero() {
        return extend_scale_to(
            rhs.with_flags(flags),
            lhs.scale.saturating_add(rhs.scale),
            ctx,
        );
    }

    let (mut scale, mut overflow) = lhs.scale.overflowing_add(rhs.scale);

    if overflow {
        return overflow_scale(scale, flags);
    }

    let (mut low, mut high) = lhs.digits.widening_mul(rhs.digits);

    let mut out;
    let mut rem;

    if high.is_zero() {
        return D::new(low, scale, flags);
    }

    while !high.is_zero() {
        (scale, overflow) = scale.overflowing_sub(1);

        if overflow {
            return overflow_scale(scale, flags);
        }

        out = [0; N];
        rem = 0;

        let mut i = N;
        while i > 0 {
            i -= 1;
            let (q, r) = div_rem_wide(high.digits()[i], rem, 10);
            rem = r;
            out[i] = q;
        }

        high = UInt::from_digits(out);

        i = N;
        out = [0; N];

        while i > 0 {
            i -= 1;
            let (q, r) = div_rem_wide(low.digits()[i], rem, 10);
            rem = r;
            out[i] = q;
        }

        low = UInt::from_digits(out);

        if rem != 0 {
            flags = flags.raise_signal(Signal::OP_INEXACT);
            low = round(low, UInt::from_digit(rem), ctx);
        }
    }

    D::new(low, scale, flags.raise_signal(Signal::OP_ROUNDED))
}
