macro_rules! from_impl {
    ($Ty: ident, $sign: ident $($name:ident $num:ty),*) => {
        $(
            impl<const N: usize> From<$num> for $Ty<N> {
                #[inline]
                fn from(n: $num) -> Self {
                    Self::$name(n)
                }
            }
        )*
    };
}

pub(crate) use from_impl;

macro_rules! try_from_impl {
    ($Ty: ident, $sign: ident $($name:ident $num:ty),*) => {
        $(
            impl<const N: usize> TryFrom<$num> for $Ty<N> {
                type Error = ParseError;

                #[inline]
                fn try_from(n: $num) -> Result<Self, Self::Error> {
                    Self::$name(n)
                }
            }
        )*
    };
}

pub(crate) use try_from_impl;