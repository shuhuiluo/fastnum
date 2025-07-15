use crate::bint::doc;

macro_rules! impl_desc {
    () => {
        doc::arithmetic_impl_desc!("Strict", "strict", "Each method will always panic if overflow/underflow occurs (i.e. when the checked equivalent would return `None`), regardless of whether overflow checks are enabled.")
    };
}

pub(crate) use impl_desc;

doc::doc_comment_impl!(
    strict_abs,
    strict_add,
    strict_add_signed,
    strict_add_unsigned,
    strict_div,
    strict_div_euclid,
    strict_mul,
    strict_neg,
    strict_pow,
    strict_rem,
    strict_rem_euclid,
    strict_shl,
    strict_shr,
    strict_sub,
    strict_sub_unsigned
);

macro_rules! strict_power_of_ten {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Returns an integer whose value is 10^power.\n\n"

            "# Panics\n\n"
            "This function will panic if `10^power` is greater than [Self::MAX]",

            "assert_eq!(" doc::type_str!($sign $bits) "::strict_power_of_ten(2), " doc::m!($sign $bits) "(100));\n"
        }
    };
}

pub(crate) use strict_power_of_ten;
