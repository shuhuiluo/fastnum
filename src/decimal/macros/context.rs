/// Apply given [`Decimal Context`](crate::decimal::Context) to the set of primitive arithmetic operations.
/// 
/// # Examples
///
/// ```
/// use fastnum::{*, decimal::*};
///
/// let ctx = Context::default().with_signal_traps(SignalsTraps::empty());
///
/// // Perform all operations inside our Context without any traps. This code never panics.
/// let res = with_context!(ctx, {
///     let a = dec256!(1.0);
///     let b = dec256!(0);
/// 
///     // No panic! Inside this Context we can divide by zero!
///     a / b
/// });
///
/// assert!(res.is_infinite());
/// assert!(res.is_op_div_by_zero());
/// assert!(res.is_op_invalid());
/// ```
/// 
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
    (@ $context: ident [ - ( $t:expr ) $($tts:tt)* ] -> [ $($r: tt)* ]) => {
        with_context!(@ $context [ $($tts)* ] -> [ $($r)*.sub($t, $context) ])
    };
    (@ $context: ident [ - $t:tt $($tts:tt)* ] -> [ $($r: tt)* ]) => {
        with_context!(@ $context [ $($tts)* ] -> [ $($r)*.sub($t, $context) ])
    };
    (@ $context: ident [ * ( $t:expr ) $($tts:tt)* ] -> [ $($r: tt)* ]) => {
        with_context!(@ $context [ $($tts)* ] -> [ $($r)*.mul($t, $context) ])
    };
    (@ $context: ident [ * $t:tt $($tts:tt)* ] -> [ $($r: tt)* ]) => {
        with_context!(@ $context [ $($tts)* ] -> [ $($r)*.mul($t, $context) ])
    };
    (@ $context: ident [ / ( $t:expr ) $($tts:tt)* ] -> [ $($r: tt)* ]) => {
        with_context!(@ $context [ $($tts)* ] -> [ $($r)*.div($t, $context) ])
    };
    (@ $context: ident [ / $t:tt $($tts:tt)* ] -> [ $($r: tt)* ]) => {
        with_context!(@ $context [ $($tts)* ] -> [ $($r)*.div($t, $context) ])
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
