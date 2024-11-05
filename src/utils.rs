macro_rules! err_prefix {
    () => {
        "(fastnum)"
    };
}

#[allow(unused_macros)]
macro_rules! err_msg {
    ($msg: expr) => {
        concat!($crate::utils::err_prefix!(), " ", $msg)
    };
}

#[allow(unused_imports)]
pub(crate) use err_msg;
pub(crate) use err_prefix;
