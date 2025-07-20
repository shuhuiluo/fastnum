macro_rules! bitand {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Performs the `&` operation."
        }
    };
}

pub(crate) use bitand;

macro_rules! bitor {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Performs the `|` operation."
        }
    };
}

pub(crate) use bitor;

macro_rules! bitxor {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Performs the `^` operation."
        }
    };
}

pub(crate) use bitxor;

macro_rules! not {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Performs the unary `!` operation."
        }
    };
}

pub(crate) use not;
