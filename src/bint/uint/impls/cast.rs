use crate::{
    bint::{Int, ParseError, UInt},
    utils::const_generics::{Dimension, Narrow, Widen},
    Cast, TryCast,
};

impl<const N: usize, const M: usize> Cast<UInt<N>> for UInt<M>
where
    Dimension<N, M>: Widen,
{
    #[inline(always)]
    fn cast(self) -> UInt<N> {
        // SAFETY: `N` is always greater or equal than `M`. So we can safely cast to the
        // widest type.
        #[allow(unsafe_code)]
        unsafe {
            self._transmute()
        }
    }
}

impl<const N: usize, const M: usize> Cast<Int<N>> for UInt<M>
where
    Dimension<N, M>: Widen,
{
    #[inline(always)]
    fn cast(self) -> Int<N> {
        // SAFETY: `N` is always greater or equal than `M`. So we can safely cast to the
        // widest type.
        #[allow(unsafe_code)]
        {
            Int::from_bits(unsafe { self._transmute() })
        }
    }
}

impl<const N: usize, const M: usize> TryCast<UInt<N>> for UInt<M>
where
    Dimension<N, M>: Narrow,
{
    type Error = ParseError;

    #[inline(always)]
    fn try_cast(self) -> Result<UInt<N>, Self::Error> {
        if self.bits() <= UInt::<N>::BITS {
            // SAFETY: UInt<M> is wider (`N` < `M`) but its value fit to UInt<N>. So we can
            // safely cast to the narrow type.
            #[allow(unsafe_code)]
            {
                Ok(unsafe { self._transmute() })
            }
        } else {
            Err(ParseError::PosOverflow)
        }
    }
}

impl<const N: usize> TryCast<Int<N>> for UInt<N> {
    type Error = ParseError;

    #[inline(always)]
    fn try_cast(self) -> Result<Int<N>, Self::Error> {
        if self.bits() < Int::<N>::BITS {
            Ok(Int::from_bits(self))
        } else {
            Err(ParseError::PosOverflow)
        }
    }
}

impl<const N: usize, const M: usize> TryCast<Int<N>> for UInt<M>
where
    Dimension<N, M>: Narrow,
{
    type Error = ParseError;

    #[inline(always)]
    fn try_cast(self) -> Result<Int<N>, Self::Error> {
        if self.bits() < Int::<N>::BITS {
            // SAFETY: UInt<M> is wider (`N` < `M`) but its value fit to Int<N>. So we can
            // safely cast to the narrow type.
            #[allow(unsafe_code)]
            {
                Ok(Int::from_bits(unsafe { self._transmute() }))
            }
        } else {
            Err(ParseError::PosOverflow)
        }
    }
}
