use crate::{
    decimal::{
        dec::{ControlBlock, ExtraPrecision},
        Decimal,
    },
    int::convert,
};

type D<const N: usize> = Decimal<N>;

macro_rules! from_uint {
    ($n: ident, $uint: ty) => {
        #[inline]
        pub const fn $n<const N: usize>(n: $uint) -> D<N> {
            D::new(
                convert::$n(n),
                0,
                ControlBlock::default(),
                ExtraPrecision::new(),
            )
        }
    };
}

from_uint!(from_u8, u8);
from_uint!(from_u16, u16);
from_uint!(from_u32, u32);
from_uint!(from_u64, u64);
from_uint!(from_u128, u128);
from_uint!(from_usize, usize);
