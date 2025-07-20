macro_rules! ops_impl {
    ($Ty: ident, U, $Op: ident, $op: ident, $OpAssign: ident, $op_assign: ident) => {
        ops_impl!(@ $Ty, U, $Op, $op, $OpAssign, $op_assign, [u8, u16, u32, u64, usize, i8 #TRY, i16 #TRY, i32 #TRY, i64 #TRY, i128 #TRY, u128 #TRY, isize #TRY, f32 #TRY, f64 #TRY]);
    };
    ($Ty: ident, I, $Op: ident, $op: ident, $OpAssign: ident, $op_assign: ident) => {
        ops_impl!(@ $Ty, I, $Op, $op, $OpAssign, $op_assign, [u8, u16, u32, u64, usize, i8, i16, i32, i64, i128 #TRY, u128 #TRY, isize, f32, f64]);
    };
    (@ $Ty: ident, $sign: ident, $Op: ident, $op: ident, $OpAssign: ident, $op_assign: ident, [$($ty: ident $(#$try: ident)?),*]) => {
        impl<const N: usize> $Op for $Ty<N> {
            type Output = Self;

            #[inline]
            fn $op(self, rhs: Self) -> Self::Output {
                self.$op(rhs)
            }
        }

        impl<const N: usize> $OpAssign for $Ty<N> {
            #[inline]
            fn $op_assign(&mut self, rhs: Self) {
                let res = $Op::<$Ty<N>>::$op(*self, rhs);
                *self = res;
            }
        }

        $(
            ops_impl!(@@ $($try)? $Ty, $ty, $Op, $op, $OpAssign, $op_assign);
        )*
    };
    (@@ $Ty: ident, $ty: ident, $Op: ident, $op: ident, $OpAssign: ident, $op_assign: ident) => {
        impl<const N: usize> $Op<$ty> for $Ty<N> {
            type Output = Self;

            #[inline]
            fn $op(self, rhs: $ty) -> Self::Output {
                let rhs = $Ty::from(rhs);
                $Op::<$Ty<N>>::$op(self, rhs)
            }
        }

        impl<const N: usize> $Op<$Ty<N>> for $ty {
            type Output = $Ty<N>;

            #[inline]
            fn $op(self, rhs: $Ty<N>) -> Self::Output {
                let this = $Ty::from(self);
                $Op::<$Ty<N>>::$op(this, rhs)
            }
        }

        impl<const N: usize> $OpAssign<$ty> for $Ty<N> {
            #[inline]
            fn $op_assign(&mut self, rhs: $ty) {
                let rhs = $Ty::from(rhs);
                self.$op_assign(rhs);
            }
        }
    };
    (@@ TRY $Ty: ident, $ty: ident, $Op: ident, $op: ident, $OpAssign: ident, $op_assign: ident) => {
        impl<const N: usize> $Op<$ty> for $Ty<N> {
            type Output = Self;

            #[inline]
            fn $op(self, rhs: $ty) -> $Ty<N> {
                let Ok(rhs) = $Ty::try_from(rhs) else {
                    #[cfg(debug_assertions)]
                    panic!(crate::utils::err_msg!(concat!("attempt to ", stringify!($op), " with invalid ", stringify!($ty))));

                    #[cfg(not(debug_assertions))]
                    return self;
                };

                $Op::<$Ty<N>>::$op(self, rhs)
            }
        }

        impl<const N: usize> $Op<$Ty<N>> for $ty {
            type Output = $Ty<N>;

            #[inline]
            fn $op(self, rhs: $Ty<N>) -> Self::Output {
                let Ok(this) = $Ty::try_from(self) else {
                    #[cfg(debug_assertions)]
                    panic!(crate::utils::err_msg!(concat!("attempt to ", stringify!($op), " with invalid ", stringify!($ty))));

                    #[cfg(not(debug_assertions))]
                    return rhs;
                };

                $Op::<$Ty<N>>::$op(this, rhs)
            }
        }

        impl<const N: usize> $OpAssign<$ty> for $Ty<N> {
            #[inline]
            fn $op_assign(&mut self, rhs: $ty) {
                let Ok(rhs) = $Ty::try_from(rhs) else {
                    #[cfg(debug_assertions)]
                    panic!(crate::utils::err_msg!(concat!("attempt to ", stringify!($op), " with invalid ", stringify!($ty))));

                    #[cfg(not(debug_assertions))]
                    return;
                };

                self.$op_assign(rhs);
            }
        }
    };
}

pub(crate) use ops_impl;
