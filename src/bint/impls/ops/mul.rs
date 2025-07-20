macro_rules! mul_impl {
    ($Ty: ident, $sign: ident) => {
        impl<const N: usize> Mul for $Ty<N> {
            type Output = Self;

            #[inline]
            fn mul(self, rhs: Self) -> Self::Output {
                self.mul(rhs)
            }
        }

        impl<const N: usize> Mul<&Self> for $Ty<N> {
            type Output = Self;

            #[inline]
            fn mul(self, rhs: &Self) -> Self::Output {
                self.mul(*rhs)
            }
        }

        impl<const N: usize> MulAssign for $Ty<N> {
            #[inline]
            fn mul_assign(&mut self, rhs: Self) {
                self.0.mul_assign(rhs.0);
            }
        }
    };
}

pub(crate) use mul_impl;
