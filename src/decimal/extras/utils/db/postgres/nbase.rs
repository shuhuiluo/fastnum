use core::cmp::Ordering;
use num_traits::Euclid;

use crate::{
    bint::UInt,
    decimal::{dec::ControlBlock, Decimal, ParseError, Sign},
};

type D<const N: usize> = Decimal<N>;

pub enum NBase {
    /// A positive number
    Positive {
        /// How many digits come before the decimal point?
        weight: i16,
        /// How many significant digits are there?
        scale: u16,
        /// The digits in this number, stored in base 10,000
        digits: Vec<i16>,
    },
    /// A negative number
    Negative {
        /// How many digits come before the decimal point?
        weight: i16,
        /// How many significant digits are there?
        scale: u16,
        /// The digits in this number, stored in base 10,000
        digits: Vec<i16>,
    },
    /// Not a number
    NaN,
}

pub const NBASE: u64 = 10_000;

struct PowerCoefficient {
    multiplier: u64,
    divider: i16,
}

const POWERS: [PowerCoefficient; 4] = [
    PowerCoefficient {
        multiplier: NBASE,
        divider: 1,
    },
    PowerCoefficient {
        multiplier: NBASE / 10,
        divider: 10,
    },
    PowerCoefficient {
        multiplier: NBASE / 100,
        divider: 100,
    },
    PowerCoefficient {
        multiplier: NBASE / 1000,
        divider: 1000,
    },
];

macro_rules! checked {
    ($a: ident *= {$b: expr}) => {
        $a = $a.checked_mul_digit($b).ok_or(ParseError::PosOverflow)?;
    };
    ($a: ident *= $b: expr) => {
        $a = $a.checked_mul($b).ok_or(ParseError::PosOverflow)?;
    };
    ($a: ident += {$b: expr}) => {
        $a = $a.checked_add_digit($b).ok_or(ParseError::PosOverflow)?;
    };
    ($a: ident += $b: expr) => {
        $a = $a.checked_add($b).ok_or(ParseError::PosOverflow)?;
    };
    ($a: ident /= {$b: expr}) => {
        $a = $a.checked_div_digit($b).ok_or(ParseError::PosOverflow)?;
    };
    ($a: ident /= $b: expr) => {
        $a = $a.checked_div($b).ok_or(ParseError::PosOverflow)?;
    };
}

impl<const N: usize> TryFrom<D<N>> for NBase {
    type Error = ParseError;

    fn try_from(dec: D<N>) -> Result<Self, Self::Error> {
        if dec.is_nan() {
            return Ok(Self::NaN);
        }

        if dec.is_infinite() {
            return if dec.is_negative() {
                Err(ParseError::NegOverflow)
            } else {
                Err(ParseError::PosOverflow)
            };
        }

        let mut uint = dec.digits();

        if uint.is_zero() {
            return if dec.is_negative() {
                Ok(Self::Negative {
                    weight: 0,
                    scale: 0,
                    digits: vec![],
                })
            } else {
                Ok(Self::Positive {
                    weight: 0,
                    scale: 0,
                    digits: vec![],
                })
            };
        }

        let mut scale = dec.fractional_digits_count();
        let mut digits = Vec::with_capacity(0);
        let mut weight = 0;
        let mut exp = 0;

        if scale < 0 {
            (weight, exp) = (-scale).div_rem_euclid(&4);
            scale = 0;
        }

        exp += 4 - scale % 4;

        while !uint.is_zero() {
            let correction = UInt::power_of_ten(exp as u32);
            let (div, rem) = uint.mul_div_rem(correction, UInt::from_digit(NBASE));

            if !digits.is_empty() || !rem.is_zero() {
                digits.push(rem.to_i16().expect("10000 always fits in an i16"));
            }

            uint = div;
            exp = 0;
            weight += 1;
        }

        digits.reverse();

        let weight = weight - (scale / 4 + 1) - 1;

        if dec.is_negative() {
            Ok(Self::Negative {
                weight,
                scale: scale as u16,
                digits,
            })
        } else {
            Ok(Self::Positive {
                weight,
                scale: scale as u16,
                digits,
            })
        }
    }
}

impl<const N: usize> TryFrom<NBase> for D<N> {
    type Error = ParseError;

    fn try_from(value: NBase) -> Result<Self, Self::Error> {
        let (sign, weight, scale, digits) = match value {
            NBase::Positive {
                weight,
                scale,
                digits,
            } => (Sign::Plus, weight, scale, digits),
            NBase::Negative {
                weight,
                scale,
                digits,
            } => (Sign::Minus, weight, scale, digits),
            NBase::NaN => {
                return Ok(Self::NAN);
            }
        };

        let count = i16::try_from(digits.len()).map_err(|_| ParseError::PosOverflow)?;
        let scale = i16::try_from(scale).map_err(|_| ParseError::ExponentOverflow)?;

        let mut uint = UInt::<N>::ZERO;

        let num_power = scale + ((weight + 1) * 4);
        let trailing_zeros = ((count * 4) - num_power).max(0);

        debug_assert!(trailing_zeros >= 0);
        debug_assert!(trailing_zeros <= 4);

        if let Some((last, digits)) = digits.split_last() {
            for digit in digits {
                let d = u64::try_from(*digit).map_err(|_| ParseError::InvalidLiteral)?;
                checked!(uint *= { NBASE });
                checked!(uint += UInt::from_digit(d));
            }

            if trailing_zeros < 4 {
                let coefficient = &POWERS[trailing_zeros as usize];

                let d = u64::try_from((*last) / coefficient.divider)
                    .map_err(|_| ParseError::InvalidLiteral)?;
                let multiplier = coefficient.multiplier;

                checked!(uint *= { multiplier });
                checked!(uint += { d });
            }
        }

        let correction_exp = -(4 * (weight - (count - 1))) - trailing_zeros;
        match scale.cmp(&correction_exp) {
            Ordering::Greater => {
                let scale_diff =
                    u32::try_from(scale - correction_exp).map_err(|_| ParseError::PosOverflow)?;
                let correction =
                    UInt::checked_power_of_ten(scale_diff).ok_or(ParseError::PosOverflow)?;
                checked!(uint *= correction);
            }
            Ordering::Less => {
                let scale_diff =
                    u32::try_from(correction_exp - scale).map_err(|_| ParseError::PosOverflow)?;
                let correction =
                    UInt::checked_power_of_ten(scale_diff).ok_or(ParseError::PosOverflow)?;
                checked!(uint /= correction);
            }
            Ordering::Equal => {}
        }

        Ok(D::new(uint, ControlBlock::basic(scale, sign)))
    }
}
