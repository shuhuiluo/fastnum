use crate::{
    decimal::{Decimal, Signal},
    int::{math::div_rem, UInt},
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn normalize<const N: usize>(mut d: D<N>) -> D<N> {
    if d.flags.is_special() {
        return d.raise_signal(Signal::OP_INVALID);
    }

    if d.digits.is_zero() {
        d.scale = 0;
    } else {
        let mut digits;
        let mut remainder;
        while !d.digits.is_zero() {
            (digits, remainder) = div_rem(d.digits, UInt::TEN);
            if remainder.is_zero() {
                if d.scale > i16::MIN {
                    d.digits = digits;
                    d.scale -= 1;
                } else {
                    return d.raise_signal(Signal::OP_SUBNORMAL);
                }
            } else {
                break;
            }
        }
    }

    d
}
