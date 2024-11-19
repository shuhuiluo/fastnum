use core::ops::{Mul, MulAssign};

use crate::decimal::{signed::Decimal, RoundingMode};

macro_rules! macro_impl {
    () => {
        macro_impl!(IMPL 128);
        macro_impl!(IMPL 256);
        macro_impl!(IMPL 512);
        macro_impl!(IMPL 1024);
        macro_impl!(IMPL 2048);
        macro_impl!(IMPL 4096);
    };
    (IMPL $bits: literal) => {
        macro_impl!(IMPL MUL $bits);
        macro_impl!(IMPL MUL_ASSIGN $bits);
    };
    (IMPL MUL $bits: literal) => {
        impl Mul for Decimal<{$bits / 64}> {
            type Output = Decimal<{$bits / 64}>;

            #[inline]
            fn mul(self, rhs: Self) -> Self::Output {
                self.mul(rhs, RoundingMode::default()).unwrap()
            }
        }

        macro_impl!(IMPL MUL FROM : $bits : u8, u16, u32, u64, u128, usize);
        macro_impl!(IMPL MUL FROM : $bits : i8, i16, i32, i64, i128, isize);
        macro_impl!(IMPL MUL TRY_FROM : $bits : f32, f64);
    };
    (IMPL MUL_ASSIGN $bits: literal) => {
        impl MulAssign for Decimal<{$bits / 64}> {
            #[inline]
            fn mul_assign(&mut self, rhs: Self) {
                *self = Mul::<Decimal<{$bits / 64}>>::mul(*self, rhs)
            }
        }
    };
    (IMPL MUL FROM : $bits: literal : $($ty: tt),*) => {
        $(
            impl Mul<$ty> for Decimal<{$bits / 64}> {
                type Output = Decimal<{$bits / 64}>;

                #[inline]
                fn mul(self, rhs: $ty) -> Self::Output {
                    let rhs = Decimal::from(rhs);
                    Mul::<Decimal<{$bits / 64}>>::mul(self, rhs)
                }
            }

            impl MulAssign<$ty> for Decimal<{$bits / 64}> {
                #[inline]
                fn mul_assign(&mut self, rhs: $ty) {
                    let rhs = Decimal::from(rhs);
                    self.mul_assign(rhs);
                }
            }
        )*
    };
    (IMPL MUL TRY_FROM : $bits: literal : $($ty: tt),*) => {
        $(
            impl Mul<$ty> for Decimal<{$bits / 64}> {
                type Output = Decimal<{$bits / 64}>;

                #[inline]
                fn mul(self, rhs: $ty) -> Self::Output {
                    let Ok(rhs) = Decimal::try_from(rhs) else {
                        #[cfg(debug_assertions)]
                        panic!(crate::utils::err_msg!(concat!("attempt to multiply with invalid ", stringify!($ty))));

                        #[cfg(not(debug_assertions))]
                        return self;
                    };

                    Mul::<Decimal<{$bits / 64}>>::mul(self, rhs)
                }
            }

            impl MulAssign<$ty> for Decimal<{$bits / 64}> {
                #[inline]
                fn mul_assign(&mut self, rhs: $ty) {
                    let Ok(rhs) = Decimal::try_from(rhs) else {
                        #[cfg(debug_assertions)]
                        panic!(crate::utils::err_msg!(concat!("attempt to multiply with invalid ", stringify!($ty))));

                        #[cfg(not(debug_assertions))]
                        return;
                    };

                    self.mul_assign(rhs);
                }
            }
        )*
    };
}

macro_impl!();
