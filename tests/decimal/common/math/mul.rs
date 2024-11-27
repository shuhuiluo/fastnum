macro_rules! test_impl {
    (D, $bits: literal) => {
        paste::paste! { test_impl!(UNSIGNED: $bits, [< dec $bits >], [<D $bits>]); }
        paste::paste! { test_impl!(SIGNED: $bits, [< dec $bits >], [<D $bits>]); }
    };
    (UD, $bits: literal) => {
        paste::paste! { test_impl!(UNSIGNED: $bits, [< udec $bits >], [<UD $bits>]); }
    };
    (UNSIGNED: $bits: tt, $dec: ident, $D: ident) => {
        mod $dec {
            use rstest::*;
            use fastnum::{$dec, $D, decimal::{ArithmeticError, ArithmeticPolicy, RoundingMode, RoundingPolicy, OverflowPolicy}};
            
            super::test_impl!(UNSIGNED:: $bits, $dec, $D);
        }
    };
    (SIGNED: $bits: tt, $dec: ident, $D: ident) => {
        paste::paste! {
            mod [< $dec _signed >]{
                use rstest::*;
                use fastnum::{$dec, $D, decimal::{ArithmeticError, RoundingMode, ArithmeticPolicy, OverflowPolicy, RoundingPolicy}};
                
                super::test_impl!(SIGNED:: $bits, $dec, $D);
            }
        }
    };
    (UNSIGNED:: 512, $dec: ident, $D: ident) => {
        super::test_impl!(UNSIGNED:: 256, $dec, $D);
    };
    (SIGNED:: 512, $dec: ident, $D: ident) => {
        super::test_impl!(SIGNED:: 256, $dec, $D);
    };
    (UNSIGNED:: 256, $dec: ident, $D: ident) => {
        super::test_impl!(UNSIGNED:: 128, $dec, $D);
        
        #[rstest(::trace)]
        #[case($dec!(995052931372975485719.533153137), $dec!(4.523087321), $dec!(4500711297616988541501.836966993116075977))]
        #[case($dec!(8.37664968), $dec!(1.9086963714056968482094712882596748), $dec!(15.988480848752691653730876239769592670324064))]
        fn test_mul_256(#[case] a: $D, #[case] b: $D, #[case] expected: $D) {
            let prod = a * b;
            
            assert_eq!(prod, expected);
            assert_eq!(prod.fractional_digits_count(), expected.fractional_digits_count());
            
            let mut a = a;
            
            a *= b;
            assert_eq!(a, expected);
            assert_eq!(a.fractional_digits_count(), expected.fractional_digits_count());
        }
    };
    (SIGNED:: 256, $dec: ident, $D: ident) => {
        super::test_impl!(SIGNED:: 128, $dec, $D);
        
        #[rstest(::trace)]
        #[case($dec!(-995052931372975485719.533153137), $dec!(4.523087321), $dec!(-4500711297616988541501.836966993116075977))]
        #[case($dec!(995052931372975485719.533153137), $dec!(-4.523087321), $dec!(-4500711297616988541501.836966993116075977))]
        #[case($dec!(-8.37664968), $dec!(-1.9086963714056968482094712882596748), $dec!(15.988480848752691653730876239769592670324064))]
        fn test_mul_256(#[case] a: $D, #[case] b: $D, #[case] expected: $D) {
            let prod = a * b;
            
            assert_eq!(prod, expected);
            assert_eq!(prod.fractional_digits_count(), expected.fractional_digits_count());
            
            let mut a = a;
            
            a *= b;
            assert_eq!(a, expected);
            assert_eq!(a.fractional_digits_count(), expected.fractional_digits_count());
        }
    };
    (UNSIGNED:: 128, $dec: ident, $D: ident) => {
        #[rstest(::trace)]
        #[case($dec!(0), $dec!(0), $dec!(0))]
        #[case($dec!(0), $dec!(1), $dec!(0))]
        #[case($dec!(1), $dec!(0), $dec!(0))]
        #[case($dec!(1), $dec!(1), $dec!(1))]
        #[case($dec!(2), $dec!(1), $dec!(2))]
        #[case($dec!(12.34), $dec!(1.234), $dec!(15.22756))]
        #[case($dec!(2e1), $dec!(1), $dec!(2e1))]
        #[case($dec!(3), $dec!(0.333333), $dec!(0.999999))]
        #[case($dec!(2389472934723), $dec!(209481029831), $dec!(500549251119075878721813))]
        #[case($dec!(1e-450), $dec!(1e500), $dec!(0.1e51))]
        #[case($dec!(8.37664968), $dec!(0), $dec!(0.00000000))]
        #[case($dec!(8.561), $dec!(10), $dec!(85.610))]
        #[case($D::MAX, $dec!(0), $dec!(0e9223372036854775808))]
        #[case($D::MAX, $dec!(1), $D::MAX)]
        #[case($dec!(10000), $dec!(638655273892892437), $dec!(6386552738928924370000))]
        #[case($dec!(1e-10), $dec!(9056180052657301), $dec!(905618.0052657301))]
        #[case($dec!(34028236692093846346337460743176821145), $dec!(1.0), $dec!(34028236692093846346337460743176821145.0))]
        fn test_mul(#[case] a: $D, #[case] b: $D, #[case] expected: $D) {
            let prod = a * b;
            
            assert_eq!(prod, expected);
            assert_eq!(prod.fractional_digits_count(), expected.fractional_digits_count());
            
            let mut a = a;
            
            a *= b;
            assert_eq!(a, expected);
            assert_eq!(a.fractional_digits_count(), expected.fractional_digits_count());
        }
        
        #[rstest(::trace)]
        #[case($D::MAX, $dec!(2))]
        #[case($D::MAX, $dec!(10))]
        #[should_panic(expected = "(fastnum) attempt to perform the operation with overflow")]
        fn test_mul_overflow_panic(#[case] a: $D, #[case] b: $D) {
            let _ = a * b;
        }
        
        #[rstest(::trace)]
        #[case($D::MAX, $dec!(0.3))]
        #[case($D::MAX, $dec!(1.1e-9223372036854775806))]
        fn test_mul_inexact(#[case] a: $D, #[case] b: $D) {
            let _ = a * b;
            let res = a.mul(b, RoundingMode::HalfUp);
            let policy = ArithmeticPolicy::new(OverflowPolicy::Strict, RoundingPolicy::Strict);
            assert_eq!(res.ok_or_err_with_policy(&policy).unwrap_err(), ArithmeticError::Inexact);
        }
    };
    (SIGNED:: 128, $dec: ident, $D: ident) => {
        #[rstest(::trace)]
        #[case($dec!(-0), $dec!(+0), $dec!(-0))]
        #[case($dec!(-0), $dec!(+1), $dec!(-0))]
        #[case($dec!(-1), $dec!(0), $dec!(-0))]
        #[case($dec!(1), $dec!(-1), $dec!(-1))]
        #[case($dec!(-2), $dec!(1), $dec!(-2))]
        #[case($dec!(-2), $dec!(-1), $dec!(2))]
        #[case($dec!(-8.37664968), $dec!(0), $dec!(-0.00000000))]
        #[case($dec!(-9e-1), $dec!(-368408638655273892892437473), $dec!(331567774789746503603193725.7))]
        #[case($dec!(-1.175470587012343730098), $dec!(577575785), $dec!(-678923347.038065234601180476930))]
        #[case($dec!(1e-10), $dec!(-9056180052657301), $dec!(-905618.0052657301))]
        #[case($D::MIN, $dec!(0), $dec!(-0e9223372036854775808))]
        #[case($D::MIN, $dec!(1), $D::MIN)]
        fn test_mul(#[case] a: $D, #[case] b: $D, #[case] expected: $D) {
            let prod = a * b;
            
            assert_eq!(prod, expected);
            assert_eq!(prod.fractional_digits_count(), expected.fractional_digits_count());
            
            let mut a = a;
            
            a *= b;
            assert_eq!(a, expected);
            assert_eq!(a.fractional_digits_count(), expected.fractional_digits_count());
        }
        
        #[rstest(::trace)]
        #[case($D::MIN, $dec!(2))]
        #[case($D::MIN, $dec!(10))]
        #[case($D::MIN, $D::MAX)]
        #[case($D::MIN, $dec!(1.1e9223372036854775806))]
        #[should_panic(expected = "(fastnum) attempt to perform the operation with overflow")]
        fn test_mul_overflow_panic(#[case] a: $D, #[case] b: $D) {
            let _ = a * b;
        }
        
        #[rstest(::trace)]
        #[case($D::MIN, $dec!(0.3))]
        #[case($D::MIN, $dec!(1.1e-9223372036854775806))]
        fn test_mul_inexact(#[case] a: $D, #[case] b: $D) {
            let _ = a * b;
            let res = a.mul(b, RoundingMode::HalfUp);
            let policy = ArithmeticPolicy::new(OverflowPolicy::Strict, RoundingPolicy::Strict);
            assert_eq!(res.ok_or_err_with_policy(&policy).unwrap_err(), ArithmeticError::Inexact);
        }
    };
}

pub(crate) use test_impl;