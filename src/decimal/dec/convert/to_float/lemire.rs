//! Implementation of the Eisel-Lemire algorithm.

mod table;
mod utils;

use utils::*;

use crate::decimal::dec::convert::to_float::{float, utils::*};

macro_rules! to_float_lemire_impl {
    ($to_f: ident, $f: ident) => {
        /// Compute w * 10^q using an extended-precision float representation.
        ///
        /// Fast conversion of the significant digits and decimal exponent
        /// a float to an extended representation with a binary float. This
        /// algorithm will accurately parse the vast majority of cases,
        /// and uses a 128-bit representation with a significand containing
        /// no more than 19 digits into an IEEE floating-point number.
        ///
        /// This algorithm scales the exponent by the decimal exponent
        /// using pre-computed powers-of-5, and calculates if the
        /// representation can be unambiguously rounded to the nearest
        /// machine float. Near-halfway cases are not handled here,
        /// and are represented by a negative, biased binary exponent.
        ///
        /// The algorithm is described in detail in "Daniel Lemire, Number Parsing
        /// at a Gigabyte per Second" in section 5, "Fast Algorithm", and
        /// section 6, "Exact Numbers And Ties", available online:
        /// <https://arxiv.org/abs/2101.11408.pdf>.
        ///
        /// First implementation of algorithm had a check leading to a fallback function
        /// to ensure correctness. This fallback function is never called in practice.
        ///
        /// A little later Noble MushTak, Daniel Lemire in their article
        /// ["Fast number parsing without fallback"](https://doi.org/10.1002/spe.3198) proved
        /// that the fallback is unnecessary both f32 and f64 conversions.
        pub const fn $to_f(mut w: u64, q: i32) -> $f {
            use float::$f::*;

            debug_assert!(w != 0);
            debug_assert!(q >= MIN_10_EXP_REAL);
            debug_assert!(q <= MAX_10_EXP);

            // Normalize our significant digits, so the most-significant bit is set.
            let lz = w.leading_zeros();
            w <<= lz;

            let (lo, hi) = compute_product_approx(q, w, SIG_BITS as usize + 3);

            let upperbit = (hi >> 63) as i32;
            let mut mantissa = hi >> (upperbit + 64 - SIG_BITS as i32 - 3);
            let mut power2 = power(q) + upperbit - lz as i32 - EXP_MIN + 1;
            if power2 <= 0 {
                if -power2 + 1 >= 64 {
                    // Have more than 64 bits below the minimum exponent, must be 0.
                    return 0.0;
                }
                // Have a subnormal value.
                mantissa >>= -power2 + 1;
                mantissa += mantissa & 1;
                mantissa >>= 1;
                power2 = (mantissa >= (1_u64 << SIG_BITS)) as i32;

                return float(mantissa, power2);
            }
            // Need to handle rounding ties. Normally, we need to round up,
            // but if we fall right in between and we have an even basis, we
            // need to round down.
            //
            // This will only occur if:
            //  1. The lower 64 bits of the 128-bit representation is 0. IE, 5^q fits in
            //     single 64-bit word.
            //  2. The least-significant bit prior to truncated mantissa is odd.
            //  3. All the bits truncated when shifting to mantissa bits + 1 are 0.
            //
            // Or, we may fall between two floats: we are exactly halfway.
            if lo <= 1
                && q >= MIN_EXPONENT_ROUND_TO_EVEN
                && q <= MAX_EXPONENT_ROUND_TO_EVEN
                && mantissa & 0b11 == 0b01
                && (mantissa << (upperbit + 64 - SIG_BITS as i32 - 3)) == hi
            {
                // Zero the lowest bit, so we don't round up.
                mantissa &= !1_u64;
            }
            // Round-to-even, then shift the significant digits into place.
            mantissa += mantissa & 1;
            mantissa >>= 1;
            if mantissa >= (2_u64 << SIG_BITS) {
                // Rounding up overflowed, so the carry bit is set. Set the
                // mantissa to 1 (only the implicit, hidden bit is set) and
                // increase the exponent.
                mantissa = 1_u64 << SIG_BITS;
                power2 += 1;
            }
            // Zero out the hidden bit.
            mantissa &= !(1_u64 << SIG_BITS);
            if power2 >= INFINITE_POWER {
                // Exponent is above largest normal value, must be infinite.
                return $f::INFINITY;
            }

            float(mantissa, power2)
        }
    };
}

to_float_lemire_impl!(to_f64, f64);
to_float_lemire_impl!(to_f32, f32);
