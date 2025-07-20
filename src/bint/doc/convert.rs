macro_rules! from {
    ($from: ident $sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Converts [`" stringify!($from) "`] to [" doc::type_str!($sign $bits) "]."
        }
    };
}

pub(crate) use from;

macro_rules! to {
    ($to: ident $sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Converts [" doc::type_str!($sign $bits) "] to [`" stringify!($from) "`]."
        }
    };
}

pub(crate) use to;

macro_rules! parse_str {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Parse [" doc::type_str!($sign $bits) "] from string.\n\n"
            "# Panics\n\n"
            "This function will panic if [" doc::type_str!($sign $bits) "] can't be constructed\n"
            "from a given string.",

            "assert_eq!(" doc::type_str!($sign $bits) "::parse_str(\"12345\"), " doc::m!($sign $bits) "(12345));\n"
        }
    };
}

pub(crate) use parse_str;

macro_rules! from_str {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Parse [" doc::type_str!($sign $bits) "] from string.\n\n"
            "# Panics\n\n"
            "This function will panic if [" doc::type_str!($sign $bits) "] can't be constructed\n"
            "from a given string.",

            "assert_eq!(" doc::type_str!($sign $bits) "::parse_str(\"12345\"), " doc::m!($sign $bits) "(12345));\n"
        }
    };
}

pub(crate) use from_str;

macro_rules! from_str_radix {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Parse [" doc::type_str!($sign $bits) "] from string.\n\n"
            "# Panics\n\n"
            "This function will panic if [" doc::type_str!($sign $bits) "] can't be constructed\n"
            "from a given string.",

            "assert_eq!(" doc::type_str!($sign $bits) "::parse_str(\"12345\"), " doc::m!($sign $bits) "(12345));\n"
        }
    };
}

pub(crate) use from_str_radix;
