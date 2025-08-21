macro_rules! from {
    ($from: ident $sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Converts [`prim@" stringify!($from) "`] to " doc::link_type_str!($sign $bits) "."
        }
    };
}

pub(crate) use from;

macro_rules! to {
    ($to: ident $sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Converts " doc::link_type_str!($sign $bits) " to [`prim@" stringify!($to) "`]."
        }
    };
}

pub(crate) use to;

macro_rules! parse_str {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Parse " doc::link_type_str!($sign $bits) " from string using hexadecimal, binary or decimal base.\n\n"
            "# Panics\n\n"
            "This function will panic if " doc::link_type_str!($sign $bits) " can't be constructed\n"
            "from a given string.",

            "assert_eq!(" doc::type_str!($sign $bits) "::parse_str(\"0b1\"), " doc::m!($sign $bits) "(1));\n"
            "assert_eq!(" doc::type_str!($sign $bits) "::parse_str(\"0xA\"), " doc::m!($sign $bits) "(10));\n"
            "assert_eq!(" doc::type_str!($sign $bits) "::parse_str(\"12345\"), " doc::m!($sign $bits) "(12345));\n"
        }
    };
}

pub(crate) use parse_str;

macro_rules! parse_str_radix {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Parse " doc::link_type_str!($sign $bits) " from string using a given base to an integer.\n\n"
            "The string is expected to be an optional `+` sign followed by digits. Leading and trailing whitespace represent an error.\n"
            "Digits are a subset of these characters, depending on `radix`:\n"
            "- `0-9`\n"
            "- `a-z`\n"
            "- `A-Z`\n"
            "\n\n"
            "# Panics\n\n"
            "This function will panic if " doc::link_type_str!($sign $bits) " can't be constructed\n"
            "from a given string or if radix is not in the range from 2 to 36 inclusive.",

            "assert_eq!(" doc::type_str!($sign $bits) "::parse_str_radix(\"A\", 16), " doc::m!($sign $bits) "(10));\n"
        }
    };
}

pub(crate) use parse_str_radix;

macro_rules! from_str {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Try parse " doc::link_type_str!($sign $bits) " from string using hexadecimal, binary or decimal base.\n\n",

            "assert_eq!(" doc::type_str!($sign $bits) "::from_str(\"0b1\"), Ok(" doc::m!($sign $bits) "(1)));\n"
            "assert_eq!(" doc::type_str!($sign $bits) "::from_str(\"0xA\"), Ok(" doc::m!($sign $bits) "(10)));\n"
            "assert_eq!(" doc::type_str!($sign $bits) "::from_str(\"12345\"), Ok(" doc::m!($sign $bits) "(12345)));\n"
        }
    };
}

pub(crate) use from_str;

macro_rules! from_str_radix {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Try parse " doc::link_type_str!($sign $bits) " from string using a given base to an integer.\n\n"
            "The string is expected to be an optional `+` sign followed by digits. Leading and trailing whitespace represent an error.\n"
            "Digits are a subset of these characters, depending on `radix`:\n"
            "- `0-9`\n"
            "- `a-z`\n"
            "- `A-Z`\n"
            "\n\n"
            "# Panics\n\n"
            "This function will panic if radix is not in the range from 2 to 36 inclusive.",

            "assert_eq!(" doc::type_str!($sign $bits) "::from_str_radix(\"A\", 16), Ok(" doc::m!($sign $bits) "(10)));\n"
        }
    };
}

pub(crate) use from_str_radix;

macro_rules! to_str_radix {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Returns the " doc::link_type_str!($sign $bits) " integer as a string in the given radix.\n\n"
            "\n\n"
            "# Panics\n\n"
            "This function will panic if radix is not in the range from 2 to 36 inclusive.",

            "let src = \"934857djkfghhkdfgbf9345hdfkh\";\n"
            "let n = " doc::type_str!($sign $bits) "::from_str_radix(src, 36).unwrap();\n"
            "assert_eq!(n.to_str_radix(36), src);\n"
        }
    };
}

pub(crate) use to_str_radix;

macro_rules! parse_bytes {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Converts a byte slice in a given base to an " doc::link_type_str!($sign $bits) " integer.\n\n"
            "The input slice must contain ascii/utf8 characters in [0-9a-zA-Z].\n\n"
            "This function is equivalent to the [`from_str_radix`](#method.from_str_radix) function for a string slice equivalent to the byte slice and the same radix.\n\n"
            "Returns `None` if the conversion of the byte slice to string slice fails or if a digit is larger than or equal to the given radix, otherwise the integer is wrapped in `Some`."
            "\n\n"
            "# Panics\n\n"
            "This function will panic if radix is not in the range from 2 to 36 inclusive.",

            "let src = \"394857hdgfjhsnkg947dgfjkeita\";\n"
            "assert_eq!(" doc::type_str!($sign $bits) "::from_str_radix(src, 32).ok(), " doc::type_str!($sign $bits) "::parse_bytes(src.as_bytes(), 32));\n"
        }
    };
}

pub(crate) use parse_bytes;

macro_rules! from_radix_be {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Converts a slice of big-endian digits in the given radix to an " doc::link_type_str!($sign $bits) " integer.\n\n"
            "Each `u8` of the slice is interpreted as one digit of base `radix` of the number, so this function will return `None` if any digit is greater than or equal to `radix`, otherwise the integer is wrapped in `Some`.\n\n"

            "# Panics\n\n"
            "This function will panic if radix is not in the range from 2 to 256 inclusive.",

            "let n = " doc::m!($sign $bits) "(3459874852685);\n"
            "let digits = n.to_radix_be(12);\n"
            "assert_eq!(Some(n), " doc::type_str!($sign $bits) "::from_radix_be(&digits, 12));\n"
        }
    };
}

pub(crate) use from_radix_be;

macro_rules! from_radix_le {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Converts a slice of little-endian digits in the given radix to an " doc::link_type_str!($sign $bits) " integer.\n\n"
            "Each `u8` of the slice is interpreted as one digit of base `radix` of the number, so this function will return `None` if any digit is greater than or equal to `radix`, otherwise the integer is wrapped in `Some`.\n\n"

            "# Panics\n\n"
            "This function will panic if radix is not in the range from 2 to 256 inclusive.",

            "let n = " doc::m!($sign $bits) "(10983745987895);\n"
            "let digits = n.to_radix_le(15);\n"
            "assert_eq!(Some(n), " doc::type_str!($sign $bits) "::from_radix_le(&digits, 15));\n"
        }
    };
}

pub(crate) use from_radix_le;

macro_rules! to_radix_be {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Returns the " doc::link_type_str!($sign $bits) " integer in the given base in big-endian digit order.\n\n"

            "# Panics\n\n"
            "This function will panic if radix is not in the range from 2 to 256 inclusive.",

            "let digits = &[3, 55, 60, 100, 5, 0, 5, 88];\n"
            "let n = " doc::type_str!($sign $bits) "::from_radix_be(digits, 120).unwrap();\n"
            "assert_eq!(n.to_radix_be(120), digits);\n"
        }
    };
}

pub(crate) use to_radix_be;

macro_rules! to_radix_le {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Returns the " doc::link_type_str!($sign $bits) " integer in the given base in little-endian digit order.\n\n"

            "# Panics\n\n"
            "This function will panic if radix is not in the range from 2 to 256 inclusive.",

            "let digits = &[1, 67, 88, 200, 55, 68, 87, 120, 178];\n"
            "let n = " doc::type_str!($sign $bits) "::from_radix_le(digits, 250).unwrap();\n"
            "assert_eq!(n.to_radix_le(250), digits);\n"
        }
    };
}

pub(crate) use to_radix_le;
