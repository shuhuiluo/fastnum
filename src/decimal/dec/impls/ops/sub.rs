use core::ops::{Sub, SubAssign};

use crate::decimal::Decimal;

impl<const N: usize> Sub for Decimal<N> {
    type Output = Decimal<N>;

    #[inline]
    fn sub(self, rhs: Self) -> Decimal<N> {
        self.sub(rhs)
    }
}

impl<const N: usize> SubAssign for Decimal<N> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        let res = Sub::<Decimal<N>>::sub(*self, rhs);
        *self = res;
    }
}

macro_rules! macro_impl {
    (FROM $($ty: tt),*) => {
        $(
            impl<const N: usize> Sub<$ty> for Decimal<N> {
                type Output = Decimal<N>;

                #[inline]
                fn sub(self, rhs: $ty) -> Decimal<N> {
                    let rhs = Decimal::from(rhs);
                    Sub::<Decimal<N>>::sub(self, rhs)
                }
            }

            impl<const N: usize> Sub<Decimal<N>> for $ty {
                type Output = Decimal<N>;

                #[inline]
                fn sub(self, rhs: Decimal<N>) -> Decimal<N> {
                    let this = Decimal::from(self);
                    Sub::<Decimal<N>>::sub(this, rhs)
                }
            }

            impl<const N: usize> SubAssign<$ty> for Decimal<N> {
                #[inline]
                fn sub_assign(&mut self, rhs: $ty) {
                    let rhs = Decimal::from(rhs);
                    self.sub_assign(rhs);
                }
            }
        )*
    };
}

macro_impl!(FROM u8, u16, u32, u64, u128, usize);
macro_impl!(FROM i8, i16, i32, i64, i128, isize);
macro_impl!(FROM f32, f64);
