use core::cmp::Ordering::{Equal, Greater, Less};

use crate::{
    decimal::{
        Context,
        RoundingMode::{self, Ceiling, Down, Floor, HalfDown, HalfEven, HalfUp, Up},
        Sign,
    },
    int::{math::div_rem, UInt},
};

#[inline]
pub(crate) const fn scale_round<const N: usize>(
    mut value: UInt<N>,
    ctx: Context,
) -> (UInt<N>, bool) {
    let remainder;

    (value, remainder) = div_rem(value, UInt::<N>::TEN);

    if !remainder.is_zero() {
        (round(value, remainder, ctx), true)
    } else {
        (value, false)
    }
}

#[inline]
pub(crate) const fn round<const N: usize>(
    mut value: UInt<N>,
    remainder: UInt<N>,
    ctx: Context,
) -> UInt<N> {
    // TODO: performance optimization
    match (ctx.rounding_mode(), remainder.cmp(&UInt::FIVE)) {
        (Up, _) | (Ceiling, _) => {
            value = value.strict_add(UInt::ONE);
        }
        (Down, _) | (Floor, _) => {
            // Do nothing
        }
        (_, Greater) | (HalfUp, Equal) => {
            value = value.strict_add(UInt::ONE);
        }
        (_, Less) | (HalfDown, Equal) => {
            // Do nothing
        }
        (HalfEven, Equal) => {
            // TODO: performance optimization
            if value.strict_rem(UInt::TWO).is_zero() {
                // Do nothing
            } else {
                value = value.strict_add(UInt::ONE);
            }
        }
    }
    value
}

// TODO: refactor
pub(crate) fn round_pair_digits(
    pair: (u8, u8),
    sign: Sign,
    rounding_mode: RoundingMode,
    trailing_zeros: bool,
) -> u8 {
    use self::RoundingMode::*;

    let (lhs, rhs) = pair;
    // if all zero after digit, never round
    if rhs == 0 && trailing_zeros {
        return lhs;
    }
    let up = lhs + 1;
    let down = lhs;
    match (rounding_mode, rhs.cmp(&5)) {
        (Up, _) => up,
        (Down, _) => down,
        (Floor, _) => {
            if sign == Sign::Minus {
                up
            } else {
                down
            }
        }
        (Ceiling, _) => {
            if sign == Sign::Minus {
                down
            } else {
                up
            }
        }
        (_, Less) => down,
        (_, Greater) => up,
        (_, Equal) if !trailing_zeros => up,
        (HalfUp, Equal) => up,
        (HalfDown, Equal) => down,
        (HalfEven, Equal) => {
            if lhs % 2 == 0 {
                down
            } else {
                up
            }
        }
    }
}
