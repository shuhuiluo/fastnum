macro_rules! sub_impl {
    ($Ty: ident, $sign: ident) => {
        impl<const N: usize> Sub for $Ty<N> {
            type Output = Self;
        
            #[inline]
            fn sub(self, rhs: Self) -> Self::Output {
                self.sub(rhs)
            }
        }
        
        impl<const N: usize> SubAssign for $Ty<N> {
            #[inline]
            fn sub_assign(&mut self, rhs: Self) {
                self.0.sub_assign(rhs.0)
            }
        }
    };
}

pub(crate) use sub_impl;