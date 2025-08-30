macro_rules! ln {
    ($bits: literal $($sign: ident)?) => {
        doc::doc_comment! {
            $bits $($sign)?,

            "Returns the natural logarithm of the decimal number.\n"
            doc::decimal_inexact!("natural logarithm"),

            #Panics
            doc::decimal_operation_panics!("logarithm operation"),

            #Also
            "More about the [logarithm function](crate#logarithm-function).",

            #Examples
            ["assert_eq!(" doc::m!($bits $($sign)?) "(2).ln(), " doc::type_str!($bits) "::LN_2);"]
            ["assert_eq!(" doc::type_str!($bits $($sign)?) "::E.ln(), " doc::type_str!($bits) "::ONE);"]
        }
    };
}

pub(crate) use ln;

macro_rules! ln_1p {
    ($bits: literal $($sign: ident)?) => {
        doc::doc_comment! {
            $bits $($sign)?,

            "Returns natural logarithm _ln(1 + self)_ more accurately than if the operations were performed separately.\n"
            doc::decimal_inexact!("natural logarithm"),

            #Panics
            doc::decimal_operation_panics!("logarithm operation"),

            #Also
            "More about the [logarithm function](crate#logarithm-function).",

            #Examples
            ["assert_eq!((" doc::type_str!($bits $($sign)?) "::E - " doc::m!($bits $($sign)?) "(1)).ln_1p(), " doc::m!($bits) "(1));"]
        }
    };
}

pub(crate) use ln_1p;

macro_rules! log {
    ($bits: literal $($sign: ident)?) => {
        doc::doc_comment! {
            $bits $($sign)?,

            "Returns the logarithm of the decimal number with respect to the given arbitrary _base_.\n"
            doc::decimal_inexact!("logarithm"),

            #Panics
            doc::decimal_operation_panics!("logarithm operation"),

            #Also
            "More about the [logarithm function](crate#logarithm-function).",

            #Examples
            ["assert_eq!(" doc::m!($bits $($sign)?) "(64).log(" doc::m!($bits $($sign)?) "(2)), " doc::m!($bits) "(6));"]
            ["assert_eq!(" doc::m!($bits $($sign)?) "(27).log(" doc::m!($bits $($sign)?) "(3)), " doc::m!($bits) "(3));"]
            ["assert_eq!(" doc::m!($bits $($sign)?) "(15625).log(" doc::m!($bits $($sign)?) "(5)), " doc::m!($bits) "(6));"]
        }
    };
}

pub(crate) use log;

macro_rules! log2 {
    ($bits: literal $($sign: ident)?) => {
        doc::doc_comment! {
            $bits $($sign)?,

            "Returns the binary logarithm of the given decimal number.\n"
            doc::decimal_inexact!("logarithm"),

            #Panics
            doc::decimal_operation_panics!("logarithm operation"),

            #Also
            "More about the [logarithm function](crate#logarithm-function).",

            #Examples
            ["assert_eq!(" doc::m!($bits $($sign)?) "(64).log2(), " doc::m!($bits) "(6));"]
            ["assert_eq!(" doc::m!($bits $($sign)?) "(32).log2(), " doc::m!($bits) "(5));"]
            ["assert_eq!(" doc::m!($bits $($sign)?) "(1024).log2(), " doc::m!($bits) "(10));"]
            ["assert_eq!(" doc::m!($bits $($sign)?) "(0.5).log2(), " doc::m!($bits) "(-1));"]
            ["assert_eq!(" doc::m!($bits $($sign)?) "(0.25).log2(), " doc::m!($bits) "(-2));"]
            ["assert_eq!(" doc::m!($bits $($sign)?) "(10).log2(), " doc::type_str!($bits) "::LOG2_10);"]
        }
    };
}

pub(crate) use log2;

macro_rules! log10 {
    ($bits: literal $($sign: ident)?) => {
        doc::doc_comment! {
            $bits $($sign)?,

            "Returns the decimal logarithm of the given decimal number.\n"
            doc::decimal_inexact!("logarithm"),

            #Panics
            doc::decimal_operation_panics!("logarithm operation"),

            #Also
            "More about the [logarithm function](crate#logarithm-function).",

            #Examples
            ["assert_eq!(" doc::m!($bits $($sign)?) "(100).log10(), " doc::m!($bits) "(2));"]
            ["assert_eq!(" doc::m!($bits $($sign)?) "(1000).log10(), " doc::m!($bits) "(3));"]
            ["assert_eq!(" doc::m!($bits $($sign)?) "(0.1).log10(), " doc::m!($bits) "(-1));"]
            ["assert_eq!(" doc::m!($bits $($sign)?) "(0.01).log10(), " doc::m!($bits) "(-2));"]
            ["assert_eq!(" doc::m!($bits $($sign)?) "(2).log10(), " doc::type_str!($bits) "::LOG10_2);"]
        }
    };
}

pub(crate) use log10;
