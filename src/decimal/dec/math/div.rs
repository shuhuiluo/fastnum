use crate::{
    decimal::{
        dec::math::utils::overflow_scale,
        round::{scale_round, RoundConsts},
        Context, Decimal, Signal,
    },
    int::{math::div_rem, UInt},
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn div<const N: usize>(dividend: D<N>, divisor: D<N>, ctx: Context) -> D<N> {
    if dividend.is_nan() {
        return dividend.with_signals_from_and(&divisor, Signal::OP_INVALID);
    }

    if divisor.is_nan() {
        return divisor.with_signals_from_and(&dividend, Signal::OP_INVALID);
    }

    if dividend.is_infinite() && divisor.is_infinite() {
        return D::NAN
            .with_signals_from(&dividend)
            .with_signals_from_and(&divisor, Signal::OP_INVALID);
    }

    let flags = dividend.flags.mul(divisor.flags);

    if divisor.is_zero() {
        D::INFINITY.with_flags(flags.raise_signal(Signal::div_by_zero()))
    } else if dividend.is_zero() || divisor.is_one() {
        dividend.with_signals_from(&divisor)
    } else if dividend.is_infinite() {
        D::INFINITY.with_flags(flags)
    } else if divisor.is_infinite() {
        D::ZERO.with_flags(flags)
    } else {
        let (mut scale, mut ofw) = dividend.scale.overflowing_sub(divisor.scale);
        // TODO: may be we can adjust scale
        if ofw {
            return overflow_scale(scale, flags);
        }

        let (mut digits, mut remainder) = div_rem(dividend.digits, divisor.digits);

        if !remainder.is_zero() {
            let mut quotient;

            while !remainder.is_zero() {
                (remainder, ofw) = remainder.overflowing_mul(UInt::TEN);

                if ofw {
                    return D::new(
                        digits,
                        scale,
                        flags
                            .raise_signal(Signal::OP_INEXACT)
                            .raise_signal(Signal::OP_ROUNDED),
                    );
                }

                (quotient, remainder) = div_rem(remainder, divisor.digits);

                if digits.gt(&RoundConsts::MAX) {
                    // TODO: performance optimizations
                    let (digit, _) = scale_round(quotient, ctx);

                    if digit.is_one() {
                        digits = digits.saturating_add(digit);
                    }

                    return D::new(
                        digits,
                        scale,
                        flags
                            .raise_signal(Signal::OP_INEXACT)
                            .raise_signal(Signal::OP_ROUNDED),
                    );
                }

                digits = digits.strict_mul(UInt::TEN);

                (scale, ofw) = scale.overflowing_add(1);
                if ofw {
                    return overflow_scale(scale, flags);
                }

                if digits.gt(&UInt::MAX.strict_sub(quotient)) {
                    return D::new(
                        UInt::MAX,
                        scale,
                        flags
                            .raise_signal(Signal::OP_INEXACT)
                            .raise_signal(Signal::OP_ROUNDED),
                    );
                }

                digits = digits.strict_add(quotient);
            }
        }

        D::new(digits, scale, flags)
    }
}
