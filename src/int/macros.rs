macro_rules! macro_impl {
    ($INT: ident, $bits: literal, $name: ident) => {
        #[macro_export]
        #[doc = concat!("A macro to construct ", $bits, "-bit [crate::", stringify!($INT), "] integer from literals.")]
        ///
        ///
        /// # Examples:
        /// ```
        #[doc = concat!("use fastnum::{", stringify!($name), ", ", stringify!($INT), "}")]
        /// use num_traits::{Signed, Zero};
        ///
        #[doc = concat!("const N: ", stringify!($INT), " = ", stringify!($name), "!(100);")]
        #[doc = concat!("let x = ", stringify!($name), "!(1);")]
        #[doc = concat!("assert!(", stringify!($name), "!(0).is_zero());")]
        #[doc = concat!("assert_eq!(", stringify!($name), "!(115792089237316195423570985008687907853269984665640564039457584007913129639935), ", stringify!($INT) ,"::MAX);")]
        /// ```
        macro_rules! $name {
            ($lit:expr) => {{
                const __UINT: $crate::$INT = $crate::int::parse::$name::parse_str(stringify!($lit));
                __UINT
            }};
        }
    };
}

macro_impl!(U128, 128, u128);
macro_impl!(U256, 256, u256);
macro_impl!(U512, 512, u512);
macro_impl!(U1024, 1024, u1024);
macro_impl!(U2048, 2048, u2048);
macro_impl!(U4096, 4096, u4096);
macro_impl!(U8192, 8192, u8192);

macro_impl!(I128, 128, i128);
macro_impl!(I256, 256, i256);
macro_impl!(I512, 512, i512);
macro_impl!(I1024, 1024, i1024);
macro_impl!(I2048, 2048, i2048);
macro_impl!(I4096, 4096, i4096);
macro_impl!(I8192, 8192, i8192);
