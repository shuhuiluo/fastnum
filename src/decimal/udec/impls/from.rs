use crate::decimal::{Decimal, DecimalError, ParseError, UnsignedDecimal};

type D<const N: usize> = Decimal<N>;
type UD<const N: usize> = UnsignedDecimal<N>;

impl<const N: usize> TryFrom<D<N>> for UD<N> {
    type Error = DecimalError;

    #[inline]
    fn try_from(d: D<N>) -> Result<Self, Self::Error> {
        if d.is_negative() {
            return Err(DecimalError::Invalid);
        }
        Ok(Self::new(d))
    }
}

macro_rules! from_uint_impl {
    ($($name:ident $uint:ident $(#$try: ident)?),*) => {
        impl<const N: usize> UD<N> {
            $(
                from_uint_impl!(@ $($try)? $name $uint);
            )*
        }

        $(
            from_uint_impl!(@@ $($try)? $name $uint);
        )*
    };
    (@ $name:ident $uint:ident) => {
        #[inline]
        #[doc = concat!("Converts [`", stringify!($uint), "`] to [Decimal].")]
        pub const fn $name(n: $uint) -> Self {
            Self::new(D::$name(n))
        }
    };
    (@ TRY $name:ident $uint:ident) => {
        #[inline]
        #[doc = concat!("Converts [`", stringify!($uint), "`] to [Decimal].")]
        pub const fn $name(n: $uint) -> Result<Self, ParseError> {
            match D::$name(n) {
                Ok(d) => Ok(Self::new(d)),
                Err(e) => Err(e),
            }
        }
    };
    (@@ $name:ident $uint:ident) => {
        impl<const N: usize> From<$uint> for UD<N> {
            #[inline]
            fn from(n: $uint) -> Self {
                Self::$name(n)
            }
        }
    };
    (@@ TRY $name:ident $uint:ident) => {
        impl<const N: usize> TryFrom<$uint> for UD<N> {
            type Error = ParseError;

            #[inline]
            fn try_from(n: $uint) -> Result<Self, Self::Error> {
                Self::$name(n)
            }
        }
    };
}

macro_rules! from_int_impl {
    ($($name:ident $int:ident $(#$try: ident)?),*) => {
        impl<const N: usize> UD<N> {
            $(
                #[inline]
                #[doc = concat!("Try converts [`", stringify!($int), "`] to [UnsignedDecimal].")]
                pub const fn $name(int: $int) -> Result<Self, ParseError> {
                    if int < 0 {
                        return Err(ParseError::Signed);
                    }

                    from_int_impl!(@ $($try)? $name int)
                }
            )*
        }

        $(
            impl<const N: usize> TryFrom<$int> for UD<N> {
                type Error = ParseError;

                #[inline]
                fn try_from(int: $int) -> Result<Self, Self::Error> {
                    Self::$name(int)
                }
            }
        )*
    };
    (@ $name:ident $int:ident) => {{
        Ok(Self::new(D::$name($int)))
    }};
    (@ TRY $name:ident $int:ident) => {{
        match D::$name($int) {
            Ok(d) => Ok(Self::new(d)),
            Err(e) => Err(e),
        }
    }};
}

macro_rules! try_from_f_impl {
    ($($name:ident $pname:ident $num:ty,)*) => {
        impl<const N: usize> UD<N> {
            $(
                #[inline]
                #[doc = concat!("Try converts [`", stringify!($num), "`] to [UnsignedDecimal].")]
                pub const fn $name(n: $num) -> Result<Self, ParseError> {
                    if n.is_sign_negative() {
                        return Err(ParseError::Signed);
                    }
                    Ok(Self::new(D::$pname(n)))
                }
            )*
        }

        $(
            impl<const N: usize> TryFrom<$num> for UD<N> {
                type Error = ParseError;

                #[inline]
                fn try_from(n: $num) -> Result<Self, Self::Error> {
                    Self::$name(n)
                }
            }
        )*
    };
}

from_uint_impl!(
    from_u8 u8,
    from_u16 u16,
    from_u32 u32,
    from_u64 u64,
    from_u128 u128 #TRY,
    from_usize usize
);

from_int_impl!(
    from_i8 i8,
    from_i16 i16,
    from_i32 i32,
    from_i64 i64,
    from_i128 i128 #TRY,
    from_isize isize
);

try_from_f_impl!(
    from_f32 from_f32 f32,
    from_f64 from_f64 f64,
);
