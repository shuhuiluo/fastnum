macro_rules! carrying_mul {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            #carrying_mul,
            $sign $bits,
            "Calculates the \"full multiplication\" `self * rhs + carry`"
            "without the possibility to overflow.\n\n"
            "This returns the low-order (wrapping) bits and the high-order (overflow) bits\n"
            "of the result as two separate values, in that order.\n\n"
            "Performs \"long multiplication\" which takes in an extra amount to add, and may return an\n"
            "additional amount of overflow. This allows for chaining together multiple\n"
            "multiplications to create \"big integers\" which represent larger values.\n\n"
            "If you don't need the `carry`, then you can use [`Self::widening_mul`] instead.",

            "let a = " doc::type_str!($sign $bits) "::MAX;\n"
            "let b = " doc::type_str!($sign $bits) "::MAX;\n"
            "let c = " doc::type_str!($sign $bits) "::ZERO;\n\n"
            "assert_eq!(a.carrying_mul(b, c), (" doc::type_str!($sign $bits) "::ONE, b - " doc::type_str!($sign $bits) "::ONE));\n"
        }
    };
}

pub(crate) use carrying_mul;
