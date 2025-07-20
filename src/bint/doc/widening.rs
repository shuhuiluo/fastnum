macro_rules! impl_desc {
    () => {
        doc::arithmetic_impl_desc!(
            "Widening",
            "widening",
            "Each method returns of the calculation without the possibility to overflow."
        )
    };
}

pub(crate) use impl_desc;

macro_rules! widening_mul {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            #widening_mul,
            $sign $bits,
            "Calculates the complete product `self * rhs` without the possibility to overflow.\n\n"
            "This returns the low-order (wrapping) bits and the high-order (overflow) bits\n"
            "of the result as two separate values, in that order.\n\n"
            "If you also need to add a carry to the wide result, then you want\n"
            "[`Self::carrying_mul`] instead.",

            "let a = " doc::type_str!($sign $bits) "::MAX;\n"
            "let b = " doc::type_str!($sign $bits) "::MAX;\n"
            "assert_eq!(a.widening_mul(b), (" doc::type_str!($sign $bits) "::ONE, b - " doc::type_str!($sign $bits) "::ONE));\n"
        }
    };
}

pub(crate) use widening_mul;
