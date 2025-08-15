macro_rules! must_use_op {
    () => {
        "this returns the result of the operation, without modifying the original"
    };
}

pub(crate) use must_use_op;

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
