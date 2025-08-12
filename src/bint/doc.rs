pub(crate) mod bits;
pub(crate) mod carrying;
pub(crate) mod checked;
pub(crate) mod cmp;
pub(crate) mod consts;
pub(crate) mod convert;
pub(crate) mod endian;
pub(crate) mod impls;
pub(crate) mod num;
pub(crate) mod overflowing;
pub(crate) mod saturating;
pub(crate) mod strict;
pub(crate) mod widening;
pub(crate) mod wrapping;

pub(crate) use crate::doc::*;

macro_rules! int_type_doc {
    ($bits: literal, $sign: literal) => {
        concat!($bits, "-bit ", $sign, " integer type.")
    };
}

pub(crate) use int_type_doc;

macro_rules! type_str {
    ($sign: ident $bits: literal) => {
        concat!(stringify!($sign), $bits)
    };
}

pub(crate) use type_str;

macro_rules! link_type_str {
    ($sign: ident $bits: literal) => {
        concat!("[`", doc::type_str!($sign $bits), "`](crate::", doc::type_str!($sign $bits), ")")
    };
}

pub(crate) use link_type_str;

macro_rules! m {
    ($sign: ident $bits: literal) => {
        concat!(doc::small_sign!($sign), $bits, "!")
    };
}

pub(crate) use m;

macro_rules! example_header {
    ($sign: ident $bits: literal) => {
        concat!(
"

# Examples

Please note that this example is shared between integer types.
Which explains why ", doc::link_type_str!($sign $bits), " is used here.", "

```
use fastnum::*", ";

"
        )
    }
}

pub(crate) use example_header;

macro_rules! small_sign {
    (U) => {
        "u"
    };
    (I) => {
        "i"
    };
}

pub(crate) use small_sign;

macro_rules! text_sign {
    (U) => {
        "unsigned"
    };
    (I) => {
        "signed"
    };
}

pub(crate) use text_sign;

macro_rules! doc_comment {
    { $(# $method: ident, )? $sign: ident $bits: literal, $($($desc: expr)+)? $(, $($code: expr)+)? } => {
        concat!(
            $($($desc), +,)?
            $("\n\n", "See also: <https://doc.rust-lang.org/std/primitive.", doc::small_sign!($sign), "64.html#method.", stringify!($method), ">.", )?
            $(
                doc::example_header!($sign $bits),
                $($code), +,
                "\n```"
            )?
        )
    }
}

pub(crate) use doc_comment;

macro_rules! doc_comment_impl {
    ($($name: ident), *) => {
        $(
            macro_rules! $name {
                ($sign: ident $bits: literal) => {
                    doc::doc_comment! {
                        #$name,
                        $sign $bits,
                    }
                };
            }

            pub(crate) use $name;
        )*
    }
}

pub(crate) use doc_comment_impl;

macro_rules! arithmetic_impl_desc {
    ($name: literal, $method: literal, $rest: literal) => {
        concat!(
            $name,
            " arithmetic methods which act on `self`: `self.",
            $method,
            "_...`. ",
            $rest
        )
    };
}

pub(crate) use arithmetic_impl_desc;
