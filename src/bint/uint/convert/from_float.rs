macro_rules! from_float_impl {
    ($method: ident, $float: ident, $decoder: ident, $mant_bits: ident, $cast: ident) => {
        #[inline(always)]
        #[doc = doc::convert::from!($float U 256)]
        pub const fn $method(f: $float) -> Result<Self, ParseError> {
            if !f.is_finite() {
                return Err(ParseError::PosOverflow);
            }
            if f == 0.0 {
                return Ok(Self::ZERO);
            }
            if f.is_sign_negative() {
                return Err(ParseError::Signed);
            }
            let (mut mant, exp) = $decoder(f);
            if exp.is_negative() {
                mant = match mant.checked_shr((-exp) as ExpType) {
                    Some(mant) => mant,
                    None => 0,
                };
                if $mant_bits(mant) > Self::BITS {
                    return Err(ParseError::PosOverflow);
                }
                Ok(Self::$cast(mant))
            } else {
                if $mant_bits(mant) + exp as ExpType > Self::BITS {
                    return Err(ParseError::PosOverflow);
                }

                match Self::$cast(mant).checked_shl(exp as ExpType) {
                    Some(value) => Ok(value),
                    None => Err(ParseError::PosOverflow),
                }
            }
        }
    };
}

pub(crate) use from_float_impl;
