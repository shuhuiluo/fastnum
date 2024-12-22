use core::ops::{Add, AddAssign};

use crate::decimal::UnsignedDecimal;

impl<const N: usize> Add for UnsignedDecimal<N> {
    type Output = UnsignedDecimal<N>;

    #[inline]
    fn add(self, rhs: Self) -> UnsignedDecimal<N> {
        self.add(rhs)
    }
}

impl<const N: usize> AddAssign for UnsignedDecimal<N> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        let res = Add::<UnsignedDecimal<N>>::add(*self, rhs);
        *self = res;
    }
}

macro_rules! macro_impl {
    (FROM $($ty: tt),*) => {
        $(
            impl<const N: usize> Add<$ty> for UnsignedDecimal<N> {
                type Output = UnsignedDecimal<N>;

                #[inline]
                fn add(self, rhs: $ty) -> UnsignedDecimal<N> {
                    let rhs = UnsignedDecimal::from(rhs);
                    Add::<UnsignedDecimal<N>>::add(self, rhs)
                }
            }

            impl<const N: usize> AddAssign<$ty> for UnsignedDecimal<N> {
                #[inline]
                fn add_assign(&mut self, rhs: $ty) {
                    let rhs = UnsignedDecimal::from(rhs);
                    self.add_assign(rhs);
                }
            }
        )*
    };
    (TRY_FROM $($ty: tt),*) => {
        $(
            impl<const N: usize> Add<$ty> for UnsignedDecimal<N> {
                type Output = UnsignedDecimal<N>;

                #[inline]
                fn add(self, rhs: $ty) -> UnsignedDecimal<N> {
                    let Ok(rhs) = UnsignedDecimal::try_from(rhs) else {
                        #[cfg(debug_assertions)]
                        panic!(crate::utils::err_msg!(concat!("attempt to add with invalid ", stringify!($ty))));

                        #[cfg(not(debug_assertions))]
                        return self;
                    };

                    Add::<UnsignedDecimal<N>>::add(self, rhs)
                }
            }

            impl<const N: usize> AddAssign<$ty> for UnsignedDecimal<N> {
                #[inline]
                fn add_assign(&mut self, rhs: $ty) {
                    let Ok(rhs) = UnsignedDecimal::try_from(rhs) else {
                        #[cfg(debug_assertions)]
                        panic!(crate::utils::err_msg!(concat!("attempt to add with invalid ", stringify!($ty))));

                        #[cfg(not(debug_assertions))]
                        return;
                    };

                    self.add_assign(rhs);
                }
            }
        )*
    };
}

macro_impl!(FROM u8, u16, u32, u64, u128, usize);
macro_impl!(TRY_FROM i8, i16, i32, i64, i128, isize, f32, f64);
