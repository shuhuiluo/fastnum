macro_rules! must_use_op {
    () => {
        "this returns the result of the operation, without modifying the original"
    };
}

pub(crate) use must_use_op;

macro_rules! decimal_type_doc {
    ($bits: literal, $sign: literal) => {
        concat!("Arbitrary precision fixed point ", $sign, " number with ", $bits, "-bit integer for decimal digits.")
    };
}

pub(crate) use decimal_type_doc;