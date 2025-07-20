macro_rules! consts_impl {
    ($Ty: ident, $sign: ident, $Int: ident) => {
        #[doc = doc::consts::impl_desc!($sign)]
        impl<const N: usize> $Ty<N> {

            #[doc = doc::consts::min!($sign 512)]
            pub const MIN: Self = Self($Int::MIN);

            #[doc = doc::consts::max!($sign 512)]
            pub const MAX: Self = Self($Int::MAX);

            #[doc = doc::consts::bits!($sign 512, 512)]
            pub const BITS: intrinsics::ExpType = $Int::<N>::BITS;

            #[doc = doc::consts::bytes!($sign 512, 512)]
            pub const BYTES: intrinsics::ExpType = $Int::<N>::BYTES;

            consts_impl!(@ CONSTS $sign $Int);
        }
    };
    (@ CONSTS U $Int: ident) => {
        consts_impl!(@ CONSTS U $Int [ZERO 0, ONE 1, TWO 2, THREE 3, FOUR 4, FIVE 5, SIX 6, SEVEN 7, EIGHT 8, NINE 9, TEN 10]);

        pub(crate) const MAX_POWER_OF_TWO: intrinsics::ExpType = Self::BITS - 1;
        pub(crate) const MAX_POWER_OF_FIVE: intrinsics::ExpType = intrinsics::Intrinsics::<N>::MAX_POWER_OF_FIVE;
    };
    (@ CONSTS I $Int: ident) => {
        consts_impl!(@ CONSTS U $Int [ZERO 0, ONE 1, TWO 2, THREE 3, FOUR 4, FIVE 5, SIX 6, SEVEN 7, EIGHT 8, NINE 9, TEN 10]);
        consts_impl!(@ CONSTS I $Int [NEG_ONE -1, NEG_TWO -2, NEG_THREE -3, NEG_FOUR -4, NEG_FIVE -5, NEG_SIX -6, NEG_SEVEN -7, NEG_EIGHT -8, NEG_NINE -9, NEG_TEN -10]);
    };
    (@ CONSTS $sign: ident $Int: ident [$($name: ident $num: literal), *]) => {
        $(
            #[doc = doc::consts::value_desc!($sign, $num)]
            pub const $name: Self = Self($Int::$name);
        )*
    };
}

pub(crate) use consts_impl;
