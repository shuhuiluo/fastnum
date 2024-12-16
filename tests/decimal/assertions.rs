use static_assertions::assert_eq_size;

use fastnum::{
    decimal::{Context, Flags, RoundingMode},
    *,
};

macro_rules! test_impl {
    ($bits: literal) => {
        paste::paste! { test_impl!($bits, [<U $bits>], [<D $bits>], [<UD $bits>]); }
    };
    ($bits: literal, $UINT: ident, $DEC: ident, $UDEC: ident) => {
        assert_eq_size!($UINT, [u64; { $bits / 64 }]);
        
        assert_eq_size!($DEC, [u64; { $bits / 64 } + 1]);
        assert_eq_size!($UDEC, [u64; { $bits / 64 } + 1]);
    };
}

test_impl!(128);
test_impl!(256);
test_impl!(512);
test_impl!(1024);

assert_eq_size!(RoundingMode, u8);
assert_eq_size!(Flags, u16);
assert_eq_size!(Context, u16);
