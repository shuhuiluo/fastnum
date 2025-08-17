use crate::{
    bint::{intrinsics::ExpType, UInt},
    decimal::{
        dec::{
            intrinsics::{Intrinsics, E_LIMIT, E_MIN},
            math::utils::{overflow, underflow},
            ControlBlock, ExtraPrecision,
        },
        signals::Signals,
        Context, Decimal, Sign,
    },
};

type D<const N: usize> = Decimal<N>;

#[inline(always)]
pub(crate) const fn construct<const N: usize>(
    mut digits: UInt<N>,
    mut exp: i32,
    sign: Sign,
    mut signals: Signals,
    ctx: Context,
    mut extra_precision: ExtraPrecision,
) -> D<N> {
    if digits.is_zero() {
        return construct_zero(exp, sign, signals, ctx, extra_precision);
    }

    // Overflow exp > Emax
    if exp > Intrinsics::<N>::E_MAX {
        return overflow(sign, signals, ctx);
    }

    // Underflow exp < Emin
    if exp < E_MIN {
        return underflow(sign, signals, ctx);
    }

    if exp <= E_LIMIT {
        if exp < Intrinsics::<N>::E_SUBNORMAL {
            let clength = digits.decimal_digits();

            if exp < E_MIN + (clength as i32 - 1) {
                signals.raise(Signals::OP_SUBNORMAL);
            }
        }

        let cb = ControlBlock::new(-exp as i16, sign, signals, ctx, extra_precision);
        return D::new(digits, cb);
    }

    signals.raise(Signals::OP_ROUNDED);
    signals.raise(Signals::OP_CLAMPED);

    let power = (exp - E_LIMIT) as ExpType;

    if power > digits.remaining_decimal_digits() {
        return overflow(sign, signals, ctx);
    }

    // SAFETY: `power` is less than `digits.remaining_decimal_digits()`
    #[allow(unsafe_code)]
    {
        let multiplier = UInt::unchecked_power_of_ten(power);
        digits = unsafe { digits.unchecked_mul(multiplier) };
    }

    if let Some(correction) = extra_precision.scale_up::<N>(power) {
        let ofw;
        (digits, ofw) = digits.overflowing_add(correction.digits);

        if ofw {
            return overflow(sign, signals, ctx);
        }
    }

    exp = E_LIMIT;

    let cb = ControlBlock::new(-exp as i16, sign, signals, ctx, extra_precision);
    D::new(digits, cb)
}

#[inline(always)]
const fn construct_zero<const N: usize>(
    exp: i32,
    sign: Sign,
    mut signals: Signals,
    ctx: Context,
    extra_precision: ExtraPrecision,
) -> D<N> {
    let cb = if exp > i16::MAX as i32 + 1 {
        signals.raise(Signals::OP_CLAMPED);
        ControlBlock::new(i16::MIN, sign, signals, ctx, extra_precision)
    } else if exp <= i16::MIN as i32 {
        signals.raise(Signals::OP_CLAMPED);
        ControlBlock::new(i16::MAX, sign, signals, ctx, extra_precision)
    } else {
        ControlBlock::new(-exp as i16, sign, signals, ctx, extra_precision)
    };

    D::new(UInt::ZERO, cb)
}
