macro_rules! from_primitive_impl {
    ($Ty: ident, $sign: ident, $($from_int:ident <- $int:ident $(#$try: ident)?),*) => {
        impl<const N: usize> FromPrimitive for $Ty<N> {
            $(
                #[inline]
                fn $from_int(n: $int) -> Option<Self> {
                    from_primitive_impl!(@ $($try)? $Ty, n $from_int)
                }
            )*
        }
    };
    (@ $Ty: ident, $n: ident $from_int: ident) => {
        Some($Ty::$from_int($n))
    };
    (@ TRY $Ty: ident, $n: ident $from_int: ident) => {
        $Ty::$from_int($n).ok()
    };
}

pub(crate) use from_primitive_impl;
