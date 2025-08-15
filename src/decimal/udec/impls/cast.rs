use crate::{
    decimal::{Decimal, ParseError, UnsignedDecimal},
    utils::const_generics::{Dimension, Narrow, Widen},
    Cast, TryCast,
};

type D<const N: usize> = Decimal<N>;
type UD<const N: usize> = UnsignedDecimal<N>;

impl<const N: usize, const M: usize> Cast<UD<N>> for UD<M>
where
    Dimension<N, M>: Widen,
{
    #[inline(always)]
    fn cast(self) -> UD<N> {
        UD::new(self.0.cast())
    }
}

impl<const N: usize, const M: usize> TryCast<UD<N>> for UD<M>
where
    Dimension<N, M>: Narrow,
{
    type Error = ParseError;

    #[inline(always)]
    fn try_cast(self) -> Result<UD<N>, Self::Error> {
        self.0.try_cast().map(UD::new)
    }
}

impl<const N: usize, const M: usize> Cast<D<N>> for UD<M>
where
    Dimension<N, M>: Widen,
{
    #[inline(always)]
    fn cast(self) -> D<N> {
        self.0.cast()
    }
}

impl<const N: usize> Cast<D<N>> for UD<N> {
    #[inline(always)]
    fn cast(self) -> D<N> {
        self.0
    }
}

impl<const N: usize, const M: usize> TryCast<D<N>> for UD<M>
where
    Dimension<N, M>: Narrow,
{
    type Error = ParseError;

    #[inline(always)]
    fn try_cast(self) -> Result<D<N>, Self::Error> {
        self.0.try_cast()
    }
}
