macro_rules! from_uint_impl {
    ($($name:ident <- $num:ident),*) => {
        impl<const N: usize> Int<N> {
            $(
                #[inline(always)]
                #[doc = doc::convert::from!($num I 256)]
                pub const fn $name(n: $num) -> Self {
                    debug_assert!($num::BITS < Self::BITS);
                    Self::from_bits(UInt::$name(n))
                }
            )*
        }
    };
}

pub(crate) use from_uint_impl;

macro_rules! try_from_uint_impl {
    ($($from_uint:ident <- $uint:ident $(#$try: ident)?),*) => {
        impl<const N: usize> Int<N> {
            $(
                #[inline(always)]
                #[doc = doc::convert::from!($uint I 256)]
                pub const fn $from_uint(uint: $uint) -> Result<Self, ParseError> {
                    try_from_uint_impl!(@ $($try)? $from_uint <- $uint uint)
                }
            )*
        }
    };
    (@ $from_uint:ident <- $uint:ident $n: ident) => {{
        let u = UInt::$from_uint($n);
        try_from_uint_impl!(@@ u)
    }};
    (@ TRY $from_uint:ident <- $uint:ident $n: ident) => {{
        match UInt::$from_uint($n) {
            Ok(u) => {
                try_from_uint_impl!(@@ u)
            },
            Err(err) => Err(err),
        }
    }};
    (@@ $n: ident) => {
        if $n.le(&intrinsics::Intrinsics::<N>::MAX_INT_AS_UINT) {
            Ok(Self::from_bits($n))
        } else {
            Err(ParseError::PosOverflow)
        }
    };
}

pub(crate) use try_from_uint_impl;