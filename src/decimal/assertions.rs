use crate::{utils::assert_eq_size, *};

macro_rules! assert_impl {
    ($DEC: ident, $UDEC: ident, $bits: literal) => {
        assert_eq_size!($DEC, [u64; { $bits / 64 } + 1]);
        assert_eq_size!($UDEC, [u64; { $bits / 64 } + 1]);
    };
}

assert_impl!(D128, UD128, 128);
assert_impl!(D256, UD256, 256);
assert_impl!(D512, UD512, 512);
// assert_impl!(D1024, UD1024, 1024);
