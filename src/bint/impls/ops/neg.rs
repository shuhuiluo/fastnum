macro_rules! neg_impl {
    ($Ty: ident, $sign: ident) => {
        impl<const N: usize> Neg for $Ty<N> {
            type Output = Self;

            #[inline]
            fn neg(self) -> Self::Output {
                self.neg()
            }
        }

        impl<const N: usize> Neg for &$Ty<N> {
            type Output = $Ty<N>;

            #[inline]
            fn neg(self) -> Self::Output {
                (*self).neg()
            }
        }
    };
}

pub(crate) use neg_impl;
