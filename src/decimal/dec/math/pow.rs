use crate::{
    decimal::{
        dec::{construct::construct, math::utils::overflow_exp, ControlBlock},
        round::scale_round,
        Decimal, Flags, Signal,
    },
    int::UInt,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn powi<const N: usize>(d: D<N>, n: i32) -> D<N> {
    if d.is_nan() {
        return d.raise_signal(Signal::OP_INVALID);
    }

    let flags = if d.is_negative() && (n % 2 != 0) {
        Flags::default().neg()
    } else {
        Flags::default()
    };

    if d.is_infinite() {
        return if n > 0 {
            if flags.is_negative() ^ d.is_negative() {
                d.neg()
            } else {
                d
            }
        } else if n == 0 {
            D::ONE
        } else {
            D::ZERO.with_cb(ControlBlock::default().with_flags(flags))
        };
    }

    if n == 0 {
        return if d.is_zero() {
            d.signaling_nan()
        } else {
            D::ONE
        };
    }

    if d.is_zero() {
        return if n < 0 {
            D::INFINITY.with_cb(ControlBlock::default().with_flags(flags))
        } else {
            D::ZERO.with_cb(ControlBlock::default().with_flags(flags))
        };
    }

    if n < 0 {
        powi_integral(
            d.digits,
            (d.scale as i32).overflowing_neg().0,
            d.cb.set_flags(flags),
            n.overflowing_neg().0 as u32,
        )
        .recip()
    } else {
        powi_integral(
            d.digits,
            (d.scale as i32).overflowing_neg().0,
            d.cb.set_flags(flags),
            n as u32,
        )
    }
}

#[inline]
const fn powi_integral<const N: usize>(
    mut digits: UInt<N>,
    mut exp: i32,
    mut cb: ControlBlock,
    n: u32,
) -> D<N> {
    let (mut out, mut overflow) = digits.overflowing_pow(n);

    if overflow {
        let mut inexact;
        cb = cb.raise_signal(Signal::OP_ROUNDED);

        while overflow {
            (digits, inexact) = scale_round(digits, cb.sign(), cb.context());

            if inexact {
                cb = cb.raise_signal(Signal::OP_INEXACT);
            }

            (exp, overflow) = exp.overflowing_add(1);

            if overflow {
                return overflow_exp(exp, cb);
            }

            (out, overflow) = digits.overflowing_pow(n);
        }
    }

    if n > i32::MAX as u32 {
        return overflow_exp(-1, cb);
    }

    (exp, overflow) = exp.overflowing_mul(n as i32);

    if overflow {
        return overflow_exp(exp, cb);
    }

    construct(out, exp, cb)
}
