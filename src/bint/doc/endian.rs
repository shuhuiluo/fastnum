macro_rules! impl_desc {
    ($Ty: ident, $sign: ident) => {
        concat!(
            "Methods which convert a `",
            stringify!($Ty),
            "` to and from data stored in different endianness."
        )
    };
}

pub(crate) use impl_desc;

macro_rules! from_be {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            #from_be,
            $sign $bits,
            "Converts an integer from big endian to the target’s endianness.\n\n"
            "On big endian this is a no-op. On little endian the bytes are swapped."
        }
    };
}

pub(crate) use from_be;

macro_rules! from_le {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            #from_le,
            $sign $bits,
            "Converts an integer from little endian to the target’s endianness.\n\n"
            "On little endian this is a no-op. On big endian the bytes are swapped."
        }
    };
}

pub(crate) use from_le;

macro_rules! to_be {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            #to_be,
            $sign $bits,
            "Converts `self` from big endian to the target’s endianness.\n\n"
            "On big endian this is a no-op. On little endian the bytes are swapped."
        }
    };
}

pub(crate) use to_be;

macro_rules! to_le {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            #to_le,
            $sign $bits,
            "Converts `self` from little endian to the target’s endianness.\n\n"
            "On little endian this is a no-op. On big endian the bytes are swapped."
        }
    };
}

pub(crate) use to_le;

macro_rules! from_be_slice {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            #from_be_slice,
            $sign $bits,
            "Create an integer value from a slice of bytes in big endian.\n\n"
            "The value is wrapped in an [`Option`](https://doc.rust-lang.org/core/option/enum.Option.html) as the integer represented by the slice of bytes may represent an integer too large to be represented by the type.\n\n"
            "If the length of the slice is shorter than `Self::BYTES`, the slice is padded with zeros or ones at the start so that it's length equals `Self::BYTES`. It is padded with ones if the bytes represent a negative integer, otherwise it is padded with zeros.\n\n"
            "If the length of the slice is longer than `Self::BYTES`, `None` will be returned, unless the bytes represent a non-negative integer and leading zeros from the slice can be removed until the length of the slice equals `Self::BYTES`, or if the bytes represent a negative integer and leading ones from the slice can be removed until the length of the slice equals `Self::BYTES`.\n\n"
        }
    };
}

pub(crate) use from_be_slice;

macro_rules! from_le_slice {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            #from_le_slice,
            $sign $bits,
            "Creates an integer value from a slice of bytes in little endian.\n\n"
            "The value is wrapped in an [`Option`](https://doc.rust-lang.org/core/option/enum.Option.html) as the bytes may represent an integer too large to be represented by the type.\n\n"
            "If the length of the slice is shorter than `Self::BYTES`, the slice is padded with zeros or ones at the start so that it's length equals `Self::BYTES`. It is padded with ones if the bytes represent a negative integer, otherwise it is padded with zeros.\n\n"
            "If the length of the slice is longer than `Self::BYTES`, `None` will be returned, unless the bytes represent a non-negative integer and leading zeros from the slice can be removed until the length of the slice equals `Self::BYTES`, or if the bytes represent a negative integer and leading ones from the slice can be removed until the length of the slice equals `Self::BYTES`.\n\n"
        }
    };
}

pub(crate) use from_le_slice;
