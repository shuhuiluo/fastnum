use core::ops::{Add, AddAssign};

use crate::decimal::Decimal;

impl<const N: usize> Add for Decimal<N> {
    type Output = Decimal<N>;

    #[inline]
    fn add(self, rhs: Self) -> Decimal<N> {
        self.add(rhs)
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
}

macro_impl!(FROM u8, u16, u32, u64, u128, usize);
macro_impl!(FROM i8, i16, i32, i64, i128, isize);
macro_impl!(FROM f32, f64);
