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
        assert_eq_size!($UINT, [u8; { $bits / 8 }]);

        // TODO: 4 bytes(u32) unused because of alignment. We must use it.
        assert_eq_size!($DEC, [u8; { $bits / 8 } + 8]);
        assert_eq_size!($UDEC, [u8; { $bits / 8 } + 8]);
    };
}

test_impl!(128);
test_impl!(256);
test_impl!(512);
test_impl!(1024);

assert_eq_size!(RoundingMode, u8);
assert_eq_size!(Flags, u16);
assert_eq_size!(Context, u16);
