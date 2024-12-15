macro_rules! consts_impl {
    () => {
        impl<const N: usize> UnsignedDecimal<N> {
            
            /// **N**ot **A** **N**umber value. More about [`NaN`](crate#special-values).
            pub const NAN: Self = Self::new(Decimal::NAN);
            
            /// Infinity (∞). More about [`±Infinity`](crate#special-values).
            pub const INFINITY: Self = Self::new(Decimal::INFINITY);
            
            /// The smallest value that can be represented by this decimal type (0).
            pub const MIN: Self = Self::ZERO;
            
            /// The largest value that can be represented by this decimal type (2<sup>N</sup> − 1)×10<sup>32'768</sup>.
            pub const MAX: Self = Self::new(Decimal::MAX);
            
            consts_impl!(CONSTS ZERO 0, ONE 1, TWO 2, THREE 3, FOUR 4, FIVE 5, SIX 6, SEVEN 7, EIGHT 8, NINE 9, TEN 10);
        }
    };
    (CONSTS $($name: ident $num: literal), *) => {
        $(
            #[doc = concat!("The value of `", $num, "` represented by this decimal type.")]
            pub const $name: Self = Self::new(Decimal::$name);
        )*
    }
}

pub(crate) use consts_impl;