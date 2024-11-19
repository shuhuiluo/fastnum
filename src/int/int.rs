/// Big signed integer generic type, of fixed size which must be known at
/// compile time.
pub type Int<const N: usize> = bnum::BInt<N>;

use crate::int::doc::int_type_doc;

macro_rules! int_types {
    ($($bits: literal $i: ident; ) *)  => {
        $(
            #[doc = int_type_doc!($bits, "signed")]
            pub type $i = Int::<{$bits / 64}>;
        )*
    };
}

int_types!(
    128 I128;
    256 I256;
    512 I512;
    1024 I1024;
    2048 I2048;
    4096 I4096;
    8192 I8192;
);
