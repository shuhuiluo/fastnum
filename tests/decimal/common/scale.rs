macro_rules! test_impl {
    (D, $bits: literal) => {
        paste::paste! { test_impl!(T: [< dec $bits >], [<D $bits>]); }
    };
    (UD, $bits: literal) => {
        paste::paste! { test_impl!(T: [< udec $bits >], [<UD $bits>]); }
    };
    (T: $dec: ident, $D: ident) => {
        mod $dec {
            use rstest::*;
            use fastnum::{$dec, $D};
            
            #[rstest(::trace)]
            #[case($dec!(1), 0)]
            #[case($dec!(0.001), -3)]
            #[case($dec!(1), -0)]
            #[case($dec!(1000), 3)]
            fn test_scale(#[case] d: $D, #[case] scale: i16) {
                assert_eq!(d, $D::from_scale(scale));
            }
        }
    };
}

pub(crate) use test_impl;
