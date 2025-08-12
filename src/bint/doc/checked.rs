use crate::bint::doc;

macro_rules! impl_desc {
    () => {
        doc::arithmetic_impl_desc!(
            "Checked",
            "checked",
            "Each method cannot panic and returns an `Option<Self>`. `None` is returned when overflow would have occurred or there was an attempt to divide by zero or calculate a remainder with a divisor of zero."
        )
    };
}

pub(crate) use impl_desc;

doc::doc_comment_impl!(
    checked_abs,
    checked_add,
    checked_add_signed,
    checked_add_unsigned,
    checked_div,
    checked_div_euclid,
    checked_ilog,
    checked_ilog10,
    checked_ilog2,
    checked_mul,
    checked_neg,
    checked_next_multiple_of,
    checked_pow,
    checked_rem,
    checked_rem_euclid,
    checked_shl,
    checked_shr,
    checked_sub,
    checked_sub_unsigned
);

macro_rules! checked_power_of_five {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Returns an integer whose value is 5^power.",

            "assert_eq!(" doc::type_str!($sign $bits) "::checked_power_of_five(2), Some(" doc::m!($sign $bits) "(25)));\n"
        }
    };
}

pub(crate) use checked_power_of_five;

macro_rules! checked_power_of_ten {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Returns an integer whose value is 10^power.",

            "assert_eq!(" doc::type_str!($sign $bits) "::checked_power_of_ten(2), Some(" doc::m!($sign $bits) "(100)));\n"
        }
    };
}

pub(crate) use checked_power_of_ten;

macro_rules! checked_next_power_of_two {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            #checked_next_power_of_two,
            $sign $bits,
            "Returns the smallest power of two greater than or equal to `self`.\n\n"
            "If the next power of two is greater than `Self::MAX`, `None` is returned, otherwise the power of two is wrapped in `Some`.",

            "let n = " doc::m!($sign $bits) "(2);\n"
            "assert_eq!(n.checked_next_power_of_two(), Some(n));\n\n"
            "let m = " doc::m!($sign $bits) "(3);\n"
            "assert_eq!(" doc::type_str!($sign $bits) "::MAX.checked_next_power_of_two(), None);\n"
        }
    };
}

pub(crate) use checked_next_power_of_two;

macro_rules! checked_mul_digit {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Checked integer multiplication by single [`prim@u64`] digit. Computes self * rhs, returning None if overflow occurred.\n\n"
        }
    };
}

pub(crate) use checked_mul_digit;
