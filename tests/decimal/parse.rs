// macro_rules! test_limits {
//     (D, $bits: literal) => {
//         paste::paste! { test_impl!(UNSIGNED: $bits, [< dec $bits >], [<D $bits>]); }
//         paste::paste! { test_impl!(SIGNED: $bits, [< dec $bits >], [<D $bits>]); }
//     };
//     (UD, $bits: literal) => {
//         paste::paste! { test_impl!(UNSIGNED: $bits, [< udec $bits >], [<UD $bits>]); }
//     };
//     (UNSIGNED: $bits: tt, $dec: ident, $D: ident) => {
//         mod $dec {
//             use rstest::*;
//             use fastnum::{$dec, $D, decimal::RoundingMode};
// 
//             super::test_impl!(UNSIGNED:: $bits, $dec, $D);
//         }
//     };
//     (SIGNED: $bits: tt, $dec: ident, $D: ident) => {
//         paste::paste! {
//             mod [< $dec _signed >]{
//                 // use rstest::*;
//                 // use fastnum::{$dec, $D, decimal::RoundingMode};
// 
//                 super::test_impl!(SIGNED:: $bits, $dec, $D);
//             }
//         }
//     };
//     (UNSIGNED:: $bits: tt, $dec: ident, $D: ident) => {
//         #[rstest(::trace)]
//         fn test_max() {
//             
//             let a = $D::MAX;
//             
//             
//             assert_eq!(x.round(digits, RoundingMode::HalfUp), y);
//             assert_eq!(x.round(digits, RoundingMode::Down), z);
//         }
//     };
//     (SIGNED:: $bits: tt, $dec: ident, $D: ident) => {};
// }
// 
// test_limits!(UD, 128);
// test_limits!(UD, 256);
// test_limits!(UD, 512);
// test_limits!(UD, 1024);
// test_limits!(UD, 2048);
// test_limits!(UD, 4096);
// test_limits!(UD, 8192);
// 
// 
// test_limits!(D, 128);
// test_limits!(D, 256);
// test_limits!(D, 512);
// test_limits!(D, 1024);
// test_limits!(D, 2048);
// test_limits!(D, 4096);
// test_limits!(D, 8192);