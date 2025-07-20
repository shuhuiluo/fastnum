use crate::{bint::intrinsics::DIGIT_POWERS_10, decimal::dec::convert::to_float::float};

macro_rules! to_float_fast_impl {
    ($to_f: ident, $f: ident) => {
        #[inline(always)]
        pub const fn $to_f(mantissa: u64, exponent: i32) -> Option<$f> {
            use float::$f::*;

            let value = if exponent <= MAX_EXPONENT_FAST_PATH {
                // normal fast path
                let value = from_u64(mantissa);
                if exponent < 0 {
                    value / pow10_fast_path((-exponent) as _)
                } else {
                    value * pow10_fast_path(exponent as _)
                }
            } else {
                // disguised fast path
                let shift = exponent - MAX_EXPONENT_FAST_PATH;

                let Some(mantissa) = mantissa.checked_mul(DIGIT_POWERS_10[shift as usize]) else {
                    return None;
                };

                if mantissa > MAX_MANTISSA_FAST_PATH {
                    return None;
                }
                from_u64(mantissa) * pow10_fast_path(MAX_EXPONENT_FAST_PATH as _)
            };

            Some(value)
        }
    };
}

to_float_fast_impl!(to_f64, f64);
to_float_fast_impl!(to_f32, f32);
