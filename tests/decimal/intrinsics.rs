// use fastnum::*;
// use rstest::*;
//
// #[rstest(::trace)]
// fn test_() {
//     let a = dec128!(789.0120);
//     let b = dec128!(12.345);
//
//     let d = a / b;
//     println!("{a} / {b}: {d}");
// }

// rust_decimal:   789.0120 / 12.345 = 63.913487241798298906439854192
// fastnum (D64):  789.0120 / 12.345 = 63.91348724179829890
// (D64(digits=[6391348724179829890], exp=[-17], flags=[], signals=[!INEXACT,
// !ROUND], ctx=[R=No, S=!DBZ, !INV, !OFW], extra=[0.6439854])) fastnum (D128):
// 789.0120 / 12.345 = 63.913487241798298906439854191980558930
// (D128(digits=[63913487241798298906439854191980558930], exp=[-36], flags=[],
// signals=[!INEXACT, !ROUND], ctx=[R=No, S=!DBZ, !INV, !OFW],
// extra=[0.7411907]))
