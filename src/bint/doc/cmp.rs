macro_rules! is_zero {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Returns whether `self` is zero.",

            "assert!(" doc::type_str!($sign $bits) "::ZERO.is_zero());\n"
            "assert!(!" doc::type_str!($sign $bits) "::ONE.is_zero());\n"
        }
    };
}

pub(crate) use is_zero;

macro_rules! is_one {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Returns whether `self` is one.",

            "assert!(" doc::type_str!($sign $bits) "::ONE.is_one());\n"
            "assert!(!" doc::type_str!($sign $bits) "::MAX.is_one());\n"
        }
    };
}

pub(crate) use is_one;

macro_rules! eq {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Tests for `self` and `other` values to be equal, and is used by `==`."
        }
    };
}

pub(crate) use eq;

macro_rules! ne {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Tests for `self` and `other` values to be not equal, and is used by `!=`."
        }
    };
}

pub(crate) use ne;

macro_rules! cmp {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "This method returns an [`Ordering`] between `self` and `other`.\n\n"
            "By convention, `self.cmp(&other)` returns the ordering matching the expression\n"
            "`self <operator> other` if true."
        }
    };
}

pub(crate) use cmp;

macro_rules! max {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Compares and returns the maximum of two values.\n\n"
            "Returns the second argument if the comparison determines them to be equal."
        }
    };
}

pub(crate) use max;

macro_rules! min {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Compares and returns the minimum of two values.\n\n"
            "Returns the first argument if the comparison determines them to be equal."
        }
    };
}

pub(crate) use min;

macro_rules! clamp {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Restrict a value to a certain interval.\n\n"
            "Returns `max` if `self` is greater than `max`, and `min` if `self` is less than `min`."
            "Otherwise this returns `self`."

            "# Panics\n\n"
            "Panics if `min > max`."
        }
    };
}

pub(crate) use clamp;

macro_rules! lt {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Tests less than (for `self` and `other`) and is used by the `<` operator."
        }
    };
}

pub(crate) use lt;

macro_rules! le {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator."
        }
    };
}

pub(crate) use le;

macro_rules! gt {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Tests greater than (for `self` and `other`) and is used by the `>` operator."
        }
    };
}

pub(crate) use gt;

macro_rules! ge {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator."
        }
    };
}

pub(crate) use ge;
