macro_rules! shift_impl {
    ($Ty: ident, $sign: ident, $Op: ident, $op: ident, $OpAssign: ident, $op_assign: ident) => {
        impl<const N: usize> $Op<ExpType> for $Ty<N> {
            type Output = Self;

            #[inline]
            fn $op(self, rhs: ExpType) -> Self::Output {
                Self::$op(self, rhs)
            }
        }
        
        impl<const N: usize> $OpAssign<ExpType> for $Ty<N> {
            #[inline]
            fn $op_assign(&mut self, rhs: ExpType) {
                self.0.$op_assign(rhs);
            }
        }
        
        shift_impl!(@ $Ty, $sign, $Op, $op, $OpAssign, $op_assign, [u8, u16, u64 #TRY, usize #TRY, u128 #TRY, i8 #TRY, i16 #TRY, i32 #TRY, i64 #TRY, isize #TRY, i128 #TRY]);
    };
    (@ $Ty: ident, $sign: ident, $Op: ident, $op: ident, $OpAssign: ident, $op_assign: ident, [$($ty: ident $(#$try: ident)?),*]) => {
        $(
            shift_impl!(@@ $($try)? $Ty, $sign, $Op, $op, $OpAssign, $op_assign, $ty);
        )*
    };
    (@@ $Ty: ident, $sign: ident, $Op: ident, $op: ident, $OpAssign: ident, $op_assign: ident, $ty: ident) => {
        impl<const N: usize> $Op<$ty> for $Ty<N> {
            type Output = Self;

            #[inline]
            fn $op(self, rhs: $ty) -> Self::Output {
                Self::$op(self, rhs as ExpType)
            }
        }
        
        impl<const N: usize> $OpAssign<$ty> for $Ty<N> {
            #[inline]
            fn $op_assign(&mut self, rhs: $ty) {
                self.0.$op_assign(rhs);
            }
        }
    };
    (@@ TRY $Ty: ident, $sign: ident, $Op: ident, $op: ident, $OpAssign: ident, $op_assign: ident, $ty: ident) => {
        impl<const N: usize> $Op<$ty> for $Ty<N> {
            type Output = Self;

            #[inline]
            fn $op(self, rhs: $ty) -> Self::Output {
                #[cfg(debug_assertions)]
                let rhs: ExpType = crate::utils::result_expect!(ExpType::try_from(rhs), crate::utils::err_msg!("attempt to shift with overflow"));

                #[cfg(not(debug_assertions))]
                let rhs = rhs as ExpType;
                
                Self::$op(self, rhs as ExpType)
            }
        }
        
        impl<const N: usize> $OpAssign<$ty> for $Ty<N> {
            #[inline]
            fn $op_assign(&mut self, rhs: $ty) {
                self.0.$op_assign(rhs);
            }
        }
    };
}

pub(crate) use shift_impl;