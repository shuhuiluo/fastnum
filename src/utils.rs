macro_rules! err_prefix {
    () => {
        "(fastnum)"
    };
}

pub(crate) use err_prefix;

macro_rules! err_msg {
    ($msg: expr) => {
        concat!($crate::utils::err_prefix!(), " ", $msg)
    };
}

pub(crate) use err_msg;

#[allow(unused_macros)]
macro_rules! assert_eq_size {
    ($x:ty, $($xs:ty),+ $(,)?) => {
        #[cfg(debug_assertions)]
        const _: fn() = || {
            $(let _ = core::mem::transmute::<$x, $xs>;)+
        };
    };
}

#[allow(unused_imports)]
pub(crate) use assert_eq_size;

#[allow(unused_macros)]
macro_rules! const_assert_eq {
    ($x:expr, $y:expr $(,)?) => {
        $crate::utils::const_assert!($x.eq($y));
    };
}

#[allow(unused_imports)]
pub(crate) use const_assert_eq;

#[allow(unused_macros)]
macro_rules! const_assert {
    ($x:expr $(,)?) => {
        #[allow(unknown_lints, clippy::eq_op)]
        #[cfg(debug_assertions)]
        const _: [(); 0 - !{
            const __ASSERT: bool = $x;
            __ASSERT
        } as usize] = [];
    };
}

#[allow(unused_imports)]
pub(crate) use const_assert;

#[allow(unused_macros)]
macro_rules! result_expect {
    ($res: expr, $msg: expr) => {
        match $res {
            Ok(value) => value,
            _ => panic!($msg),
        }
    };
}

#[allow(unused_imports)]
pub(crate) use result_expect;
