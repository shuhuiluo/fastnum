use crate::decimal::{dec::convert, Decimal};

type D<const N: usize> = Decimal<N>;

impl<const N: usize> From<D<N>> for f32 {
    #[inline]
    fn from(d: D<N>) -> Self {
        convert::to_f32(d)
    }
}

impl<const N: usize> From<D<N>> for f64 {
    #[inline]
    fn from(d: D<N>) -> Self {
        convert::to_f64(d)
    }
}
