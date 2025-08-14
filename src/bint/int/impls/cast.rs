use crate::{
    bint::{Int, ParseError, UInt},
    utils::const_generics::{Dimension, Narrow, Widen},
    Cast, TryCast,
};

impl<const N: usize, const M: usize> Cast<Int<N>> for Int<M>
where
    Dimension<N, M>: Widen,
{
    #[inline(always)]
    fn cast(self) -> Int<N> {
        // SAFETY: `N` is always greater or equal than `M`. So we can safely cast to the
        // widest type.
        #[allow(unsafe_code)]
        unsafe {
            self._transmute()
        }
    }
}

impl<const N: usize, const M: usize> TryCast<Int<N>> for Int<M>
where
    Dimension<N, M>: Narrow,
{
    type Error = ParseError;

    #[inline(always)]
    fn try_cast(self) -> Result<Int<N>, Self::Error> {
        if self.bits() <= Int::<N>::BITS {
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

impl<const N: usize, const M: usize> TryCast<UInt<N>> for Int<M> {
    type Error = ParseError;

    #[inline(always)]
    fn try_cast(self) -> Result<UInt<N>, Self::Error> {
        if self.is_negative() {
            Err(ParseError::Signed)
        } else if self.bits() <= Int::<N>::BITS {
            // SAFETY: UInt<M> is wider (`N` < `M`) but its value fit to UInt<N>. So we can
            // safely cast to the narrow type.
            #[allow(unsafe_code)]
            {
                Ok(unsafe { self._transmute() }.to_bits())
            }
        } else {
            Err(ParseError::PosOverflow)
        }
    }
}
