use core::ops::{Add, AddAssign};

use crate::decimal::{signed::Decimal, RoundingMode};

impl<const N: usize> Add for Decimal<N> {
    type Output = Decimal<N>;

    #[inline]
    fn add(self, rhs: Self) -> Decimal<N> {
        self.add(rhs, RoundingMode::default()).unwrap()
    }
}

impl<const N: usize> AddAssign for Decimal<N> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        let res = Add::<Decimal<N>>::add(*self, rhs);
        *self = res;
    }
}

macro_rules! macro_impl {
    (FROM $($ty: tt),*) => {
        $(
            impl<const N: usize> Add<$ty> for Decimal<N> {
                type Output = Decimal<N>;

                #[inline]
                fn add(self, rhs: $ty) -> Decimal<N> {
                    let rhs = Decimal::from(rhs);
                    Add::<Decimal<N>>::add(self, rhs)
                }
            }

            impl<const N: usize> AddAssign<$ty> for Decimal<N> {
                #[inline]
                fn add_assign(&mut self, rhs: $ty) {
                    let rhs = Decimal::from(rhs);
                    self.add_assign(rhs);
                }
            }
        )*
    };
    (TRY_FROM $($ty: tt),*) => {
        $(
            impl<const N: usize> Add<$ty> for Decimal<N> {
                type Output = Decimal<N>;

                #[inline]
                fn add(self, rhs: $ty) -> Decimal<N> {
                    let Ok(rhs) = Decimal::try_from(rhs) else {
                        #[cfg(debug_assertions)]
                        panic!(crate::utils::err_msg!(concat!("attempt to add with invalid ", stringify!($ty))));

                        #[cfg(not(debug_assertions))]
                        return self;
                    };

                    Add::<Decimal<N>>::add(self, rhs)
                }
            }

            impl<const N: usize> AddAssign<$ty> for Decimal<N> {
                #[inline]
                fn add_assign(&mut self, rhs: $ty) {
                    let Ok(rhs) = Decimal::try_from(rhs) else {
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
macro_impl!(FROM i8, i16, i32, i64, i128, isize);
macro_impl!(TRY_FROM f32, f64);
