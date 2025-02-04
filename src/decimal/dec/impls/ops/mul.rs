use core::ops::{Mul, MulAssign};

use crate::decimal::Decimal;

impl<const N: usize> Mul for Decimal<N> {
    type Output = Decimal<N>;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.mul(rhs)
    }
}

impl<const N: usize> MulAssign for Decimal<N> {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = Mul::<Decimal<N>>::mul(*self, rhs)
    }
}

macro_rules! macro_impl {
    (FROM $($ty: tt),*) => {
        $(
            impl<const N: usize> Mul<$ty> for Decimal<N> {
                type Output = Decimal<N>;

                #[inline]
                fn mul(self, rhs: $ty) -> Self::Output {
                    let rhs = Decimal::from(rhs);
                    Mul::<Decimal<N>>::mul(self, rhs)
                }
            }

            impl<const N: usize> MulAssign<$ty> for Decimal<N> {
                #[inline]
                fn mul_assign(&mut self, rhs: $ty) {
                    let rhs = Decimal::from(rhs);
                    self.mul_assign(rhs);
                }
            }
        )*
    };
}

macro_impl!(FROM u8, u16, u32, u64, u128, usize);
macro_impl!(FROM i8, i16, i32, i64, i128, isize);
macro_impl!(FROM f32, f64);
