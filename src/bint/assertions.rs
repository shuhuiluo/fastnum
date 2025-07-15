use crate::{utils::assert_eq_size, *};

macro_rules! assert_impl {
    ($INT: ident, $UINT: ident, $bits: literal) => {
        assert_eq_size!($INT, [u64; { $bits / 64 }]);
        assert_eq_size!($UINT, [u64; { $bits / 64 }]);
    };
}

assert_impl!(I64, U64, 64);
assert_impl!(I128, U128, 128);
assert_impl!(I256, U256, 256);
assert_impl!(I512, U512, 512);
assert_impl!(I1024, U1024, 1024);
