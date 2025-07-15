macro_rules! div_impl {
    ($Ty: ident, $sign: ident) => {
        impl<const N: usize> Div<Self> for $Ty<N> {
            type Output = Self;

            #[inline]
            fn div(self, rhs: Self) -> Self::Output {
                Self::div(self, rhs)
            }
        }

        impl<const N: usize> DivAssign for $Ty<N> {
            #[inline]
            fn div_assign(&mut self, rhs: Self) {
                self.0.div_assign(rhs.0)
            }
        }
    };
}

pub(crate) use div_impl;
