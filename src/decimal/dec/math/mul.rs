use crate::{
    bint::{intrinsics::*, UInt},
    decimal::{
        dec::{
            construct::construct,
            math::{add::add, utils::correct},
            ExtraPrecision,
        },
        signals::Signals,
        Context, Decimal,
    },
};

type D<const N: usize> = Decimal<N>;

#[inline(never)]
pub(crate) const fn mul<const N: usize>(mut lhs: D<N>, mut rhs: D<N>) -> D<N> {
    if lhs.is_nan() {
        return lhs.compound(&rhs).op_invalid();
    }

    if rhs.is_nan() {
        return rhs.compound(&lhs).op_invalid();
    }

    let sign = lhs.sign().mul(rhs.sign());
    let mut signals = Signals::combine(lhs.cb.get_signals(), rhs.cb.get_signals());
    let ctx = Context::merge(lhs.cb.get_context(), rhs.cb.get_context());

    if lhs.is_infinite() || rhs.is_infinite() {
        return if lhs.is_zero() || rhs.is_zero() {
            D::SIGNALING_NAN.set_ctx(ctx).compound(&lhs).compound(&rhs)
        } else {
            D::INFINITY.set_ctx(ctx).set_sign(sign)
        };
    }

    let mut exp = lhs.cb.get_exponent() + rhs.cb.get_exponent();

    let mut extra_precision = ExtraPrecision::new();

    if lhs.is_zero() {
        return construct(UInt::ZERO, exp, sign, signals, ctx, extra_precision);
    }

    if rhs.is_zero() {
        return construct(UInt::ZERO, exp, sign, signals, ctx, extra_precision);
    }

    let correction = if lhs.cb.has_extra_precision() || rhs.cb.has_extra_precision() {
        Some(mul_correction(&mut lhs, &mut rhs))
    } else {
        None
    };

    // TODO: Performance optimization
    // If overflows then we can try to truncate coefficient and make extra precision
    // in single (several) step instead of iteration looping divide by 10.
    // See `div` implementation.
    let (mut low, mut high) = lhs.digits.widening_mul(rhs.digits);

    if !high.is_zero() {
        signals.raise(Signals::OP_ROUNDED);

        let mut out;
        let mut rem;

        while !high.is_zero() {
            exp += 1;

            out = [0; N];
            rem = 0;

            let mut i = N;
            while i > 0 {
                i -= 1;
                let (q, r) = _div_rem_128_64(high.digits()[i], rem, 10);
                rem = r;
                out[i] = q;
            }

            high = UInt::from_digits(out);

            i = N;
            out = [0; N];

            while i > 0 {
                i -= 1;
                let (q, r) = _div_rem_128_64(low.digits()[i], rem, 10);
                rem = r;
                out[i] = q;
            }

            low = UInt::from_digits(out);

            if rem != 0 {
                signals.raise(Signals::OP_INEXACT);
            }

            extra_precision.push_digit(rem);
        }
    }

    let result = construct(low, exp, sign, signals, ctx, extra_precision);

    if let Some(correction) = correction {
        correct(result, correction)
    } else {
        result
    }
}

#[inline]
const fn mul_correction<const N: usize>(lhs: &mut D<N>, rhs: &mut D<N>) -> D<N> {
    let xi_lhs = lhs.cb.take_extra_precision_decimal();
    let xi_rhs = rhs.cb.take_extra_precision_decimal();

    if xi_lhs.is_zero() && xi_rhs.is_zero() {
        D::ZERO
    } else if xi_lhs.is_zero() {
        mul(*lhs, xi_rhs)
    } else if xi_rhs.is_zero() {
        mul(*rhs, xi_lhs)
    } else {
        add(
            mul(*lhs, xi_rhs),
            add(mul(*rhs, xi_lhs), mul(xi_rhs, xi_lhs)),
        )
    }
}
