macro_rules! consts_impl {
    () => {
        impl<const N: usize> UnsignedDecimal<N> {
            /// The smallest value that can be represented by this decimal type (0).
            pub const MIN: Self = Self::new(UInt::MIN, 0);
            
            /// The largest value that can be represented by this decimal type (2<sup>N</sup> − 1)×10<sup>63</sup>.
            pub const MAX: Self = Self::new(UInt::MAX, i64::MIN);
            
            consts_impl!(CONSTS ZERO 0, ONE 1, TWO 2, THREE 3, FOUR 4, FIVE 5, SIX 6, SEVEN 7, EIGHT 8, NINE 9, TEN 10);
            
            /// Return if the referenced unsigned decimal is zero.
            ///
            /// # Examples
            ///
            /// ```
            /// use fastnum::{udec256};
            ///
            /// let a = udec256!(0);
            /// assert!(a.is_zero());
            ///
            /// let b = udec256!(0.0);
            /// assert!(b.is_zero());
            ///
            /// let c = udec256!(0.00);
            /// assert!(c.is_zero());
            ///
            /// let d = udec256!(0.1);
            /// assert!(!d.is_zero());
            /// ```
            #[inline]
            pub const fn is_zero(&self) -> bool {
                self.value.is_zero()
            }
            
            /// Return if the referenced unsigned decimal is strictly [Self::ONE].
            ///
            /// # Examples
            ///
            /// ```
            /// use fastnum::{udec256};
            ///
            /// let a = udec256!(1);
            /// assert!(a.is_one());
            ///
            /// let b = udec256!(10e-1);
            /// assert!(!b.is_one());
            ///
            /// ```
            #[inline]
            pub const fn is_one(&self) -> bool {
                self.value.is_one() && self.scale == 0
            }
        }
    };
    (CONSTS $($name: ident $num: literal), *) => {
        $(
            #[doc = concat!("The value of `", $num, "` represented by this decimal type.")]
            pub const $name: Self = Self::new(UInt::$name, 0);
        )*
    }
}

pub(crate) use consts_impl;