macro_rules! iter_impl {
    ($Ty: ident, $sign: ident) => {
        impl<const N: usize> Product<Self> for $Ty<N> {
            #[inline]
            fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(Self::ONE, |a, b| a * b)
            }
        }

        impl<'a, const N: usize> Product<&'a Self> for $Ty<N> {
            #[inline]
            fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
                iter.fold(Self::ONE, |a, b| a * b)
            }
        }

        impl<const N: usize> Sum<Self> for $Ty<N> {
            #[inline]
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(Self::ZERO, |a, b| a + b)
            }
        }

        impl<'a, const N: usize> Sum<&'a Self> for $Ty<N> {
            #[inline]
            fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
                iter.fold(Self::ZERO, |a, b| a + b)
            }
        }
    };
}

pub(crate) use iter_impl;
