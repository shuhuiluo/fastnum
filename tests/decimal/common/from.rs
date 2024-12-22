macro_rules! test_impl {
    (D, $bits: literal) => {
        paste::paste! { test_impl!(SIGNED: $bits, [< dec $bits >], [<D $bits>]); }
    };
    (UD, $bits: literal) => {
        paste::paste! { test_impl!(UNSIGNED: $bits, [< udec $bits >], [<UD $bits>]); }
    };
    (UNSIGNED: $bits: tt, $dec: ident, $D: ident) => {
        mod $dec {
            use rstest::*;
            use fastnum::{*, decimal::*};
            
            super::test_impl!(COMMON:: $bits, $dec, $D, THIS);
            super::test_impl!(UNSIGNED:: $bits, $dec, $D, THIS);
        }
    };
    (SIGNED: $bits: tt, $dec: ident, $D: ident) => {
        mod $dec {
            use rstest::*;
            use fastnum::{*, decimal::*};
            
            super::test_impl!(COMMON:: $bits, $dec, $D, THIS);
            super::test_impl!(SIGNED:: $bits, $dec, $D, THIS);
        }
    };
    (COMMON:: 512, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(COMMON:: 256, $dec, $D);
        
        // f32
        #[rstest(::trace)]
        #[case(f32::MIN_POSITIVE, $dec!(1.1754943508222875079687365372222456778186655567720875215087517062784172594547271728515625e-38))]
        #[case(317e-40, $dec!(3.1700000098946435501119816090716154772221806896649747100732700841687651538425285480116144753992557525634765625E-38))]
        #[case(2.35098744048e-38, $dec!(2.350987440475957123602109243087866394712812961308427354153308831195379018097479928428583662025630474090576171875E-38))]
        #[case(2.3509889819e-38, $dec!(2.35098898190426788090088725919040801362055736959656341832065776397049129686767088287524529732763767242431640625E-38))]
        fn test_from_f32_ok_512(#[case] n: f32, #[case] expected: $D) {
            let d = $D::try_from(n).unwrap();
            assert_eq!(d, expected);
        }
        
        #[rstest(::trace)]
        #[case(1.0e-40, $dec!(0.9999946101114759581525919052273499496042205269619191850412790687494327124262838424328947439789772033691406250e-40))]
        #[case(1.0e-39, $dec!(1.00000021530533325742087560014568310926874564800968669110436609702256827159061458587530069053173065185546875000e-39))]
        #[case(3.92E-39, $dec!(3.91999933059456489828739575494312783522406115751507460249208160269472102366083987590172910131514072418212890625E-39))]
        #[case(1e-42, $dec!(1.0005271035279193886395429224690001177341070264998322610345467546973108330377044694614596664905548095703125e-42))]
        #[case(1.40129846432e-45, $dec!(1.40129846432481707092372958328991613128026194187651577175706828388979108268586060148663818836212158203125E-45))]
        fn test_from_f32_subnormal_512(#[case] n: f32, #[case] expected: $D) {
            assert!(n.is_subnormal());
            let d = $D::try_from(n).unwrap();
            assert_eq!(d, expected);
        }
        
        // f64
        #[rstest(::trace)]
        #[case(2.3509889819e-38, $dec!(2.350988981899999809754458784267977344449574846971330400802740014330933327934877246295418973536135720525663828084361739456653594970703125e-38))]
        fn test_from_f64_ok_512(#[case] n: f64, #[case] expected: $D) {
            let d = $D::try_from(n).unwrap();
            assert_eq!(d, expected);
        }
    };
    (UNSIGNED:: 512, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(UNSIGNED:: 256, $dec, $D);
    };
    (SIGNED:: 512, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(SIGNED:: 256, $dec, $D);
        
        // f32
        #[rstest(::trace)]
        #[case(-1e-42, $dec!(-1.0005271035279193886395429224690001177341070264998322610345467546973108330377044694614596664905548095703125e-42))]
        fn test_from_f32_subnormal_512_signed(#[case] n: f32, #[case] expected: $D) {
            assert!(n.is_subnormal());
            let d = $D::try_from(n).unwrap();
            assert_eq!(d, expected);
        }
        
        // f64
        #[rstest(::trace)]
        #[case(-2.3509889819e-38, $dec!(-2.350988981899999809754458784267977344449574846971330400802740014330933327934877246295418973536135720525663828084361739456653594970703125e-38))]
        fn test_from_f64_ok_512_signed(#[case] n: f64, #[case] expected: $D) {
            let d = $D::try_from(n).unwrap();
            assert_eq!(d, expected);
        }
    };
    
    
    (COMMON:: 256, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(COMMON:: 256, $dec, $D);
        
        // f32
        #[rstest(::trace)]
        #[case(f32::MIN_POSITIVE)]
        #[should_panic(expected = "(fastnum) number too large to fit in target type")]
        fn test_from_f32_overflow_256(#[case] n: f32) {
            let _ = $D::try_from(n).unwrap();
        }

        #[rstest(::trace)]
        #[case(1.0e-40)]
        #[case(1.0e-39)]
        #[case(3.92E-39)]
        #[case(1.40129846432e-45)]
        #[case(1e-42)]
        #[should_panic(expected = "(fastnum) number too large to fit in target type")]
        fn test_from_f32_subnormal_256(#[case] n: f32) {
            assert!(n.is_subnormal());
            let _ = $D::try_from(n).unwrap();
        }
    };
    (COMMON:: 256, $dec: ident, $D: ident) => {
        super::test_impl!(COMMON:: 128, $dec, $D);
        
        // f32
        #[rstest(::trace)]
        #[case(f32::MAX, $dec!(340282346638528859811704183484516925440))]
        fn test_from_f32_ok_256(#[case] n: f32, #[case] expected: $D) {
            let d = $D::try_from(n).unwrap();
            assert_eq!(d, expected);
        }
        
        // f64
        #[rstest(::trace)]
        #[case(0.1, $dec!(0.1000000000000000055511151231257827021181583404541015625))]
        #[case(0.001, $dec!(0.001000000000000000020816681711721685132943093776702880859375))]
        #[case(12.34, $dec!(12.339999999999999857891452847979962825775146484375))]
        #[case(0.333333333333333333333333333333, $dec!(0.333333333333333314829616256247390992939472198486328125))]
        #[case(1.0 / 3.0, $dec!(0.333333333333333314829616256247390992939472198486328125))]
        #[case(core::f64::consts::PI, $dec!(3.141592653589793115997963468544185161590576171875))]
        #[case(core::f64::consts::E, $dec!(2.718281828459045090795598298427648842334747314453125))]
        #[case(core::f64::consts::PI * 10000.0, $dec!(31415.926535897931898944079875946044921875))]
        #[case(core::f64::consts::PI * 30000.0, $dec!(94247.779607693795696832239627838134765625))]
        #[case(3.0000000000000004, $dec!(3.000000000000000444089209850062616169452667236328125))]
        #[case(0.07155292, $dec!(0.07155292000000000596227067717336467467248439788818359375))]
        #[case(21509.2, $dec!(21509.20000000000072759576141834259033203125))]
        #[case(2.3283064e-10, $dec!(2.328306399999999934987650668772826180463741962967105791904032230377197265625e-10))]
        #[case(0.14693861798803098, $dec!(0.146938617988030983951830421574413776397705078125))]
        #[case(6.99999952316, $dec!(6.9999995231599996259319595992565155029296875))]
        fn test_from_f64_ok_256(#[case] n: f64, #[case] expected: $D) {
            let d = $D::try_from(n).unwrap();
            assert_eq!(d, expected);
        }
    };
    (UNSIGNED:: 256, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(UNSIGNED:: 256, $dec, $D);
    };
    (UNSIGNED:: 256, $dec: ident, $D: ident) => {
        super::test_impl!(UNSIGNED:: 128, $dec, $D);
    };
    (SIGNED:: 256, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(SIGNED:: 256, $dec, $D);
    };
    (SIGNED:: 256, $dec: ident, $D: ident) => {
        super::test_impl!(SIGNED:: 128, $dec, $D);
    };
    
    (COMMON:: 128, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(COMMON:: 128, $dec, $D);
        
        // f32
        #[rstest(::trace)]
        #[case(f32::MIN_POSITIVE)]
        #[should_panic(expected = "(fastnum) number too large to fit in target type")]
        fn test_from_f32_overflow_128(#[case] n: f32) {
            let _ = $D::try_from(n).unwrap();
        }
        
        // f32
        #[rstest(::trace)]
        #[case(1.0e-40)]
        #[case(1.0e-39)]
        #[case(3.92E-39)]
        #[case(1.40129846432e-45)]
        #[case(1e-42)]
        #[should_panic(expected = "(fastnum) number too large to fit in target type")]
        fn test_from_f32_subnormal_128(#[case] n: f32) {
            assert!(n.is_subnormal());
            let _ = $D::try_from(n).unwrap();
        }
        
        // f64
        #[rstest(::trace)]
        #[case(0.1)]
        #[case(0.01)]
        #[case(12.34)]
        #[case(0.333333333333333333333333333333)]
        #[case(1.0 / 3.0)]
        #[case(core::f64::consts::PI)]
        #[case(core::f64::consts::E)]
        #[case(3.0000000000000004)]
        #[case(0.07155292)]
        #[case(21509.2)]
        #[case(2.3283064e-10)]
        #[case(0.14693861798803098)]
        #[case(6.99999952316)]
        #[should_panic(expected = "(fastnum) number too large to fit in target type")]
        fn test_from_f64_overflow_128(#[case] n: f64) {
            let _ = $D::try_from(n).unwrap();
        }
    };
    (COMMON:: 128, $dec: ident, $D: ident) => {
        // int
        super::test_impl!(FROM UINT $dec, $D, u8, u16, u32, u64, u128, usize);
        
        // f32
        super::test_impl!(FROM f32, test_from_f32_ok, $dec, $D);
        super::test_impl!(FROM INF f32, test_from_f32_infinity, $dec, $D);
        super::test_impl!(FROM NAN f32, 0b0111_1111_1100_0000_0000_0000_0000_0000, test_from_f32_nan, $dec, $D);
        
        #[rstest(::trace)]
        #[case(0.1, $dec!(0.100000001490116119384765625))]
        #[case(1e-1, $dec!(0.100000001490116119384765625))]
        #[case(2e-1, $dec!(0.20000000298023223876953125))]
        #[case(0.01, $dec!(0.00999999977648258209228515625))]
        #[case(1e-2, $dec!(0.00999999977648258209228515625))]
        #[case(0.001, $dec!(0.001000000047497451305389404296875))]
        #[case(1e-5, $dec!(0.00000999999974737875163555145263671875))]
        #[case(12.34, $dec!(12.340000152587890625))]
        #[case(0.3333333, $dec!(0.333333313465118408203125))]
        #[case(0.333333333333333333333333333333, $dec!(0.3333333432674407958984375))]
        #[case(1.0 / 3.0, $dec!(0.3333333432674407958984375))]
        #[case(core::f32::consts::PI, $dec!(3.1415927410125732421875))]
        #[case(core::f32::consts::PI * 10000.0, $dec!(31415.927734375))]
        #[case(core::f32::consts::PI * 30000.0, $dec!(94247.78125))]
        #[case(core::f32::consts::E, $dec!(2.71828174591064453125))]
        #[case(f32::EPSILON, $dec!(1.1920928955078125E-7))]
        #[case(3.0000000000000004, $dec!(3.0))]
        #[case(0.07155292, $dec!(0.07155291736125946044921875))]
        #[case(21509.2, $dec!(21509.19921875))]
        #[case(2289620000.0, $dec!(2289619968))]
        #[case(80000197e0, $dec!(80000200))]
        #[case(2.3283064e-10, $dec!(0.00000000023283064365386962890625))]
        #[case(0.14693861798803098, $dec!(0.146938621997833251953125))]
        #[case(1e20, $dec!(100000002004087734272))]
        #[case(1e30, $dec!(1000000015047466219876688855040))]
        #[case(1e38, $dec!(99999996802856924650656260769173209088))]
        #[case(317e36, $dec!(317000006395220278118691742155288870912))]
        #[case(6.99999952316, $dec!(6.999999523162841796875))]
        #[case(1.58456325029e+29, $dec!(158456325028528675187087900672))]
        #[case(4294967295., $dec!(4294967296))]
        fn test_from_f32_ok_ex(#[case] n: f32, #[case] expected: $D) {
            let d = $D::try_from(n).unwrap();
            assert_eq!(d, expected);
        }
        
        // f64
        super::test_impl!(FROM f64, test_from_f64_ok, $dec, $D);
        super::test_impl!(FROM INF f64, test_from_f64_infinity, $dec, $D);
        super::test_impl!(FROM NAN f64, 0b0111_1111_1111_1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000, test_from_f64_nan, $dec, $D);
        
        #[rstest(::trace)]
        #[case(f64::EPSILON, $dec!(2.220446049250313080847263336181640625e-16))]
        #[case(2289620000.0, $dec!(2289620000))]
        #[case(80000197e0, $dec!(80000197))]
        #[case(1e20, $dec!(100000000000000000000))]
        #[case(1e30, $dec!(1000000000000000019884624838656))]
        #[case(1e38, $dec!(99999999999999997748809823456034029568))]
        #[case(317e36, $dec!(317000000000000010053141138001136451584))]
        #[case(4294967295., $dec!(4294967295))]
        #[case(1.58456325029e+29, $dec!(158456325029000005035589894144))]
        fn test_from_f64_ok_ex(#[case] n: f64, #[case] expected: $D) {
            let d = $D::try_from(n).unwrap();
            assert_eq!(d, expected);
        }

        #[rstest(::trace)]
        #[case(8.544283616667655e-306)]
        #[case(3e300)]
        #[case(2.81341650018752E-308)]
        #[case(f64::MAX)]
        #[should_panic(expected = "(fastnum) number too large to fit in target type")]
        fn test_from_f64_overflow_ex(#[case] n: f64) {
            let _ = $D::try_from(n).unwrap();
        }

        #[rstest(::trace)]
        #[case(1.0e-308)]
        #[case(4.940656e-324)]
        #[should_panic(expected = "(fastnum) number too large to fit in target type")]
        fn test_from_f64_subnormal(#[case] n: f64) {
            assert!(n.is_subnormal());
            let _ = $D::try_from(n).unwrap();
        }
        
    };
    (UNSIGNED:: 128, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(UNSIGNED:: 128, $dec, $D);
    };
    (UNSIGNED:: 128, $dec: ident, $D: ident) => {
        // int
        super::test_impl!(FROM U INT $dec, $D, i8, i16, i32, i64, i128, isize);
        
        // f32
        super::test_impl!(FROM U f32, test_from_f32_unsigned_neg, $dec, $D);
        
        //f64
        super::test_impl!(FROM U f64, test_from_f64_unsigned_neg, $dec, $D);
    };
    (SIGNED:: 128, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(SIGNED:: 128, $dec, $D);
    };
    (SIGNED:: 128, $dec: ident, $D: ident) => {
        // int
        super::test_impl!(FROM INT $dec, $D, i8, i16, i32, i64, i128, isize);
        
        // f32
        super::test_impl!(FROM S f32, 32, test_from_f32_ok_signed, $dec, $D);
        super::test_impl!(FROM NEG INF f32, test_from_f32_neg_inf, $dec, $D);
        
        //f64
        super::test_impl!(FROM S f64, 64, test_from_f64_ok_signed, $dec, $D);
        super::test_impl!(FROM NEG INF f64, test_from_f64_neg_inf, $dec, $D);
        
        #[rstest(::trace)]
        #[case(f64::MIN)]
        #[should_panic(expected = "(fastnum) number too large to fit in target type")]
        fn test_from_f64_overflow_ex_signed(#[case] n: f64) {
            let _ = $D::try_from(n).unwrap();
        }
    };
    
    (FROM $f: ident, $name: ident, $dec: ident, $D: ident) => {
        #[rstest(::trace)]
        #[case(0., $dec!(0))]
        #[case(0.0, $dec!(0))]
        #[case(1., $dec!(1.0))]
        #[case(1.0, $dec!(1.0))]
        #[case(2.0, $dec!(2.0))]
        #[case(3.0, $dec!(3.0))]
        #[case(0.5, $dec!(0.5))]
        #[case(0.25, $dec!(0.25))]
        #[case(7.5, $dec!(7.5))]
        #[case(50., $dec!(50))]
        #[case(1234., $dec!(1234))]
        #[case(50000., $dec!(50000))]
        #[case(5.0 * 0.03125, $dec!(0.15625))]
        #[case(0.033203125, $dec!(0.033203125))]
        #[case(4.5, $dec!(4.5))]
        #[case(0.15625, $dec!(0.15625))]
        #[case(1401757440., $dec!(1401757440))]
        #[case(10000000., $dec!(10000000))]
        #[case(1048576., $dec!(1048576))]
        fn $name(#[case] n: $f, #[case] expected: $D) {
            let d = $D::try_from(n).unwrap();
            assert_eq!(d, expected);
        }
    };
    (FROM U $f: ident, $name: ident, $dec: ident, $D: ident) => {
        #[rstest(::trace)]
        #[case(-0.)]
        #[case(-0.0)]
        #[case(-1.0)]
        #[case($f::NEG_INFINITY)]
        #[case($f::MIN)]
        #[should_panic(expected = "(fastnum) number would be signed for unsigned type")]
        fn $name(#[case] n: $f) {
            let _ = $D::try_from(n).unwrap();
        }
    };
    (FROM S $f: ident, $bits: literal, $name: ident, $dec: ident, $D: ident) => {
        #[rstest(::trace)]
        #[case(-0., $dec!(-0))]
        #[case(-0.0, $dec!(-0))]
        #[case(-1., $dec!(-1.0))]
        #[case(-1.0, $dec!(-1.0))]
        #[case(-2.0, $dec!(-2.0))]
        #[case(-3.0, $dec!(-3.0))]
        #[case(-0.5, $dec!(-0.5))]
        #[case(-0.25, $dec!(-0.25))]
        #[case(-7.5, $dec!(-7.5))]
        #[case(-50., $dec!(-50))]
        #[case(-1234., $dec!(-1234))]
        #[case(-50000., $dec!(-50000))]
        #[case(-5.0 * 0.03125, $dec!(-0.15625))]
        #[case(-0.033203125, $dec!(-0.033203125))]
        #[case(-4.5, $dec!(-4.5))]
        #[case(-0.15625, $dec!(-0.15625))]
        #[case(-1401757440., $dec!(-1401757440))]
        #[case(-10000000., $dec!(-10000000))]
        fn $name(#[case] n: $f, #[case] expected: $D) {
            let d = $D::try_from(n).unwrap();
            assert_eq!(d, expected);
        }
    };
    (FROM INF $f: ident, $name: ident, $dec: ident, $D: ident) => {
        #[rstest(::trace)]
        fn $name() {
            let n = $f::INFINITY;
            let d = $D::try_from(n).unwrap();
            assert!(d.is_infinite());
        }
    };
    (FROM NEG INF $f: ident, $name: ident, $dec: ident, $D: ident) => {
        #[rstest(::trace)]
        fn $name() {
            let n = $f::NEG_INFINITY;
            let d = $D::try_from(n).unwrap();
            assert!(d.is_infinite());
            assert!(d.is_negative());
        }
    };
    (FROM NAN $f: ident, $bits: literal, $name: ident, $dec: ident, $D: ident) => {
        #[rstest(::trace)]
        #[case($f::NAN)]
        #[case($f::from_bits($bits))]
        fn $name(#[case] n: $f) {
            let d = $D::try_from(n).unwrap();
            assert!(d.is_nan());
        }
    };
    (FROM UINT $dec: ident, $D: ident, $($Pt: ty),*) => {
        $(
            paste::paste! {
                #[rstest(::trace)]
                #[case(0, $dec!(0))]
                #[case(1, $dec!(1))]
                #[case(10, $dec!(10))]
                #[case(100, $dec!(100))]
                #[case($Pt::MAX, $D::from_str(format!("{}", $Pt::MAX).as_str(), Context::default()).unwrap())]
                #[case($Pt::MAX - 1, $D::from_str(format!("{}", $Pt::MAX).as_str(), Context::default()).unwrap() - $dec!(1))]
                fn [< test_from_ $Pt >](#[case] n: $Pt, #[case] expected: $D) {
                    let d = $D::from(n);
                    assert_eq!(d, expected);
                }
            }
        )*
    };
    (FROM U INT $dec: ident, $D: ident, $($Pt: ty),*) => {
        $(
            paste::paste! {
                #[rstest(::trace)]
                #[case(0, $dec!(0))]
                #[case(1, $dec!(1))]
                #[case(10, $dec!(10))]
                #[case(100, $dec!(100))]
                #[case($Pt::MAX, $D::from_str(format!("{}", $Pt::MAX).as_str(), Context::default()).unwrap())]
                #[case($Pt::MAX - 1, $D::from_str(format!("{}", $Pt::MAX).as_str(), Context::default()).unwrap() - $dec!(1))]
                fn [< test_from_ $Pt >](#[case] n: $Pt, #[case] expected: $D) {
                    let d = $D::try_from(n).unwrap();
                    assert_eq!(d, expected);
                }
                
                #[rstest(::trace)]
                #[case(-1)]
                #[case(-10)]
                #[case($Pt::MIN)]
                #[should_panic(expected = "(fastnum) number would be signed for unsigned type")]
                fn [< test_from_ $Pt _negative>](#[case] n: $Pt) {
                     let _ = $D::try_from(n).unwrap();
                }
            }
        )*
    };
    (FROM INT $dec: ident, $D: ident, $($Pt: ty),*) => {
        $(
            paste::paste! {
                #[rstest(::trace)]
                #[case(-1, $dec!(-1))]
                #[case(-10, $dec!(-10))]
                #[case(-100, $dec!(-100))]
                #[case($Pt::MIN, $D::from_str(format!("{}", $Pt::MIN).as_str(), Context::default()).unwrap())]
                #[case($Pt::MIN + 1, $D::from_str(format!("{}", $Pt::MIN).as_str(), Context::default()).unwrap() + $dec!(1))]
                fn [< test_from_ $Pt >](#[case] n: $Pt, #[case] expected: $D) {
                    let d = $D::from(n);
                    assert_eq!(d, expected);
                }
            }
        )*
    };
}

pub(crate) use test_impl;

