macro_rules! try_to_impl {
    ($Ty: ident, $sign: ident $($name:ident $int:ty),*) => {
        $(
            impl<const N: usize> TryFrom<$Ty<N>> for $int {
                type Error = ParseError;

                #[inline]
                fn try_from(n: $Ty<N>) -> Result<$int, Self::Error> {
                    n.$name()
                }
            }
        )*
    };
}

pub(crate) use try_to_impl;

macro_rules! try_to_float_impl {
    ($Ty: ident, $sign: ident $($f:ident),*) => {
        $(
            impl<const N: usize> TryFrom<$Ty<N>> for $f {
                type Error = ParseError;

                #[inline]
                fn try_from(n: $Ty<N>) -> Result<$f, Self::Error> {
                    // TODO
                    Ok(bnum::cast::CastFrom::cast_from(n.0))
                }
            }
        )*
    };
}

pub(crate) use try_to_float_impl;