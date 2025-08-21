macro_rules! from_str_impl {
    ($Ty: ident, $sign: ident, $Int: ident) => {
        #[doc = doc::convert::parse_str!($sign 256)]
        #[must_use = doc::must_use_op!()]
        #[inline(always)]
        pub const fn parse_str(s: &str) -> Self {
            match Self::from_str(s) {
                Ok(n) => n,
                Err(e) => panic!("{}", e.description()),
            }
        }

        #[doc = doc::convert::parse_str_radix!($sign 256)]
        #[must_use = doc::must_use_op!()]
        #[inline(always)]
        pub const fn parse_str_radix(s: &str, radix: u32) -> Self {
            match Self::from_str_radix(s, radix) {
                Ok(n) => n,
                Err(e) => panic!("{}", e.description()),
            }
        }

        #[doc = doc::convert::from_str!($sign 256)]
        #[must_use = doc::must_use_op!()]
        #[inline(always)]
        pub const fn from_str(s: &str) -> Result<Self, ParseError> {
            let buf = s.as_bytes();
            if buf.len() > 1 && buf[0] == b'0' && buf[1] == b'x' {
                #[allow(unsafe_code)]
                let s = unsafe { from_utf8_unchecked(buf.split_at(2).1) };
                Self::from_str_radix(s, 16)
            } else if buf.len() > 1 && buf[0] == b'0' && buf[1] == b'b' {
                #[allow(unsafe_code)]
                let s = unsafe { from_utf8_unchecked(buf.split_at(2).1) };
                Self::from_str_radix(s, 2)
            } else {
                Self::from_str_radix(s, 10)
            }
        }

        #[doc = doc::convert::from_str_radix!($sign 256)]
        #[must_use = doc::must_use_op!()]
        #[inline(always)]
        pub const fn from_str_radix(s: &str, radix: u32) -> Result<Self, ParseError> {
            match $Int::from_str_radix(s, radix) {
                Ok(val) => Ok(Self(val)),
                Err(e) => Err(from_int_error_kind(e.kind())),
            }
        }
    };
}

pub(crate) use from_str_impl;
