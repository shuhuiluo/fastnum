use core::ops::{Rem, RemAssign};

use crate::decimal::Decimal;

impl<const N: usize> Rem for Decimal<N> {
    type Output = Decimal<N>;

    #[inline]
    fn rem(self, rhs: Self) -> Decimal<N> {
        self.rem(rhs)
    }
}

impl<const N: usize> RemAssign for Decimal<N> {
    #[inline]
    fn rem_assign(&mut self, rhs: Self) {
        let res = Rem::<Decimal<N>>::rem(*self, rhs);
        *self = res;
    }
}

macro_rules! macro_impl {
    (FROM $($ty: tt),*) => {
        $(
            impl<const N: usize> Rem<$ty> for Decimal<N> {
                type Output = Decimal<N>;

                #[inline]
                fn rem(self, rhs: $ty) -> Decimal<N> {
                    let rhs = Decimal::from(rhs);
                    Rem::<Decimal<N>>::rem(self, rhs)
                }
            }

            impl<const N: usize> RemAssign<$ty> for Decimal<N> {
                #[inline]
                fn rem_assign(&mut self, rhs: $ty) {
                    let rhs = Decimal::from(rhs);
                    self.rem_assign(rhs);
                }
            }
        )*
    };
    (TRY_FROM $($ty: tt),*) => {
        $(
            impl<const N: usize> Rem<$ty> for Decimal<N> {
                type Output = Decimal<N>;

                #[inline]
                fn rem(self, rhs: $ty) -> Decimal<N> {
                    let Ok(rhs) = Decimal::try_from(rhs) else {
                        #[cfg(debug_assertions)]
                        panic!(crate::utils::err_msg!(concat!("attempt to rem with invalid ", stringify!($ty))));

                        #[cfg(not(debug_assertions))]
                        return self;
                    };

                    Rem::<Decimal<N>>::rem(self, rhs)
                }
            }

            impl<const N: usize> RemAssign<$ty> for Decimal<N> {
                #[inline]
                fn rem_assign(&mut self, rhs: $ty) {
                    let Ok(rhs) = Decimal::try_from(rhs) else {
                        #[cfg(debug_assertions)]
                        panic!(crate::utils::err_msg!(concat!("attempt to rem with invalid ", stringify!($ty))));

                        #[cfg(not(debug_assertions))]
                        return;
                    };

                    self.rem_assign(rhs);
                }
            }
        )*
    };
}

macro_impl!(FROM u8, u16, u32, u64, u128, usize);
macro_impl!(FROM i8, i16, i32, i64, i128, isize);
macro_impl!(TRY_FROM f32, f64);
