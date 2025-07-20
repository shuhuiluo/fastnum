use crate::{
    bint::UInt,
    decimal::{
        dec::{Context, ControlBlock, ExtraPrecision, Sign, Signals},
        Decimal, ParseError,
    },
};

type D<const N: usize> = Decimal<N>;
type U<const N: usize> = UInt<N>;

macro_rules! from_uint {
    ($n: ident, $uint: ident $(#$try: ident)?) => {
        from_uint!(@ $($try)? $n, $uint);
    };
    (@ $n: ident, $uint: ident) => {
        #[inline]
        pub const fn $n<const N: usize>(n: $uint) -> D<N> {
            D::new(
                U::$n(n),
                ControlBlock::new(
                    0,
                    Sign::Plus,
                    Signals::empty(),
                    Context::default(),
                    ExtraPrecision::new(),
                ),
            )
        }
    };
    (@ TRY $n: ident, $uint: ident) => {
        #[inline]
        pub const fn $n<const N: usize>(n: $uint) -> Result<D<N>, ParseError> {
            let Ok(u) = U::$n(n) else {
                return Err(ParseError::PosOverflow);
            };

            Ok(D::new(
                u,
                ControlBlock::new(
                    0,
                    Sign::Plus,
                    Signals::empty(),
                    Context::default(),
                    ExtraPrecision::new(),
                ),
            ))
        }
    };
}

from_uint!(from_u8, u8);
from_uint!(from_u16, u16);
from_uint!(from_u32, u32);
from_uint!(from_u64, u64);
from_uint!(from_u128, u128 #TRY);
from_uint!(from_usize, usize);
