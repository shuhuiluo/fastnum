macro_rules! must_use_op {
    () => {
        "this returns the result of the operation, without modifying the original"
    };
}

pub(crate) use must_use_op;