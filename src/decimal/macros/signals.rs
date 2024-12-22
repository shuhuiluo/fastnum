/// Macro helper for fast initializing of signals set.
///
/// # Examples
///
/// ```
/// use fastnum::{UD256, udec256, signals};
/// use fastnum::decimal::Signal;
///
/// let signals = signals![!OFW, !CP];
///
/// assert_eq!(signals, Signal::OP_OVERFLOW.combine(Signal::OP_CLAMPED));
/// ```
#[macro_export]
macro_rules! signals {
    [ $(! $tts: tt),* ] => {{
        const __SIGNALS: $crate::decimal::Signal = signals!(@ [$($tts),*]);
        __SIGNALS
    }};
    (@ []) => {
        $crate::decimal::Signal::EMPTY
    };
    (@ CP) => {
        $crate::decimal::Signal::OP_CLAMPED
    };
    (@ DBZ) => {
        $crate::decimal::Signal::OP_DIV_BY_ZERO
    };
    (@ INEXACT) => {
        $crate::decimal::Signal::OP_INEXACT
    };
    (@ INV) => {
        $crate::decimal::Signal::OP_INVALID
    };
    (@ OFW) => {
        $crate::decimal::Signal::OP_OVERFLOW
    };
    (@ ROUND) => {
        $crate::decimal::Signal::OP_ROUNDED
    };
    (@ SN) => {
        $crate::decimal::Signal::OP_SUBNORMAL
    };
    (@ UFW) => {
        $crate::decimal::Signal::OP_UNDERFLOW
    };
    (@ [$t:tt]) => {
        signals!(@ $t)
    };
    (@ [$t:tt, $($tts:tt),*]) => {
        signals!(@ $t).combine(signals!(@ [$($tts),*]))
    };
}
