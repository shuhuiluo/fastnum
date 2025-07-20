macro_rules! common_float_impl {
    ($f: ident) => {
        pub use $crate::decimal::utils::types::$f::*;

        pub const MAX_10_EXP: i32 = $f::MAX_10_EXP;

        #[allow(dead_code)]
        pub const MIN_10_EXP: i32 = $f::MIN_10_EXP;

        /// The number of bits in the significand, *including* the hidden bit.
        pub const SIG_TOTAL_BITS: u32 = $f::MANTISSA_DIGITS;

        /// The number of bits in the significand, *excluding* the hidden bit.
        pub const SIG_BITS: u32 = SIG_TOTAL_BITS - 1;

        /// Number of bits in the exponent.
        pub const EXP_BITS: u32 = BITS - SIG_BITS - 1;

        /// The saturated (maximum bitpattern) value of the exponent, i.e. the infinite
        /// representation.
        ///
        /// This shifted fully right, use `EXP_MASK` for the shifted value.
        pub const EXP_SAT: u32 = (1 << EXP_BITS) - 1;

        /// Signed version of `EXP_SAT` since we convert a lot.
        pub const INFINITE_POWER: i32 = EXP_SAT as i32;

        /// The exponent bias value. This is also the maximum value of the exponent.
        pub const EXP_BIAS: u32 = EXP_SAT >> 1;

        /// Minimum exponent value of normal values.
        pub const EXP_MIN: i32 = -(EXP_BIAS as i32 - 1);

        /// Maximum exponent for a fast path case, or `⌊(SIG_BITS+1)/log2(5)⌋`
        pub const MAX_EXPONENT_FAST_PATH: i32 = {
            let log2_5 = core::f64::consts::LOG2_10 - 1.0;
            (SIG_TOTAL_BITS as f64 / log2_5) as i32
        };

        /// Minimum exponent for a fast path case, or `-⌊(SIG_BITS+1)/log2(5)⌋`
        pub const MIN_EXPONENT_FAST_PATH: i32 = -MAX_EXPONENT_FAST_PATH;

        /// Maximum exponent that can be represented for a disguised-fast path case.
        /// This is `MAX_EXPONENT_FAST_PATH + ⌊(SIG_BITS+1)/log2(10)⌋`
        pub const MAX_EXPONENT_DISGUISED_FAST_PATH: i32 =
            MAX_EXPONENT_FAST_PATH + (SIG_TOTAL_BITS as f64 / core::f64::consts::LOG2_10) as i32;

        /// Maximum mantissa for the fast-path (`1 << 53` for f64).
        pub const MAX_MANTISSA_FAST_PATH: u64 = 1 << SIG_TOTAL_BITS;

        #[inline(always)]
        pub const fn from_u64(v: u64) -> $f {
            debug_assert!(v <= MAX_MANTISSA_FAST_PATH);
            v as _
        }
    };
}

pub(crate) use common_float_impl;
