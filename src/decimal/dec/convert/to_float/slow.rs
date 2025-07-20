//! Arbitrary-precision decimal type used by fallback algorithms.
//!
//! This is only used if the fast-path (native floats) and
//! the Eisel-Lemire algorithm are unable to unambiguously
//! determine the float.
//!
//! The technique used is "Simple Decimal Conversion", developed
//! by Nigel Tao and Ken Thompson. A detailed description of the
//! algorithm can be found in "ParseNumberF64 by Simple Decimal Conversion",
//! available online: <https://nigeltao.github.io/blog/2020/parse-number-f64-simple.html>.

use crate::{
    bint::UInt,
    decimal::{
        dec::{convert::to_float::float, math::mul::mul},
        Decimal, RoundingMode, Sign,
    },
};

type D<const N: usize> = Decimal<N>;
type U<const N: usize> = UInt<N>;

/// The integral (non-fractional) part of d, if it is 18 or fewer
/// decimal digits.
///
/// For 19 or more digits, it returns [u64::MAX].
///
/// Note that:
///   - (1 << 53) is    9007199254740992, which has 16 decimal digits.
///   - (1 << 56) is   72057594037927936, which has 17 decimal digits.
///   - (1 << 59) is  576460752303423488, which has 18 decimal digits.
///   - (1 << 63) is 9223372036854775808, which has 19 decimal digits.
///
/// IEEE 754 double precision has 52 mantissa bits.
///
/// That integral part is rounded-to-even: rounding 7.5 or 8.5 both give 8.
#[inline]
const fn decimal_round<const N: usize>(d: D<N>) -> u64 {
    // TODO: performance optimization
    let rounded = d.with_rounding_mode(RoundingMode::HalfEven).round(0);

    let digits = rounded.transmute().digits();

    if digits.ge(&U::<1>::MAX) {
        return u64::MAX;
    }

    digits.digits()[0]
}

#[inline]
const fn fast_mul_power_two<const N: usize>(d: &mut D<N>, mut power: u32) {
    // TODO: performance optimization
    while power > 0 {
        let shift = if power <= U::<N>::MAX_POWER_OF_TWO {
            power
        } else {
            U::<N>::MAX_POWER_OF_TWO
        };

        *d = mul(
            *d,
            D::from_parts(U::power_of_two(shift), 0, Sign::Plus, d.context()),
        );
        power -= shift;
    }
}

#[inline]
const fn fast_div_power_two<const N: usize>(d: &mut D<N>, mut power: u32) {
    // TODO: performance optimization
    while power > 0 {
        let shift = if power <= U::<N>::MAX_POWER_OF_FIVE {
            power
        } else {
            U::<N>::MAX_POWER_OF_FIVE
        };

        *d = mul(
            *d,
            D::from_parts(
                U::power_of_five(shift),
                -(shift as i32),
                Sign::Plus,
                d.context(),
            ),
        );
        power -= shift;
    }
}

macro_rules! to_float_slow_impl {
    ($to_f: ident, $f: ident) => {
        #[inline]
        pub const fn $to_f<const N: usize>(mut d: D<N>) -> $f {
            use float::$f::*;

            // Decimal to binary floating point conversion.
            // "Simple Decimal Conversion" Algorithm:
            //   1) Store input in multiprecision decimal.
            //   2) Multiply/divide decimal by powers of two until in range [0.5, 1)
            //   3) Multiply by 2^precision and round to get mantissa.

            let mut exp2 = 0_i32;

            // Scale by powers of 2 until we're in the range [½ .. 1], which gives us
            // our exponent (in base-2). First we shift right, possibly a little too
            // far, ending with a value certainly below 1 and possibly below ½...
            while true {
                let decimal_point = d.digits.ilog10() as i32 + 1 - d.cb.get_scale() as i32;

                if decimal_point > INFINITE_POWER {
                    return $f::INFINITY;
                } else if decimal_point < -INFINITE_POWER {
                    return 0.0;
                }

                if decimal_point <= 0 {
                    break;
                }

                let shift = decimal_point as u32;
                fast_div_power_two(&mut d, shift);
                exp2 += shift as i32;
            }

            // ...then we shift left, putting us in [½ .. 1].
            // TODO: performance optimization
            while d.lt(&D::HALF) {
                let decimal_point = d.digits.ilog10() as i32 + 1 - d.cb.get_scale() as i32;

                debug_assert!(decimal_point <= 0);

                if decimal_point > INFINITE_POWER {
                    return $f::INFINITY;
                } else if decimal_point < -INFINITE_POWER {
                    return 0.0;
                }

                let shift = if decimal_point == 0 {
                    1
                } else {
                    -decimal_point as u32
                };
                fast_mul_power_two(&mut d, shift);
                exp2 -= shift as i32;
            }

            // We're in the range [½ .. 1] but f64 uses [1 .. 2].
            exp2 -= 1;

            // The minimum normal exponent is EXP_MIN.
            while EXP_MIN > exp2 {
                fast_div_power_two(&mut d, 1);
                exp2 += 1;
            }

            // Check for overflow.
            if (exp2 - EXP_MIN + 1) >= INFINITE_POWER {
                return $f::INFINITY;
            }

            // Extract 53 bits for the mantissa (in base-2).
            fast_mul_power_two(&mut d, SIG_BITS + 1);
            let mut mantissa = decimal_round(d);

            // Rounding might have added one bit. If so, shift and re-check overflow.
            if ((mantissa >> (SIG_BITS + 1)) != 0) {
                mantissa >>= 1;
                exp2 += 1;

                if ((exp2 - EXP_MIN + 1) >= INFINITE_POWER) {
                    // (1 << 11) - 1.
                    return $f::INFINITY;
                }
            }

            let mut power2 = exp2 - EXP_MIN + 1;
            if mantissa < (1_u64 << SIG_BITS) {
                power2 -= 1;
            }

            mantissa &= (1_u64 << SIG_BITS) - 1;

            float(mantissa, power2)
        }
    };
}

to_float_slow_impl!(to_f64, f64);
to_float_slow_impl!(to_f32, f32);
