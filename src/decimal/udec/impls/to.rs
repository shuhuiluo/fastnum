use crate::decimal::UnsignedDecimal;

type UD<const N: usize> = UnsignedDecimal<N>;

impl<const N: usize> From<UD<N>> for f32 {
    #[inline]
    fn from(d: UD<N>) -> Self {
        f32::from(d.0)
    }
}

impl<const N: usize> From<UD<N>> for f64 {
    #[inline]
    fn from(d: UD<N>) -> Self {
        f64::from(d.0)
    }
}
