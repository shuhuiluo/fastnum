use crate::decimal::{unsigned::UnsignedDecimal, utils::name::TypeName};

macro_rules! macro_impl {
    ($bits: literal) => {
        impl TypeName for UnsignedDecimal<{ $bits / 64 }> {
            #[inline]
            fn type_name() -> &'static str {
                concat!("UD", $bits)
            }
        }
    };
}

macro_impl!(128);
macro_impl!(256);
macro_impl!(512);
macro_impl!(1024);
macro_impl!(2048);
macro_impl!(4096);
macro_impl!(8192);
