macro_rules! ops_impl {
    () => {
        ops_impl!(IMPL : MUL : 128);
        ops_impl!(IMPL : MUL : 256);
        ops_impl!(IMPL : MUL : 512);
        ops_impl!(IMPL : MUL : 1024);
        ops_impl!(IMPL : MUL : 2048);
        ops_impl!(IMPL : MUL : 4096);
    };
    (IMPL : MUL : $bits: literal) => {
        impl UnsignedDecimal<{$bits / 64}> {
            /// Calculates `self` × `rhs`.
            ///
            /// Returns [DecimalResult] with result of multiplication and [emergency
            /// flags](crate#arithmetic-result). Is internally used by the `*`
            /// operator.
            ///
            /// # Examples
            ///
            /// Basic usage:
            ///
            /// ```
            /// use fastnum::{udec256, UD256};
            /// use fastnum::decimal::RoundingMode;
            ///
            /// let a = UD256::FIVE;
            /// let b = UD256::TWO;
            ///
            /// let c = a.mul(b, RoundingMode::default()).unwrap();
            /// assert_eq!(c, udec256!(10));
            /// ```
            ///
            /// ```should_panic
            /// use fastnum::{udec256, UD256};
            /// use fastnum::decimal::RoundingMode;
            ///
            /// let a = UD256::MAX;
            /// let b = UD256::MAX;
            ///
            /// let c = a * b;
            /// ```
            ///
            /// For more information about flags and [crate::decimal::ArithmeticPolicy] see:
            /// [section](crate#arithmetic-result).
            #[must_use = doc::must_use_op!()]
            #[inline]
            pub const fn mul(self, rhs: Self, rounding_mode: RoundingMode) -> DecimalResult<Self> {
                math::mul::<{$bits / 64}, {2 * $bits / 64}>(self, rhs, rounding_mode)
            }
        }
    };
}

pub(crate) use ops_impl;
