use crate::{
    decimal::{
        dec::{construct::construct, math::utils::overflow, scale::extend_scale_to},
        round::round,
        Decimal, Signal,
    },
    int::{math::div_rem_wide, UInt},
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn mul<const N: usize>(lhs: D<N>, rhs: D<N>) -> D<N> {
    if lhs.is_nan() {
        return lhs.compound_and_raise(&rhs, Signal::OP_INVALID);
    }

    if rhs.is_nan() {
        return rhs.compound_and_raise(&lhs, Signal::OP_INVALID);
    }

    let mut cb = lhs.cb.combine_mul(rhs.cb);

    if lhs.is_infinite() || rhs.is_infinite() {
        return if lhs.is_zero() || rhs.is_zero() {
            lhs.with_cb(cb).signaling_nan()
        } else {
            D::INFINITY.with_cb(cb)
        };
    }

    if lhs.is_zero() {
        return extend_scale_to(lhs.with_cb(cb), rhs.scale.saturating_add(lhs.scale));
    }

    if rhs.is_zero() {
        return extend_scale_to(rhs.with_cb(cb), lhs.scale.saturating_add(rhs.scale));
    }

    let (mut exp, mut overflow_exp) = (lhs.scale as i32 + rhs.scale as i32).overflowing_neg();

    if overflow_exp {
        return overflow(cb);
    }

    let (mut low, mut high) = lhs.digits.widening_mul(rhs.digits);

    if high.is_zero() {
        return construct(low, exp, cb);
    }

    let mut out;
    let mut rem;

    while !high.is_zero() {
        (exp, overflow_exp) = exp.overflowing_add(1);

        if overflow_exp {
            return overflow(cb);
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
            cb = cb.raise_signal(Signal::OP_INEXACT);
            low = round(low, rem, cb.sign(), cb.context());
        }
    }

    construct(low, exp, cb.raise_signal(Signal::OP_ROUNDED))
}
