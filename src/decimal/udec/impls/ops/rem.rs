use core::ops::{Rem, RemAssign};

use crate::decimal::{UnsignedDecimal, Context};

impl<const N: usize> Rem for UnsignedDecimal<N> {
    type Output = UnsignedDecimal<N>;

    #[inline]
    fn rem(self, rhs: Self) -> UnsignedDecimal<N> {
        self.rem(rhs, Context::default())
    }
}

impl<const N: usize> RemAssign for UnsignedDecimal<N> {
    #[inline]
    fn rem_assign(&mut self, rhs: Self) {
        let res = Rem::<UnsignedDecimal<N>>::rem(*self, rhs);
        *self = res;
    }
}

macro_rules! macro_impl {
    (FROM $($ty: tt),*) => {
        $(
            impl<const N: usize> Rem<$ty> for UnsignedDecimal<N> {
                type Output = UnsignedDecimal<N>;

                #[inline]
                fn rem(self, rhs: $ty) -> UnsignedDecimal<N> {
                    let rhs = UnsignedDecimal::from(rhs);
                    Rem::<UnsignedDecimal<N>>::rem(self, rhs)
                }
            }

            impl<const N: usize> RemAssign<$ty> for UnsignedDecimal<N> {
                #[inline]
                fn rem_assign(&mut self, rhs: $ty) {
                    let rhs = UnsignedDecimal::from(rhs);
                    self.rem_assign(rhs);
                }
            }
        )*
    };
    (TRY_FROM $($ty: tt),*) => {
        $(
            impl<const N: usize> Rem<$ty> for UnsignedDecimal<N> {
                type Output = UnsignedDecimal<N>;

                #[inline]
                fn rem(self, rhs: $ty) -> UnsignedDecimal<N> {
                    let Ok(rhs) = UnsignedDecimal::try_from(rhs) else {
                        #[cfg(debug_assertions)]
                        panic!(crate::utils::err_msg!(concat!("attempt to rem with invalid ", stringify!($ty))));

                        #[cfg(not(debug_assertions))]
                        return self;
                    };

                    Rem::<UnsignedDecimal<N>>::rem(self, rhs)
                }
            }

            impl<const N: usize> RemAssign<$ty> for UnsignedDecimal<N> {
                #[inline]
                fn rem_assign(&mut self, rhs: $ty) {
                    let Ok(rhs) = UnsignedDecimal::try_from(rhs) else {
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
macro_impl!(TRY_FROM i8, i16, i32, i64, i128, isize, f32, f64);
