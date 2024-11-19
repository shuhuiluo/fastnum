use crate::{
    decimal::{
        math::{result, DecimalResult, Flags},
        round::{round, RoundConsts},
        unsigned::UnsignedDecimal,
        RoundingMode,
    },
    int::UInt,
};

type UD<const N: usize> = UnsignedDecimal<N>;

#[inline]
pub(crate) const fn with_scale<const N: usize>(
    mut dec: UD<N>,
    new_scale: i64,
    rounding_mode: RoundingMode,
) -> DecimalResult<UD<N>> {
    if dec.value.is_zero() {
        dec.scale = new_scale;
        return result!(dec);
    }

    if new_scale == dec.scale {
        result!(dec)
    } else if new_scale > dec.scale {
        // increase number of zeros if it possible
        while new_scale > dec.scale {
            if dec.value.gt(&RoundConsts::<N>::MAX) {
                return result!(dec).overflow();
            } else {
                dec.value = dec.value.strict_mul(UInt::<N>::TEN);
                dec.scale += 1;
            }
        }
        result!(dec)
    } else {
        // round
        let mut flags = Flags::empty();
        let mut is_rounded;
        while new_scale < dec.scale {
            (dec.value, is_rounded) = round(dec.value, rounding_mode);
            dec.scale -= 1;

            if is_rounded {
                flags = flags.union(Flags::INEXACT);
            }

            if dec.value.is_zero() {
                dec.scale = new_scale;
                return result!(dec).add_flags(flags);
            }
        }
        result!(dec).add_flags(flags)
    }
}
