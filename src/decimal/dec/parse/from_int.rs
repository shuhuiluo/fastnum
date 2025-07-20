use crate::{
    bint::UInt,
    decimal::{
        dec::{Context, ControlBlock, ExtraPrecision, Signals},
        Decimal, ParseError, Sign,
    },
};

type D<const N: usize> = Decimal<N>;
type U<const N: usize> = UInt<N>;

macro_rules! from_int {
    ($n: ident, $nu: ident, $int: ident $(#$try: ident)?) => {
        from_int!(@ $($try)? $n, $nu, $int);
    };
    (@ $n: ident, $nu: ident, $int: ident) => {
        #[inline]
        pub const fn $n<const N: usize>(n: $int) -> D<N> {
            let sign = if n.is_negative() {
                Sign::Minus
            } else {
                Sign::Plus
            };

            D::new(
                U::$nu(n.unsigned_abs()),
                ControlBlock::new(
                    0,
                    sign,
                    Signals::empty(),
                    Context::default(),
                    ExtraPrecision::new(),
                ),
            )
        }
    };
    (@ TRY $n: ident, $nu: ident, $int: ident) => {
        #[inline]
        pub const fn $n<const N: usize>(n: $int) -> Result<D<N>, ParseError> {
            let sign = if n.is_negative() {
                Sign::Minus
            } else {
                Sign::Plus
            };

            let Ok(u) = U::$nu(n.unsigned_abs()) else {
                return Err(ParseError::PosOverflow);
            };

            Ok(D::new(
                u,
                ControlBlock::new(
                    0,
                    sign,
                    Signals::empty(),
                    Context::default(),
                    ExtraPrecision::new(),
                ),
            ))
        }
    };
}

from_int!(from_i8, from_u8, i8);
from_int!(from_i16, from_u16, i16);
from_int!(from_i32, from_u32, i32);
from_int!(from_i64, from_u64, i64);
from_int!(from_i128, from_u128, i128 #TRY);
from_int!(from_isize, from_usize, isize);
