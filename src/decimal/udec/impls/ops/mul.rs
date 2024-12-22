use core::ops::{Mul, MulAssign};

use crate::decimal::UnsignedDecimal;

impl<const N: usize> Mul for UnsignedDecimal<N> {
    type Output = UnsignedDecimal<N>;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.mul(rhs)
    }
}

impl<const N: usize> MulAssign for UnsignedDecimal<N> {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = Mul::<UnsignedDecimal<N>>::mul(*self, rhs)
    }
}

macro_rules! macro_impl {
    (FROM $($ty: tt),*) => {
        $(
            impl<const N: usize> Mul<$ty> for UnsignedDecimal<N> {
                type Output = UnsignedDecimal<N>;

                #[inline]
                fn mul(self, rhs: $ty) -> Self::Output {
                    let rhs = UnsignedDecimal::from(rhs);
                    Mul::<UnsignedDecimal<N>>::mul(self, rhs)
                }
            }

            impl<const N: usize> MulAssign<$ty> for UnsignedDecimal<N> {
                #[inline]
                fn mul_assign(&mut self, rhs: $ty) {
                    let rhs = UnsignedDecimal::from(rhs);
                    self.mul_assign(rhs);
                }
            }
        )*
    };
    (TRY_FROM $($ty: tt),*) => {
        $(
            impl<const N: usize> Mul<$ty> for UnsignedDecimal<N> {
                type Output = UnsignedDecimal<N>;

                #[inline]
                fn mul(self, rhs: $ty) -> Self::Output {
                    let Ok(rhs) = UnsignedDecimal::try_from(rhs) else {
                        #[cfg(debug_assertions)]
                        panic!(crate::utils::err_msg!(concat!("attempt to multiply with invalid ", stringify!($ty))));

                        #[cfg(not(debug_assertions))]
                        return self;
                    };

                    Mul::<UnsignedDecimal<N>>::mul(self, rhs)
                }
            }

            impl<const N: usize> MulAssign<$ty> for UnsignedDecimal<N> {
                #[inline]
                fn mul_assign(&mut self, rhs: $ty) {
                    let Ok(rhs) = UnsignedDecimal::try_from(rhs) else {
                        #[cfg(debug_assertions)]
                        panic!(crate::utils::err_msg!(concat!("attempt to multiply with invalid ", stringify!($ty))));

                        #[cfg(not(debug_assertions))]
                        return;
                    };

                    self.mul_assign(rhs);
                }
            }
        )*
    };
}

macro_impl!(FROM u8, u16, u32, u64, u128, usize);
macro_impl!(TRY_FROM i8, i16, i32, i64, i128, isize, f32, f64);
