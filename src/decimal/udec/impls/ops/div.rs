use core::ops::{Div, DivAssign};

use crate::decimal::UnsignedDecimal;

impl<const N: usize> Div for UnsignedDecimal<N> {
    type Output = UnsignedDecimal<N>;

    #[inline]
    fn div(self, rhs: Self) -> UnsignedDecimal<N> {
        self.div(rhs)
    }
}

impl<const N: usize> DivAssign for UnsignedDecimal<N> {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        let res = Div::<UnsignedDecimal<N>>::div(*self, rhs);
        *self = res;
    }
}

macro_rules! macro_impl {
    (FROM $($ty: tt),*) => {
        $(
            impl<const N: usize> Div<$ty> for UnsignedDecimal<N> {
                type Output = UnsignedDecimal<N>;

                #[inline]
                fn div(self, rhs: $ty) -> UnsignedDecimal<N> {
                    let rhs = UnsignedDecimal::from(rhs);
                    Div::<UnsignedDecimal<N>>::div(self, rhs)
                }
            }

            impl<const N: usize> DivAssign<$ty> for UnsignedDecimal<N> {
                #[inline]
                fn div_assign(&mut self, rhs: $ty) {
                    let rhs = UnsignedDecimal::from(rhs);
                    self.div_assign(rhs);
                }
            }
        )*
    };
    (TRY_FROM $($ty: tt),*) => {
        $(
            impl<const N: usize> Div<$ty> for UnsignedDecimal<N> {
                type Output = UnsignedDecimal<N>;

                #[inline]
                fn div(self, rhs: $ty) -> UnsignedDecimal<N> {
                    let Ok(rhs) = UnsignedDecimal::try_from(rhs) else {
                        #[cfg(debug_assertions)]
                        panic!(crate::utils::err_msg!(concat!("attempt to divide with invalid ", stringify!($ty))));

                        #[cfg(not(debug_assertions))]
                        return self;
                    };

                    Div::<UnsignedDecimal<N>>::div(self, rhs)
                }
            }

            impl<const N: usize> DivAssign<$ty> for UnsignedDecimal<N> {
                #[inline]
                fn div_assign(&mut self, rhs: $ty) {
                    let Ok(rhs) = UnsignedDecimal::try_from(rhs) else {
                        #[cfg(debug_assertions)]
                        panic!(crate::utils::err_msg!(concat!("attempt to divide with invalid ", stringify!($ty))));

                        #[cfg(not(debug_assertions))]
                        return;
                    };

                    self.div_assign(rhs);
                }
            }
        )*
    };
}

macro_impl!(FROM u8, u16, u32, u64, u128, usize);
macro_impl!(TRY_FROM i8, i16, i32, i64, i128, isize, f32, f64);
