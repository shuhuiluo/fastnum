use core::cmp::Ordering;

use crate::{
    decimal::{
        math::{result, DecimalResult, Flags},
        round::{round, RoundConsts},
        unsigned::{round, UnsignedDecimal},
        RoundingMode,
    },
    int::{
        math::{cast, div_rem},
        UInt,
    },
};

type UD<const N: usize> = UnsignedDecimal<N>;

// TODO: Replace DN with {2 * N} and remove all upward macros when `generic_const_exprs` is stabilized: https://github.com/rust-lang/rust/issues/76560

struct MulConsts<const N: usize, const DN: usize>;

impl<const N: usize, const DN: usize> MulConsts<N, DN> {
    pub const MAX: UInt<DN> = cast::<N, DN>(UInt::<N>::MAX);
}

#[inline]
pub(crate) const fn add<const N: usize>(
    lhs: UD<N>,
    rhs: UD<N>,
    rounding_mode: RoundingMode,
) -> DecimalResult<UD<N>> {
    if rhs.is_zero() {
        return extend_scale_to(lhs, rhs.scale, rounding_mode).overflow_to_inexact();
    }

    if lhs.is_zero() {
        return extend_scale_to(rhs, lhs.scale, rounding_mode).overflow_to_inexact();
    }

    if lhs.scale == rhs.scale {
        add_aligned(lhs, rhs, rounding_mode)
    } else if lhs.scale < rhs.scale {
        add_rescale(lhs, rhs, rounding_mode)
    } else {
        add_rescale(rhs, lhs, rounding_mode)
    }
}

#[inline]
pub(crate) const fn sub<const N: usize>(
    lhs: UD<N>,
    rhs: UD<N>,
    rounding_mode: RoundingMode,
) -> DecimalResult<UD<N>> {
    if rhs.is_zero() {
        return extend_scale_to(lhs, rhs.scale, rounding_mode).overflow_to_inexact();
    }

    if lhs.is_zero() {
        return result!(lhs).negative();
    }

    if lhs.scale == rhs.scale {
        sub_aligned(lhs, rhs)
    } else if lhs.scale < rhs.scale {
        let (lhs, flags) = round::with_scale(lhs, rhs.scale, rounding_mode).split();
        sub_aligned(lhs, rhs).add_flags(flags)
    } else {
        let (rhs, flags) = round::with_scale(rhs, lhs.scale, rounding_mode).split();
        sub_aligned(lhs, rhs).add_flags(flags)
    }
}

#[inline]
pub(crate) const fn mul<const N: usize, const DN: usize>(
    lhs: UD<N>,
    rhs: UD<N>,
    rounding_mode: RoundingMode,
) -> DecimalResult<UD<N>> {
    if lhs.is_one() {
        result!(rhs)
    } else if rhs.is_one() {
        result!(lhs)
    } else {
        let (mut scale, mut overflow) = lhs.scale.overflowing_add(rhs.scale);

        if overflow {
            scale = lhs.scale.saturating_add(rhs.scale);
            return result!(UD::new(UInt::MAX, scale)).overflow();
        }

        let mut value = cast::<N, DN>(lhs.value);
        value = value.strict_mul(cast::<N, DN>(rhs.value));

        if value.gt(&MulConsts::<N, DN>::MAX) {
            while value.gt(&MulConsts::<N, DN>::MAX) {
                (scale, overflow) = scale.overflowing_sub(1);

                if overflow {
                    return result!(UD::new(UInt::MAX, i64::MIN)).overflow();
                }

                (value, _) = round(value, rounding_mode);
            }

            let value = UD::new(cast(value), scale);
            result!(value).inexact()
        } else {
            let value = UD::new(cast(value), scale);
            result!(value)
        }
    }
}

#[inline]
pub(crate) const fn div<const N: usize>(
    dividend: UD<N>,
    divisor: UD<N>,
    rounding_mode: RoundingMode,
) -> DecimalResult<UD<N>> {
    if divisor.is_zero() {
        result!(dividend).div_by_zero()
    } else if dividend.is_zero() || divisor.is_one() {
        result!(dividend)
    } else {
        let (mut scale, mut overflow) = dividend.scale.overflowing_sub(divisor.scale);

        if overflow {
            scale = dividend.scale.saturating_sub(divisor.scale);
            return result!(UD::new(UInt::MAX, scale)).overflow();
        }

        let (mut value, mut remainder) = div_rem(dividend.value, divisor.value);

        if !remainder.is_zero() {
            let mut quotient;

            while !remainder.is_zero() {
                (remainder, overflow) = remainder.overflowing_mul(UInt::TEN);

                if overflow {
                    return result!(UnsignedDecimal::new(value, scale)).inexact();
                }

                (quotient, remainder) = div_rem(remainder, divisor.value);

                if value.gt(&RoundConsts::MAX) {
                    // TODO: performance optimizations
                    let (digit, _) = round(quotient, rounding_mode);
                    if digit.is_one() {
                        value = value.saturating_add(digit);
                    }

                    return result!(UnsignedDecimal::new(value, scale)).inexact();
                }

                value = value.strict_mul(UInt::TEN);

                (scale, overflow) = scale.overflowing_add(1);
                if overflow {
                    return result!(UD::new(UInt::MAX, i64::MAX)).overflow();
                }

                if value.gt(&UInt::MAX.strict_sub(quotient)) {
                    return result!(UnsignedDecimal::new(UInt::MAX, scale)).inexact();
                }

                value = value.strict_add(quotient);
            }
        }

        let value = UnsignedDecimal::new(value, scale);
        result!(value)
    }
}

#[inline]
const fn add_rescale<const N: usize>(
    mut lhs: UD<N>,
    mut rhs: UD<N>,
    rounding_mode: RoundingMode,
) -> DecimalResult<UD<N>> {
    let mut flags;
    (lhs, flags) = round::with_scale(lhs, rhs.scale, rounding_mode).split();
    if flags.contains(Flags::OVERFLOW) {
        (rhs, flags) = round::with_scale(rhs, lhs.scale, rounding_mode).split();
        add_aligned(lhs, rhs, rounding_mode).add_flags(flags)
    } else {
        add_aligned(lhs, rhs, rounding_mode).add_flags(flags)
    }
}

#[inline]
const fn add_aligned<const N: usize>(
    mut lhs: UD<N>,
    mut rhs: UD<N>,
    rounding_mode: RoundingMode,
) -> DecimalResult<UD<N>> {
    let mut overflow;

    (lhs.value, overflow) = lhs.value.overflowing_add(rhs.value);

    if !overflow {
        result!(lhs)
    } else {
        rhs.value = RoundConsts::MAX;
        rhs.value = rhs.value.strict_add(UInt::ONE);
        (rhs.scale, overflow) = rhs.scale.overflowing_sub(1);

        if overflow {
            return result!(lhs).overflow();
        }

        let scale;
        (scale, overflow) = lhs.scale.overflowing_sub(1);

        if overflow {
            return result!(lhs).overflow();
        }

        let flags;

        (lhs, flags) = round::with_scale(lhs, scale, rounding_mode).split();
        add_aligned(lhs, rhs, rounding_mode)
            .add_flags(flags)
            .inexact()
    }
}

#[inline]
const fn sub_aligned<const N: usize>(lhs: UD<N>, rhs: UD<N>) -> DecimalResult<UD<N>> {
    match lhs.value.cmp(&rhs.value) {
        Ordering::Less => result!(lhs).negative(),
        Ordering::Equal => {
            result!(UD::ZERO)
        }
        Ordering::Greater => {
            result!(UD::new(lhs.value.strict_sub(rhs.value), lhs.scale))
        }
    }
}

#[inline]
const fn extend_scale_to<const N: usize>(
    dec: UD<N>,
    new_scale: i64,
    rounding_mode: RoundingMode,
) -> DecimalResult<UD<N>> {
    if new_scale > dec.scale {
        round::with_scale(dec, new_scale, rounding_mode)
    } else {
        result!(dec)
    }
}
