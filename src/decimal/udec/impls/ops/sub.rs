use core::ops::{Sub, SubAssign};

use crate::decimal::UnsignedDecimal;

impl<const N: usize> Sub for UnsignedDecimal<N> {
    type Output = UnsignedDecimal<N>;

    #[inline]
    fn sub(self, rhs: Self) -> UnsignedDecimal<N> {
        self.sub(rhs)
    }
}

impl<const N: usize> SubAssign for UnsignedDecimal<N> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        let res = Sub::<UnsignedDecimal<N>>::sub(*self, rhs);
        *self = res;
    }
}

macro_rules! macro_impl {
    (FROM $($ty: tt),*) => {
        $(
            impl<const N: usize> Sub<$ty> for UnsignedDecimal<N> {
                type Output = UnsignedDecimal<N>;

                #[inline]
                fn sub(self, rhs: $ty) -> UnsignedDecimal<N> {
                    let rhs = UnsignedDecimal::from(rhs);
                    Sub::<UnsignedDecimal<N>>::sub(self, rhs)
                }
            }

            impl<const N: usize> Sub<UnsignedDecimal<N>> for $ty {
                type Output = UnsignedDecimal<N>;

                #[inline]
                fn sub(self, rhs: UnsignedDecimal<N>) -> UnsignedDecimal<N> {
                    let this = UnsignedDecimal::from(self);
                    Sub::<UnsignedDecimal<N>>::sub(this, rhs)
                }
            }

            impl<const N: usize> SubAssign<$ty> for UnsignedDecimal<N> {
                #[inline]
                fn sub_assign(&mut self, rhs: $ty) {
                    let rhs = UnsignedDecimal::from(rhs);
                    self.sub_assign(rhs);
                }
            }
        )*
    };
    (TRY_FROM $($ty: tt),*) => {
        $(
            impl<const N: usize> Sub<$ty> for UnsignedDecimal<N> {
                type Output = UnsignedDecimal<N>;

                #[inline]
                fn sub(self, rhs: $ty) -> UnsignedDecimal<N> {
                    let Ok(rhs) = UnsignedDecimal::try_from(rhs) else {
                        #[cfg(debug_assertions)]
                        panic!(crate::utils::err_msg!(concat!("attempt to subtract with invalid ", stringify!($ty))));

                        #[cfg(not(debug_assertions))]
                        return self;
                    };

                    Sub::<UnsignedDecimal<N>>::sub(self, rhs)
                }
            }

            impl<const N: usize> Sub<UnsignedDecimal<N>> for $ty {
                type Output = UnsignedDecimal<N>;

                #[inline]
                fn sub(self, rhs: UnsignedDecimal<N>) -> UnsignedDecimal<N> {
                    let Ok(this) = UnsignedDecimal::try_from(self) else {
                        #[cfg(debug_assertions)]
                        panic!(crate::utils::err_msg!(concat!("attempt to subtract with invalid ", stringify!($ty))));

                        #[cfg(not(debug_assertions))]
                        return rhs;
                    };

                    Sub::<UnsignedDecimal<N>>::sub(this, rhs)
                }
            }

            impl<const N: usize> SubAssign<$ty> for UnsignedDecimal<N> {
                #[inline]
                fn sub_assign(&mut self, rhs: $ty) {
                    let Ok(rhs) = UnsignedDecimal::try_from(rhs) else {
                        #[cfg(debug_assertions)]
                        panic!(crate::utils::err_msg!(concat!("attempt to subtract with invalid ", stringify!($ty))));

                        #[cfg(not(debug_assertions))]
                        return;
                    };

                    self.sub_assign(rhs);
                }
            }
        )*
    };
}

macro_impl!(FROM u8, u16, u32, u64, u128, usize);
macro_impl!(TRY_FROM i8, i16, i32, i64, i128, isize, f32, f64);
