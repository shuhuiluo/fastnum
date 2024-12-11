macro_rules! consts_impl {
    () => {
        impl<const N: usize> Decimal<N> {
            
            pub const NAN: Self = Self::new(UInt::ZERO, 0, Flags::NAN);
            
            /// Infinity (∞).
            pub const INFINITY: Self = Self::new(UInt::ZERO, 0, Flags::INFINITY);
            
            /// Negative infinity (−∞).
            pub const NEG_INFINITY: Self = Self::new(UInt::ZERO, 0, Flags::NEG_INFINITY);
            
            /// The smallest value that can be represented by this decimal type - 2<sup>N</sup> × 10<sup>32768</sup>.
            pub const MIN: Self = Self::new(UInt::MAX, i16::MIN, Flags::NEG);
            
            /// The maximum value that this type can represent 2<sup>N</sup> × 10<sup>32768</sup>.
            pub const MAX: Self = Self::new(UInt::MAX, i16::MIN, Flags::default());
    
            consts_impl!(CONSTS ZERO 0, ONE 1, TWO 2, THREE 3, FOUR 4, FIVE 5, SIX 6, SEVEN 7, EIGHT 8, NINE 9, TEN 10);
        }
    };
    (CONSTS $($name: ident $num: literal), *) => {
        $(
            #[doc = concat!("The value of `", $num, "` represented by this decimal type.")]
            pub const $name: Self = Self::new(UInt::$name, 0, Flags::default());
        )*
    }
}

pub(crate) use consts_impl;