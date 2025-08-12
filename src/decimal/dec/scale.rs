use crate::{
    bint::{intrinsics::ExpType, UInt},
    decimal::{
        dec::{construct::construct, math::add::add, ExtraPrecision},
        signals::Signals,
        Context, Decimal, Sign,
    },
};

type D<const N: usize> = Decimal<N>;

const EXTRA_PRECISION_DIGITS: ExpType = ExtraPrecision::EXTRA_PRECISION_DIGITS;

#[inline(always)]
pub(crate) const fn extend_scale_to<const N: usize>(mut d: D<N>, new_scale: i16) -> D<N> {
    if new_scale > d.cb.get_scale() {
        rescale(&mut d, new_scale)
    }
    d
}

#[inline(always)]
pub(crate) const fn rescale<const N: usize>(d: &mut D<N>, new_scale: i16) {
    if d.cb.is_special() {
        d.cb.signaling_nan();
        return;
    }

    let scale = d.cb.get_scale();

    if new_scale > scale {
        // Re-scale up. Multiply Coefficient by 10, increase scale: 10 -> 100e-1
        rescale_up(d, new_scale);
    } else if new_scale < scale {
        // Re-scale down. Divide Coefficient by 10 with rounding, decrease scale: 1234
        // -> 123e+1
        rescale_down(d, new_scale);
    }
}

#[inline]
pub(crate) const fn quantum<const N: usize>(exp: i32, ctx: Context) -> D<N> {
    construct(
        UInt::ONE,
        exp,
        Sign::Plus,
        Signals::empty(),
        ctx,
        ExtraPrecision::new(),
    )
}

#[inline(never)]
pub(crate) const fn reduce<const N: usize>(mut d: D<N>) -> D<N> {
    if d.cb.is_special() {
        return d.signaling_nan();
    }

    if d.has_extra_precision() {
        return d;
    }

    if d.digits.is_zero() {
        d.cb.set_scale(0);
    } else {
        let mut digits;
        let mut remainder;
        while !d.digits.is_zero() {
            (digits, remainder) = d.digits.div_rem_digit(10);
            if remainder == 0 {
                if d.cb.get_scale() > i16::MIN {
                    d.digits = digits;
                    d.cb.dec_scale(1);
                } else {
                    return d.raise_signals(Signals::OP_SUBNORMAL);
                }
            } else {
                break;
            }
        }
    }

    d
}

#[inline]
pub(crate) const fn quantize<const N: usize>(mut d: D<N>, other: D<N>) -> D<N> {
    if d.is_infinite() && other.is_infinite() {
        d
    } else if d.cb.is_special() || other.cb.is_special() {
        d.signaling_nan()
    } else {
        let scale = other.cb.get_scale();

        rescale(&mut d, scale);
        d.cb.quiet_signals(Signals::OP_UNDERFLOW);

        if d.cb.get_scale() != scale {
            d.signaling_nan()
        } else {
            d
        }
    }
}

#[inline(never)]
const fn rescale_up<const N: usize>(d: &mut D<N>, new_scale: i16) {
    debug_assert!(new_scale > d.cb.get_scale());

    let power = (new_scale as i32 - d.cb.get_scale() as i32) as u32;

    if d.digits.is_zero() {
        d.cb.set_scale(new_scale);

        let mut extra_precision = d.cb.get_extra_precision();

        if let Some(correction) = extra_precision.scale_up(power) {
            correct(d, correction);
        }

        return d.cb.set_extra_precision(extra_precision);
    }

    let remaining_decimal_digits = d.digits.remaining_decimal_digits();

    if remaining_decimal_digits == 0 {
        d.cb.raise_signals(Signals::OP_CLAMPED);
    } else if remaining_decimal_digits < power {
        d.cb.raise_signals(Signals::OP_CLAMPED);
        rescale_up_unchecked(d, remaining_decimal_digits);
    } else {
        rescale_up_unchecked(d, power);
    }
}

#[inline(never)]
const fn rescale_down<const N: usize>(d: &mut D<N>, new_scale: i16) {
    debug_assert!(new_scale < d.cb.get_scale());
    d.cb.raise_signals(Signals::OP_ROUNDED);

    let extra_precision = d.cb.get_extra_precision();

    let power = (d.cb.get_scale() - new_scale) as u32;
    let decimal_digits = d.digits.decimal_digits();

    if decimal_digits <= power {
        // Fast path: all digits are shifted, and current extra precision burned
        if !d.digits.is_zero() || !extra_precision.is_zero() {
            d.cb.raise_signals(Signals::OP_INEXACT);
        }

        let extra_precision_shift = power - decimal_digits;

        if extra_precision_shift >= EXTRA_PRECISION_DIGITS {
            // Very fast path: all digits burned
            d.cb.reset_extra_precision();
        } else {
            // Medium fast path: some digits shifted to extra precision
            let new_extra_precision =
                make_extra_precision(d.digits, decimal_digits, extra_precision_shift);
            d.cb.set_extra_precision(new_extra_precision);
        }

        d.digits = UInt::ZERO;
    } else {
        let divisor = UInt::power_of_ten(power);
        let extra;
        (d.digits, extra) = d.digits.div_rem(divisor);

        if !extra.is_zero() || !extra_precision.is_zero() {
            d.cb.raise_signals(Signals::OP_INEXACT);

            let mut new_extra_precision = make_extra_precision(extra, power, 0);
            new_extra_precision.push_back(extra_precision);
            d.cb.set_extra_precision(new_extra_precision);
        }
    }

    d.cb.dec_scale(power);
}

#[inline(always)]
const fn rescale_up_unchecked<const N: usize>(d: &mut D<N>, gap: u32) {
    debug_assert!(gap <= d.digits.remaining_decimal_digits());
    // SAFETY: `gap` is less than or equal to `d.digits.remaining_decimal_digits()`
    #[allow(unsafe_code)]
    {
        d.digits = unsafe { d.digits.unchecked_mul(UInt::<N>::power_of_ten(gap)) };
    }

    d.cb.inc_scale(gap);

    let mut extra_precision = d.cb.get_extra_precision();

    if let Some(correction) = extra_precision.scale_up(gap) {
        correct(d, correction);
    }

    d.cb.set_extra_precision(extra_precision);
}

// FIXME: remove
#[inline]
const fn correct<const N: usize>(d: &mut D<N>, mut correction: D<N>) {
    correction.set_sign(d.sign());
    correction.cb.inc_scale(d.cb.get_scale() as ExpType);
    *d = add(*d, correction);
}

#[inline(always)]
const fn make_extra_precision<const N: usize>(
    digits: UInt<N>,
    decimal_digits: ExpType,
    shift: ExpType,
) -> ExtraPrecision {
    // Need to take `EXTRA_PRECISION_DIGITS - shift` leading decimal digits
    let power = decimal_digits.saturating_sub(EXTRA_PRECISION_DIGITS - shift);
    let digits = if power > 0 {
        let scale = UInt::power_of_ten(power);
        let extra = digits.div(scale);
        debug_assert!(extra.last_digit_index() == 0);
        extra.digits()[0]
    } else {
        debug_assert!(digits.last_digit_index() == 0);
        digits.digits()[0]
    };

    ExtraPrecision::from_digits(digits, decimal_digits - power + shift)
}

// #[cfg(test)]
// mod tests {
//     use rstest::*;
//     use crate::D64;
//     use super::*;
//
//     #[rstest(::trace)]
//     fn test() {
//         let mut d = D64::from_u64(18446744073709551615);
//         d.cb.set_extra_digits(1234567);
//
//         println!("D[   ]: {d} [{d:?}]");
//
//         for i in 1..30 {
//             let mut dd = d;
//             rescale_down(&mut dd, -(i as i16));
//
//             println!("D[R{i:02}]: {dd} [{dd:?}]");
//         }
//
//         let mut d = D64::from_u64(18446744073709550000);
//         d.cb.set_extra_digits(1234567);
//
//         println!("\n\nD[   ]: {d} [{d:?}]");
//
//         for i in 1..30 {
//             let mut dd = d;
//             rescale_down(&mut dd, -(i as i16));
//
//             println!("D[R{i:02}]: {dd} [{dd:?}]");
//         }
//
//         let mut d = D64::from_u64(18446744073709551615);
//         d.cb.set_extra_digits(123);
//
//         println!("\n\nD[   ]: {d} [{d:?}]");
//
//         for i in 1..30 {
//             let mut dd = d;
//             rescale_down(&mut dd, -(i as i16));
//
//             println!("D[R{i:02}]: {dd} [{dd:?}]");
//         }
//     }
// }
