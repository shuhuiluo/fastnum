use rstest::*;

use fastnum::{decimal::RoundingMode, udec128};

use crate::decimal::common::round::test_impl;

test_impl!(UD, 128);
test_impl!(UD, 256);
test_impl!(UD, 512);
test_impl!(D, 128);
test_impl!(D, 256);
test_impl!(D, 512);

#[rstest(::trace)]
fn test_round_smoke() {
    let n = udec128!(129.41675);

    assert_eq!(n.round(2, RoundingMode::Up), udec128!(129.42));
    assert_eq!(n.round(-1, RoundingMode::Down), udec128!(120));
    assert_eq!(n.round(4, RoundingMode::HalfUp), udec128!(129.4168));
    assert_eq!(n.round(4, RoundingMode::HalfEven), udec128!(129.4168));
    assert_eq!(n.round(4, RoundingMode::HalfDown), udec128!(129.4167));
}
