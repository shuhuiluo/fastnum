macro_rules! carrying_impl {
    ($Ty: ident, $sign: ident) => {
        impl<const N: usize> $Ty<N> {}
    };
}

pub(crate) use carrying_impl;