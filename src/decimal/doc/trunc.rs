macro_rules! trunc {
    ($bits: literal $($sign: ident)?) => {
        doc::doc_comment! {
            $bits $($sign)?,

            "Truncates the decimal number to integral with no fractional portion.\n"
            "This is a true truncation whereby no rounding is performed."
            "This operation is equivalent to [Self::rescale] or [Self::trunc_with_scale] with `scale` set to `0`.\n"

            "# Performance\n\n"
            "This operation is typically much faster than [Self::rescale]",

            #Panics
            doc::decimal_operation_panics!("truncate operation"),

            #Also
            "More about [`truncate`](crate#truncate) decimals."
            "[Self::trunc_with_scale]"
            "[Self::rescale]"
            "[Self::quantize]",

            #Examples
            ["assert_eq!(" doc::m!($bits $($sign)?) "(3.141).trunc(), " doc::m!($bits $($sign)?) "(3));"]
            ["assert_eq!(" doc::m!($bits $($sign)?) "(2.9).trunc(), " doc::m!($bits $($sign)?) "(2));"]
            [$(@ {$sign})? "assert_eq!(" doc::m!($bits) "(-1.98765).trunc(), " doc::m!($bits) "(-1));"]
            ["\n"]
            ["let ctx = decimal::Context::default().without_traps();"]
            ["assert!(" doc::type_str!($bits $($sign)?) "::INFINITY.with_ctx(ctx).trunc().is_nan());"]
            [$(@ {$sign})? "assert!(" doc::type_str!($bits $($sign)?) "::NEG_INFINITY.with_ctx(ctx).trunc().is_nan());"]
            ["assert!(" doc::type_str!($bits $($sign)?) "::NAN.with_ctx(ctx).trunc().is_nan());"]
        }
    };
}

pub(crate) use trunc;

macro_rules! trunc_with_scale {
    ($bits: literal $($sign: ident)?) => {
        doc::doc_comment! {
            $bits $($sign)?,

            "Truncates the decimal number to the given number of digits after the decimal point.\n"
            "This is a true truncation whereby no rounding is performed."
            "This operation is equivalent to [Self::rescale].\n"

            "# Performance\n\n"
            "This operation is typically much faster than [Self::rescale]",

            #Panics
            doc::decimal_operation_panics!("truncate operation"),

            #Also
            "More about [`truncate`](crate#truncate) decimals."
            "[Self::trunc]"
            "[Self::rescale]"
            "[Self::quantize]",

            #Examples
            ["assert_eq!(" doc::m!($bits $($sign)?) "(3.141592).trunc_with_scale(2), " doc::m!($bits $($sign)?) "(3.14));"]
            ["assert_eq!(" doc::m!($bits $($sign)?) "(3.141592).trunc_with_scale(3), " doc::m!($bits $($sign)?) "(3.141));"]
            ["assert_eq!(" doc::m!($bits $($sign)?) "(3.141592).trunc_with_scale(4), " doc::m!($bits $($sign)?) "(3.1415));"]
            ["assert_eq!(" doc::m!($bits $($sign)?) "(3.141592).trunc_with_scale(5), " doc::m!($bits $($sign)?) "(3.14159));"]
            ["assert_eq!(" doc::m!($bits $($sign)?) "(3.141592).trunc_with_scale(6), " doc::m!($bits $($sign)?) "(3.141592));"]
            [$(@ {$sign})? "assert_eq!(" doc::m!($bits) "(-1.98765).trunc_with_scale(1), " doc::m!($bits) "(-1.9));"]
            ["\n"]
            ["let ctx = decimal::Context::default().without_traps();"]
            ["assert!(" doc::type_str!($bits $($sign)?) "::INFINITY.with_ctx(ctx).trunc_with_scale(1).is_nan());"]
            [$(@ {$sign})? "assert!(" doc::type_str!($bits $($sign)?) "::NEG_INFINITY.with_ctx(ctx).trunc_with_scale(1).is_nan());"]
            ["assert!(" doc::type_str!($bits $($sign)?) "::NAN.with_ctx(ctx).trunc_with_scale(1).is_nan());"]
        }
    };
}

pub(crate) use trunc_with_scale;
