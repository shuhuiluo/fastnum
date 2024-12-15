use crate::decimal::{Decimal, Flags, Signal};

type D<const N: usize> = Decimal<N>;

#[inline(always)]
pub(crate) const fn overflow<const N: usize>(flags: Flags) -> D<N> {
    D::INFINITY.with_flags(flags.raise_signal(Signal::overflow()))
}

#[inline(always)]
pub(crate) const fn underflow<const N: usize>(flags: Flags) -> D<N> {
    D::ZERO.with_flags(flags.raise_signal(Signal::underflow()))
}

// TODO: overflow_exp()
#[inline]
pub(crate) const fn overflow_scale<const N: usize>(scale: i16, flags: Flags) -> D<N> {
    if scale >= 0 {
        D::INFINITY.with_flags(flags.raise_signal(Signal::overflow()))
    } else {
        D::ZERO.with_flags(flags.raise_signal(Signal::underflow()))
    }
}
