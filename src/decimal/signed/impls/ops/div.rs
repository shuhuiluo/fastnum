use core::ops::{Div, DivAssign};

use crate::decimal::{signed::Decimal, RoundingMode};

impl<const N: usize> Div for Decimal<N> {
    type Output = Decimal<N>;

    #[inline]
    fn div(self, rhs: Self) -> Decimal<N> {
        self.div(rhs, RoundingMode::default()).unwrap()
    }
}

impl<const N: usize> DivAssign for Decimal<N> {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        let res = Div::<Decimal<N>>::div(*self, rhs);
        *self = res;
    }
}

macro_rules! macro_impl {
    (FROM $($ty: tt),*) => {
        $(
            impl<const N: usize> Div<$ty> for Decimal<N> {
                type Output = Decimal<N>;

                #[inline]
                fn div(self, rhs: $ty) -> Decimal<N> {
                    let rhs = Decimal::from(rhs);
                    Div::<Decimal<N>>::div(self, rhs)
                }
            }

            impl<const N: usize> DivAssign<$ty> for Decimal<N> {
                #[inline]
                fn div_assign(&mut self, rhs: $ty) {
                    let rhs = Decimal::from(rhs);
                    self.div_assign(rhs);
                }
            }
        )*
    };
    (TRY_FROM $($ty: tt),*) => {
        $(
            impl<const N: usize> Div<$ty> for Decimal<N> {
                type Output = Decimal<N>;

                #[inline]
                fn div(self, rhs: $ty) -> Decimal<N> {
                    let Ok(rhs) = Decimal::try_from(rhs) else {
                        #[cfg(debug_assertions)]
                        panic!(crate::utils::err_msg!(concat!("attempt to divide with invalid ", stringify!($ty))));

                        #[cfg(not(debug_assertions))]
                        return self;
                    };

                    Div::<Decimal<N>>::div(self, rhs)
                }
            }

            impl<const N: usize> DivAssign<$ty> for Decimal<N> {
                #[inline]
                fn div_assign(&mut self, rhs: $ty) {
                    let Ok(rhs) = Decimal::try_from(rhs) else {
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
macro_impl!(FROM i8, i16, i32, i64, i128, isize);
macro_impl!(TRY_FROM f32, f64);
