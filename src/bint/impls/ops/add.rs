macro_rules! add_impl {
    ($Ty: ident, $sign: ident) => {
        impl<const N: usize> Add for $Ty<N> {
            type Output = Self;

            #[inline]
            fn add(self, rhs: Self) -> Self::Output {
                Self::add(self, rhs)
            }
        }

        impl<const N: usize> Add<&Self> for $Ty<N> {
            type Output = Self;

            #[inline]
            fn add(self, rhs: &Self) -> Self::Output {
                Self::add(self, *rhs)
            }
        }

        impl<const N: usize> AddAssign for $Ty<N> {
            #[inline]
            fn add_assign(&mut self, rhs: Self) {
                self.0.add_assign(rhs.0)
            }
        }
    };
}

pub(crate) use add_impl;
