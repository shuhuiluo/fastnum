macro_rules! widening_impl {
    ($Ty: ident, $sign: ident) => {
        #[doc = doc::widening::impl_desc!()]
        impl<const N: usize> $Ty<N> {}
    };
}

pub(crate) use widening_impl;
