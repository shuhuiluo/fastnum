macro_rules! macro_impl {
    ($INT: ident, $bits: literal, $name: ident) => {
        #[doc = concat!("Parse utils for ", $bits, "-bit [crate::", stringify!($INT), "] integer from string literals.")]
        pub mod $name {

            use crate::int::error::from_int_error_kind;
            use crate::int::ParseError;
            use crate::$INT;

            #[inline]
            pub const fn parse_str(s: &str) -> $INT {
                match from_str(s) {
                    Ok(n) => n,
                    Err(e) => panic!("{}", e.description()),
                }
            }
            
            #[inline]
            const fn from_str(s: &str) -> Result<$INT, ParseError> {
                if let Some(val) = const_str::strip_prefix!(s, "0x") {
                    from_str_radix(val, 16)
                } else {
                    from_str_radix(s, 10)
                }
            }
            
            #[inline]
            const fn from_str_radix(s: &str, radix: u32) -> Result<$INT, ParseError> {
                match $INT::from_str_radix(s, radix) {
                    Ok(val) => Ok(val),
                    Err(e) => Err(from_int_error_kind(e.kind())),
                }
            }
        }
    };
}

macro_impl!(U128, 128, u128);
macro_impl!(U256, 256, u256);
macro_impl!(U512, 512, u512);
