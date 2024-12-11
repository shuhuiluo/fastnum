#[macro_export]
macro_rules! with_context {
        (@ $context: ident [] -> [ $($r: tt)* ]) => {
            { $($r)* }
        };
        (@ $context: ident [ + ( $t:expr ) $($tts:tt)* ] -> [ $($r: tt)* ]) => {
            with_context!(@ $context [ $($tts)* ] -> [ $($r)*.add($t, $context) ])
        };
        (@ $context: ident [ + $t:tt $($tts:tt)* ] -> [ $($r: tt)* ]) => {
            with_context!(@ $context [ $($tts)* ] -> [ $($r)*.add($t, $context) ])
        };
        (@ $context: ident [ $t:tt $($tts:tt)* ] -> [ $($r: tt)* ]) => {
            with_context!(@ $context [ $($tts)* ] -> [ $($r)* $t ])
        };
        ($context: expr, { $($tts:tt)* }) => {
            {
                let __ctx = $context;
                let __res = with_context!(@ __ctx [ $($tts)* ] -> []);
                __res
            }
        };
    }

// mod t {
//     use crate::{decimal::Context, *};
//     
//     fn f() -> D128 {
//         with_context!(Context::default(), {
//             let a = dec128!(1);
//             let b = dec128!(2);
// 
//             a + b
//         })
//     }
// }