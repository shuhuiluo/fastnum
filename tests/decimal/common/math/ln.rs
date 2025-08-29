macro_rules! test_impl {
    (D, $bits: literal) => {
        paste::paste! { test_impl!(SIGNED: $bits, [< dec $bits >], [<D $bits>]); }
    };
    (UD, $bits: literal) => {
        paste::paste! { test_impl!(UNSIGNED: $bits, [< udec $bits >], [<UD $bits>], [< dec $bits >], [<D $bits>]); }
    };
    (UNSIGNED: $bits: tt, $dec: ident, $D: ident, $sdec: ident, $SD: ident) => {
        mod $dec {
            use rstest::*;
            use fastnum::{*, decimal::*};

            super::test_impl!(COMMON:: $bits, $dec, $D, $sdec, $SD, THIS);
            super::test_impl!(UNSIGNED:: $bits, $dec, $D, $sdec, $SD, THIS);
        }
    };
    (SIGNED: $bits: tt, $dec: ident, $D: ident) => {
        mod $dec {
            use rstest::*;
            use fastnum::{*, decimal::*};

            super::test_impl!(COMMON:: $bits, $dec, $D, $dec, $D, THIS);
            super::test_impl!(SIGNED:: $bits, $dec, $D, THIS);
        }
    };
    (COMMON:: 512, $dec: ident, $D: ident, $sdec: ident, $SD: ident, THIS) => {
        super::test_impl!(COMMON:: 256, $dec, $D, $sdec, $SD);

        #[rstest(::trace)]
        #[case($dec!(3),                       $sdec!(1.0986122886681096913952452369225257046474905578227494517346943336374942932186089668736157548137320887879700290659578657423680042259305198210528018707672774))]
        #[case($dec!(4),                       $sdec!(1.386294361119890618834464242916353136151000268720510508241360018986787243939389431211726653992837375084002962041141371467371040471516261114065341503270152))]
        #[case($dec!(7),                       $sdec!(1.945910149055313305105352743443179729637084729581861188459390149937579862752069267787658498587871526993061694205851140911723752257677786843148958095163901))]
        #[case($dec!(9.95682444689608e-60),    $sdec!(-135.8568473895164500599613458425294918462557573171482152153639743822659185277582573333598780973750766761695226444169670427437369890574579403237457741927138))]
        fn test_ln_512(#[case] d: $D, #[case] expected: $SD) {
            let res = d.ln();

            assert_eq!(res, expected);
            assert!(res.is_op_inexact());
            assert!(res.is_op_rounded());
        }
    };
    (UNSIGNED:: 512, $dec: ident, $D: ident, $sdec: ident, $SD: ident, THIS) => {
        super::test_impl!(UNSIGNED:: 256, $dec, $D, $sdec, $SD);
    };
    (SIGNED:: 512, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(SIGNED:: 256, $dec, $D);
    };


    (COMMON:: 256, $dec: ident, $D: ident, $sdec: ident, $SD: ident, THIS) => {
        super::test_impl!(COMMON:: 256, $dec, $D, $sdec, $SD);

        #[rstest(::trace)]
        #[case($dec!(3),                       $sdec!(1.09861228866810969139524523692252570464749055782274945173469433363749429321861))]
        #[case($dec!(4),                       $sdec!(1.3862943611198906188344642429163531361510002687205105082413600189867872439394))]
        #[case($dec!(7),                       $sdec!(1.9459101490553133051053527434431797296370847295818611884593901499375798627521))]
        #[case($dec!(9.95682444689608e-60),    $sdec!(-135.85684738951645005996134584252949184625575731714821521536397438226591852776))]
        fn test_ln_256(#[case] d: $D, #[case] expected: $SD) {
            let res = d.ln();

            assert_eq!(res, expected);
            assert!(res.is_op_inexact());
            assert!(res.is_op_rounded());
        }
    };
    (COMMON:: 256, $dec: ident, $D: ident, $sdec: ident, $SD: ident) => {
        super::test_impl!(COMMON:: 128, $dec, $D, $sdec, $SD);
    };
    (UNSIGNED:: 256, $dec: ident, $D: ident, $sdec: ident, $SD: ident, THIS) => {
        super::test_impl!(UNSIGNED:: 256, $dec, $D, $sdec, $SD);
    };
    (UNSIGNED:: 256, $dec: ident, $D: ident, $sdec: ident, $SD: ident) => {
        super::test_impl!(UNSIGNED:: 128, $dec, $D, $sdec, $SD);
    };
    (SIGNED:: 256, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(SIGNED:: 256, $dec, $D);
    };
    (SIGNED:: 256, $dec: ident, $D: ident) => {
        super::test_impl!(SIGNED:: 128, $dec, $D);
    };

    (COMMON:: 128, $dec: ident, $D: ident, $sdec: ident, $SD: ident, THIS) => {
        super::test_impl!(COMMON:: 128, $dec, $D, $sdec, $SD);

        #[rstest(::trace)]
        #[case($dec!(3),                       $sdec!(1.09861228866810969139524523692252570465))]
        #[case($dec!(4),                       $sdec!(1.38629436111989061883446424291635313615))]
        #[case($dec!(7),                       $sdec!(1.94591014905531330510535274344317972964))]
        #[case($dec!(9.95682444689608e-60),    $sdec!(-135.856847389516450059961345842529491846))]
        fn test_ln_128(#[case] d: $D, #[case] expected: $SD) {
            let res = d.ln();

            assert_eq!(res, expected);
            assert!(res.is_op_inexact());
            assert!(res.is_op_rounded());
        }
    };
    (COMMON:: 128, $dec: ident, $D: ident, $sdec: ident, $SD: ident) => {
        #[rstest(::trace)]
        #[case($dec!(1), $SD::ZERO, signals![])]
        #[case($dec!(2), $SD::LN_2, signals![!ROUND, !INEXACT])]
        fn test_ln(#[case] d: $D, #[case] expected: $SD, #[case] signals: Signals) {
            let res = d.ln();

            assert_eq!(res, expected);
            assert_eq!(res.op_signals(), signals);
        }

        #[rstest(::trace)]
        #[case($D::ZERO)]
        fn test_ln_zero(#[case] d: $D) {
            let ctx = Context::default().without_traps();
            let res = d.with_ctx(ctx).ln();

            assert!(res.is_negative());
            assert!(res.is_infinite());
        }

        #[rstest(::trace)]
        #[case($D::NAN)]
        fn test_ln_nan(#[case] d: $D) {
            let ctx = Context::default().without_traps();
            let res = d.with_ctx(ctx).ln();

            assert!(res.is_nan());
            assert!(res.is_op_invalid());
        }
    };
    (UNSIGNED:: 128, $dec: ident, $D: ident, $sdec: ident, $SD: ident, THIS) => {
        super::test_impl!(UNSIGNED:: 128, $dec, $D, $sdec, $SD);
    };
    (UNSIGNED:: 128, $dec: ident, $D: ident, $sdec: ident, $SD: ident) => {
    };
    (SIGNED:: 128, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(SIGNED:: 128, $dec, $D);
    };
    (SIGNED:: 128, $dec: ident, $D: ident) => {
        #[rstest(::trace)]
        #[case($dec!(-1))]
        fn test_ln_neg(#[case] d: $D) {
            let ctx = Context::default().without_traps();
            let res = d.with_ctx(ctx).ln();

            assert!(res.is_nan());
            assert!(res.is_op_invalid());
        }
    };
}

pub(crate) use test_impl;
