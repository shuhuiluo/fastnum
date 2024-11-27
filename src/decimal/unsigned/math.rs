use core::cmp::Ordering;

use crate::{
    decimal::{
        math::{result, DecimalResult, Flags},
        round::{round, scale_round, RoundConsts},
        unsigned::{round::with_scale, UnsignedDecimal},
        RoundingMode,
    },
    int::{
        math::{div_rem, div_rem_wide},
        UInt,
    },
};

type UD<const N: usize> = UnsignedDecimal<N>;

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
    mut lhs: UD<N>,
    mut rhs: UD<N>,
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
        let flags_ov;
        (lhs, flags_ov) = with_scale(lhs, rhs.scale, rounding_mode).split();
        
        if flags_ov.contains(Flags::OVERFLOW) {
            let flags;
            (rhs, flags) = with_scale(rhs, lhs.scale, rounding_mode).split();
            sub_aligned(lhs, rhs).add_flags(flags_ov.overflow_to_inexact()).add_flags(flags)
        } else {
            sub_aligned(lhs, rhs).add_flags(flags_ov)
        }
    } else {
        let flags_ov;
        (rhs, flags_ov) = with_scale(rhs, lhs.scale, rounding_mode).split();

        if flags_ov.contains(Flags::OVERFLOW) {
            let flags;
            (lhs, flags) = with_scale(lhs, rhs.scale, rounding_mode).split();
            sub_aligned(lhs, rhs).add_flags(flags_ov.overflow_to_inexact()).add_flags(flags)
        } else {
            sub_aligned(lhs, rhs).add_flags(flags_ov)
        }
    }
}

#[inline]
pub(crate) const fn mul<const N: usize>(
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

        let (mut low, mut high) = lhs.value.widening_mul(rhs.value);

        let mut out;
        let mut rem;

        if high.is_zero() {
            let value = UD::new(low, scale);
            return result!(value);
        }

        while !high.is_zero() {
            (scale, overflow) = scale.overflowing_sub(1);

            if overflow {
                return result!(UD::new(UInt::MAX, i64::MIN)).overflow();
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
                // TODO
                low = round(low, UInt::from_digit(rem), rounding_mode);
            }
        }

        let value = UD::new(low, scale);
        result!(value).inexact()
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
                    let (digit, _) = scale_round(quotient, rounding_mode);
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
pub(crate) const fn rem<const N: usize>(
    lhs: UD<N>,
    rhs: UD<N>,
    rounding_mode: RoundingMode,
) -> DecimalResult<UD<N>> {
    let scale = if lhs.scale >= rhs.scale {
        lhs.scale
    } else {
        rhs.scale
    };

    let (num, flags_num) = with_scale(lhs, scale, rounding_mode).split();
    let (den, flags_den) = with_scale(rhs, scale, rounding_mode).split();

    if num.scale != den.scale {
        return result!(lhs).overflow();
    }

    result!(UD::new(num.value.rem(den.value), scale))
        .add_flags(flags_num)
        .add_flags(flags_den)
}

#[inline]
const fn add_rescale<const N: usize>(
    mut lhs: UD<N>,
    mut rhs: UD<N>,
    rounding_mode: RoundingMode,
) -> DecimalResult<UD<N>> {
    let mut flags;
    (lhs, flags) = with_scale(lhs, rhs.scale, rounding_mode).split();
    if flags.contains(Flags::OVERFLOW) {
        (rhs, flags) = with_scale(rhs, lhs.scale, rounding_mode).split();
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
    debug_assert!(lhs.scale == rhs.scale);
    
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

        (lhs, flags) = with_scale(lhs, scale, rounding_mode).split();
        add_aligned(lhs, rhs, rounding_mode)
            .add_flags(flags)
            .inexact()
    }
}

#[inline]
const fn sub_aligned<const N: usize>(lhs: UD<N>, rhs: UD<N>) -> DecimalResult<UD<N>> {
    debug_assert!(lhs.scale == rhs.scale);
    
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
        with_scale(dec, new_scale, rounding_mode)
    } else {
        result!(dec)
    }
}
