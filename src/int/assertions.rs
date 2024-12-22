use crate::{utils::assert_eq_size, *};

macro_rules! assert_impl {
    ($UINT: ident, $bits: literal) => {
        assert_eq_size!($UINT, [u64; { $bits / 64 }]);
    };
}

assert_impl!(U128, 128);
assert_impl!(U256, 256);
assert_impl!(U512, 512);
assert_impl!(U1024, 1024);
