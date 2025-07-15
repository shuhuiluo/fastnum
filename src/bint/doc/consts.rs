macro_rules! impl_desc {
    ($sign: ident) => {
        concat!("Associated constants for ", doc::text_sign!($sign), " integer type.")
    };
}

pub(crate) use impl_desc;

macro_rules! min {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "The minimum value that this type can represent.",

            "assert_eq!(!" doc::type_str!($sign $bits) "::MIN, " doc::type_str!($sign $bits) "::MAX);\n"
        }
    };
}

pub(crate) use min;

macro_rules! max {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "The maximum value that this type can represent.",

            "assert_eq!(" doc::type_str!($sign $bits) "::MAX.wrapping_add(" doc::type_str!($sign $bits) "::ONE), " doc::type_str!($sign $bits) "::MIN);\n"
        }
    };
}

pub(crate) use max;

macro_rules! bits {
    ($sign: ident $bits: literal, $digit_bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "The total number of bits that this type contains.",

            "assert_eq!(" doc::type_str!($sign $bits) "::BITS, " $digit_bits ");\n"
        }
    };
}

pub(crate) use bits;

macro_rules! bytes {
    ($sign: ident $bits: literal, $digit_bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "The total number of bytes that this type contains.",

            "assert_eq!(" doc::type_str!($sign $bits) "::BYTES, " $digit_bits " / 8);\n"
        }
    };
}

pub(crate) use bytes;

macro_rules! value_desc {
    ($sign: ident, $($lit: literal) +) => {
        concat!("The value of `", $($lit,)+ "` represented by this ", doc::text_sign!($sign), " integer type.")
    }
}

pub(crate) use value_desc;