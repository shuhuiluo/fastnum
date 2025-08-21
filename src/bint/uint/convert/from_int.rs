macro_rules! try_from_int_impl {
    ($($from_int:ident <- $int:ident ($(#$try: ident)? $from_uint:ident <- $uint:ident)),*) => {
        $(
            try_from_int_impl!(@ $from_int <- $int ($($try)? $from_uint <- $uint));
        )*
    };
    (@ $from_int:ident <- $int:ident ($from_uint:ident <- $uint:ident)) => {
        #[inline(always)]
        #[doc = doc::convert::from!($int U 256)]
        pub const fn $from_int(int: $int) -> Result<Self, ParseError> {
            if int < 0 {
                 return Err(ParseError::PosOverflow);
            } else {
                Ok(Self::$from_uint(int as $uint))
            }
        }
    };
    (@ $from_int:ident <- $int:ident (TRY $from_uint:ident <- $uint:ident)) => {
        #[inline(always)]
        #[doc = doc::convert::from!($int U 256)]
        pub const fn $from_int(int: $int) -> Result<Self, ParseError> {
            if int < 0 {
                 return Err(ParseError::PosOverflow);
            } else {
                Self::$from_uint(int as $uint)
            }
        }
    };
}

pub(crate) use try_from_int_impl;
