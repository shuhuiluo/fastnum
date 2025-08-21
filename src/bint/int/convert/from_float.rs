macro_rules! from_float_impl {
    ($method: ident, $float: ident) => {
        #[inline(always)]
        #[doc = doc::convert::from!($float I 256)]
        pub const fn $method(f: $float) -> Result<Self, ParseError> {
            if f.is_sign_negative() {
                let i = match UInt::$method(-f) {
                    Ok(u) => Self::from_bits(u),
                    Err(e) => return Err(e),
                };

                if i.eq(&Self::MIN) {
                    Ok(Self::MIN)
                } else if i.is_negative() {
                    Err(ParseError::NegOverflow)
                } else {
                    Ok(i.neg())
                }
            } else {
                let i = match UInt::$method(f) {
                    Ok(u) => Self::from_bits(u),
                    Err(e) => return Err(e),
                };

                if i.is_negative() {
                    Err(ParseError::PosOverflow)
                } else {
                    Ok(i)
                }
            }
        }
    };
}

pub(crate) use from_float_impl;
