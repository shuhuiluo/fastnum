macro_rules! bits_impl {
    ($Ty: ident, $sign: ident) => {
        impl<const N: usize> Not for $Ty<N> {
            type Output = Self;

            #[inline]
            fn not(self) -> Self {
                self.not()
            }
        }

        impl<const N: usize> BitAnd for $Ty<N> {
            type Output = Self;

            #[inline]
            fn bitand(self, rhs: Self) -> Self {
                Self::bitand(self, rhs)
            }
        }

        impl<const N: usize> BitOr for $Ty<N> {
            type Output = Self;

            #[inline]
            fn bitor(self, rhs: Self) -> Self {
                Self::bitor(self, rhs)
            }
        }

        impl<const N: usize> BitXor for $Ty<N> {
            type Output = Self;

            #[inline]
            fn bitxor(self, rhs: Self) -> Self {
                Self::bitxor(self, rhs)
            }
        }
    };
}

pub(crate) use bits_impl;
