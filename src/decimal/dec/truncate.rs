use crate::decimal::{dec::scale, Decimal};

type D<const N: usize> = Decimal<N>;

#[inline(always)]
pub(crate) const fn truncate<const N: usize>(mut d: D<N>, scale: i16) -> D<N> {
    // TODO: performance optimization
    // We do not need to keep the extra precision
    scale::rescale(&mut d, scale);
    d.cb.reset_extra_precision();
    d
}
