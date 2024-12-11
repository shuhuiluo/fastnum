use num_traits::ToPrimitive;

use crate::decimal::{utils::types, Context, Decimal};

macro_rules! to_int_impl {
    ($to_int: ident, $int: ty, $to_uint: ident) => {
        #[inline]
        fn $to_int(&self) -> Option<$int> {
            if self.flags().is_special() {
                return None;
            }

            if self.is_negative() {
                self.with_scale(0, Context::default())
                    .digits
                    .$to_uint()
                    .and_then(|n| (0 as $int).checked_sub_unsigned(n))
            } else {
                self.with_scale(0, Context::default()).digits.$to_int()
            }
        }
    };
}

macro_rules! to_uint_impl {
    ($to_uint: ident, $uint: ty) => {
        #[inline]
        fn $to_uint(&self) -> Option<$uint> {
            if self.flags().is_special() || self.is_negative() {
                return None;
            }

            self.with_scale(0, Context::default()).digits.$to_uint()
        }
    };
}

macro_rules! to_float_impl {
    ($to_f: ident, $f: ident) => {
        #[inline]
        fn $to_f(&self) -> Option<$f> {
            if self.is_nan() {
                return Some($f::NAN);
            }

            if self.is_infinite() {
                return if self.is_negative() {
                    Some($f::NEG_INFINITY)
                } else {
                    Some($f::INFINITY)
                };
            }

            self.digits.$to_f().and_then(|x| {
                self.scale.checked_neg().and_then(|scale| {
                    scale.to_i32().map(|n| {
                        let sign = if self.is_negative() { -1.0 } else { 1.0 };
                        sign * x * types::$f::powi(10 as $f, n)
                    })
                })
            })
        }
    };
}

impl<const N: usize> ToPrimitive for Decimal<N> {
    to_int_impl!(to_isize, isize, to_usize);
    to_int_impl!(to_i8, i8, to_u8);
    to_int_impl!(to_i16, i16, to_u16);
    to_int_impl!(to_i32, i32, to_u32);
    to_int_impl!(to_i64, i64, to_u64);
    to_int_impl!(to_i128, i128, to_u128);

    to_uint_impl!(to_usize, usize);
    to_uint_impl!(to_u8, u8);
    to_uint_impl!(to_u16, u16);
    to_uint_impl!(to_u32, u32);
    to_uint_impl!(to_u64, u64);
    to_uint_impl!(to_u128, u128);

    to_float_impl!(to_f32, f32);
    to_float_impl!(to_f64, f64);
}
