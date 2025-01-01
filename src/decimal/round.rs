use core::cmp::Ordering::{Equal, Greater, Less};

use crate::{
    decimal::{
        Context,
        RoundingMode::{self, Ceiling, Down, Floor, HalfDown, HalfEven, HalfUp, Up},
        Sign,
    },
    int::{intrinsics::Digit, math::div_rem_digit, UInt},
};

#[inline]
pub(crate) const fn scale_round<const N: usize>(
    mut value: UInt<N>,
    sign: Sign,
    ctx: Context,
) -> (UInt<N>, bool) {
    let remainder;

    (value, remainder) = div_rem_digit(value, 10);

    if remainder == 0 {
        (value, false)
    } else {
        (round(value, remainder, sign, ctx), true)
    }
}

#[inline]
pub(crate) const fn round<const N: usize>(
    mut value: UInt<N>,
    remainder: Digit,
    sign: Sign,
    ctx: Context,
) -> UInt<N> {
    if match ctx.rounding_mode() {
        Up => true,
        Down => false,
        Ceiling => !matches!(sign, Sign::Minus),
        Floor => matches!(sign, Sign::Minus),
        HalfUp => remainder >= 5,
        HalfDown => remainder > 5,
        HalfEven => {
            if remainder > 5 {
                true
            } else if remainder == 5 {
                let last_digit = value.digits()[0];
                let last_bit = last_digit & 0x0000_0000_0000_0001_u64;
                last_bit != 0
            } else {
                false
            }
        }
    } {
        value = value.strict_add(UInt::ONE);
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
