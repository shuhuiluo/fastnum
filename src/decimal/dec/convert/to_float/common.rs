use crate::{
    bint::UInt,
    decimal::{
        dec::convert::to_float::{fast, float, lemire, slow, utils},
        Decimal,
    },
};

type D<const N: usize> = Decimal<N>;
type U<const N: usize> = UInt<N>;

macro_rules! to_float_common_impl {
    ($to_f: ident, $f: ident) => {
        #[inline(always)]
        pub const fn $to_f<const N: usize>(d: D<N>) -> $f {
            use float::$f::*;

            if d.is_infinite() {
                return $f::INFINITY;
            }

            if d.is_zero() {
                return 0.0;
            }

            debug_assert!(!d.cb.is_special());
            debug_assert!(!d.digits.is_zero());

            let d_exp = d.cb.get_exponent();

            let bits = d.digits.bits();
            let digits = d.digits;

            if bits <= u64::BITS {
                let w = digits.digits()[0];

                if d_exp < MIN_10_EXP_REAL {
                    return 0.0;
                } else if d_exp > MAX_10_EXP {
                    return $f::INFINITY;
                }

                if d_exp >= MIN_EXPONENT_FAST_PATH
                    && d_exp <= MAX_EXPONENT_DISGUISED_FAST_PATH
                    && w <= MAX_MANTISSA_FAST_PATH
                {
                    if let Some(value) = fast::$to_f(w, d_exp) {
                        return value;
                    }
                }

                return lemire::$to_f(w, d_exp);
            }

            let utils::TruncatedResult {
                digits: w,
                exp,
                is_inexact,
            } = utils::truncate(digits, d_exp);

            if exp < MIN_10_EXP_REAL {
                return 0.0;
            } else if exp > MAX_10_EXP {
                return $f::INFINITY;
            }

            if !is_inexact {
                return $to_f(D::from_parts(
                    U::<1>::from_digit(w),
                    exp,
                    d.sign(),
                    d.context(),
                ));
            }

            if w < u64::MAX {
                let approx = lemire::$to_f(w, exp);
                let approx_next = lemire::$to_f(w + 1, exp);

                if approx == approx_next {
                    return approx;
                }
            }

            slow::$to_f(d)
        }
    };
}

to_float_common_impl!(to_f64, f64);
to_float_common_impl!(to_f32, f32);
