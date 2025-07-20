macro_rules! fmt_impl {
    ($Ty: ident, $sign: ident) => {
        impl<const N: usize> Display for $Ty<N> {
            #[inline]
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                Display::fmt(&self.0, f)
            }
        }

        impl<const N: usize> Debug for $Ty<N> {
            #[inline]
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                Debug::fmt(&self.0, f)
            }
        }
    };
}

pub(crate) use fmt_impl;
