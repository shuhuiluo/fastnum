macro_rules! rem_impl {
    ($Ty: ident, $sign: ident) => {
        impl<const N: usize> Rem for $Ty<N> {
            type Output = Self;

            #[inline]
            fn rem(self, rhs: Self) -> Self::Output {
                self.rem(rhs)
            }
        }

        impl<const N: usize> RemAssign for $Ty<N> {
            #[inline]
            fn rem_assign(&mut self, rhs: Self) {
                self.0.rem_assign(rhs.0)
            }
        }
    };
}

pub(crate) use rem_impl;
