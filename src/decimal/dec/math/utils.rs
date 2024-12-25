use crate::decimal::{dec::ControlBlock, Decimal, Signal};

type D<const N: usize> = Decimal<N>;

#[inline(always)]
pub(crate) const fn overflow<const N: usize>(cb: ControlBlock) -> D<N> {
    D::INFINITY
        .with_ctx(cb.context())
        .with_cb(cb.raise_signal(Signal::overflow()))
}

#[inline(always)]
pub(crate) const fn underflow<const N: usize>(cb: ControlBlock) -> D<N> {
    D::ZERO
        .with_ctx(cb.context())
        .with_cb(cb.raise_signal(Signal::underflow()))
}

#[inline]
pub(crate) const fn overflow_exp<const N: usize>(exp: i32, cb: ControlBlock) -> D<N> {
    if exp > 0 {
        underflow(cb)
    } else {
        overflow(cb)
    }
}
