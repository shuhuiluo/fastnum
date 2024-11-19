macro_rules! consts_impl {
    () => {
        impl<const N: usize> Decimal<N> {
            /// The smallest value that can be represented by this decimal type -(2<sup>N</sup> − 1)×10<sup>63</sup>.
            pub const MIN: Self = Self::new(UnsignedDecimal::<N>::MAX, Sign::Minus);
    
            /// The maximum value that this type can represent (2<sup>N</sup> − 1)×10<sup>63</sup>.
            pub const MAX: Self = Self::new(UnsignedDecimal::<N>::MAX, Sign::NoSign);
    
            consts_impl!(CONSTS ZERO 0, ONE 1, TWO 2, THREE 3, FOUR 4, FIVE 5, SIX 6, SEVEN 7, EIGHT 8, NINE 9, TEN 10);
    
            /// Return if the referenced unsigned decimal is zero.
            ///
            /// # Examples
            ///
            /// ```
            /// use fastnum::{dec256};
            ///
            /// let a = dec256!(0);
            /// assert!(a.is_zero());
            ///
            /// let b = dec256!(+0.0);
            /// assert!(b.is_zero());
            ///
            /// let c = dec256!(-0.00);
            /// assert!(c.is_zero());
            ///
            /// let d = dec256!(-0.1);
            /// assert!(!d.is_zero());
            /// ```
            #[inline]
            pub const fn is_zero(&self) -> bool {
                self.value.is_zero()
            }
        }
    };
    (CONSTS $($name: ident $num: literal), *) => {
        $(
            #[doc = concat!("The value of `", $num, "` represented by this decimal type.")]
            pub const $name: Self = Self::new(UnsignedDecimal::<N>::$name, Sign::NoSign);
        )*
    }
}

pub(crate) use consts_impl;