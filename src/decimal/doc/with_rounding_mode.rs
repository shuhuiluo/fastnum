macro_rules! with_rounding_mode {
    ($bits: literal $($sign: ident)?) => {
        doc::doc_comment! {
            $bits $($sign)?,

            "Apply new [RoundingMode] to the given decimal number.\n"
            "Returns a copy of the value with an updated rounding mode in its context.\n"
            "This method generally does not immediately change the mathematical value; it only sets"
            "the rounding rule that will be used by subsequent operations that may round"
            "(e.g., add, sub, mul, div, round, rescale, quantize, conversions, etc.).\n"

            "**Important:**"
            "- The change is local to the returned value and does not affect other values."
            "- If you ignore the returned value, the rounding mode update is lost."
            "- If the value currently carries extra precision, that extra precision is"
            "  rounded using the newly provided rounding mode at the moment this method"
            "  is applied. This ensures internal consistency of the stored representation with the new rounding rule.",

            #Panics
            doc::decimal_operation_panics!("possible extra precision rounding operation"),

            #Also
            "More about [`round`](crate#rounding) decimals."
            "[RoundingMode]",

            #Examples
            ["let a = " doc::m!($bits $($sign)?) "(1).with_rounding_mode(decimal::RoundingMode::No);"]
            ["let b = " doc::m!($bits $($sign)?) "(3).with_rounding_mode(decimal::RoundingMode::No);"]
            ["let c = " doc::m!($bits $($sign)?) "(6).with_rounding_mode(decimal::RoundingMode::No);"]
            [""]
            ["assert_eq!(((a / b) * c).with_rounding_mode(decimal::RoundingMode::HalfUp), " doc::m!($bits $($sign)?) "(2));"]
        }
    };
}

pub(crate) use with_rounding_mode;
