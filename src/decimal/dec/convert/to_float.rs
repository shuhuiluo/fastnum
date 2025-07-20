mod common;
mod fast;
mod float;
mod lemire;
mod slow;
mod utils;

use crate::decimal::Decimal;

type D<const N: usize> = Decimal<N>;

macro_rules! to_float_impl {
    ($to_f: ident, $f: ident) => {
        #[inline(always)]
        pub const fn $to_f<const N: usize>(d: D<N>) -> $f {
            if d.is_nan() {
                return $f::NAN;
            }

            let sign = d.is_negative();
            let res = common::$to_f(d.abs());

            if sign {
                -res
            } else {
                res
            }
        }
    };
}

to_float_impl!(to_f32, f32);
to_float_impl!(to_f64, f64);
