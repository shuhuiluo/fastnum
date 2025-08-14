use crate::{
    bint,
    bint::UInt,
    decimal::{Decimal, ParseError, Sign, UnsignedDecimal},
    utils::const_generics::{Dimension, Narrow, Widen},
    Cast, TryCast,
};

type D<const N: usize> = Decimal<N>;
type UD<const N: usize> = UnsignedDecimal<N>;

impl<const N: usize, const M: usize> Cast<D<N>> for D<M>
where
    Dimension<N, M>: Widen,
{
    #[inline(always)]
    fn cast(self) -> D<N> {
        D::new(self.digits.cast(), self.cb)
    }
}

impl<const N: usize, const M: usize> TryCast<D<N>> for D<M>
where
    Dimension<N, M>: Narrow,
{
    type Error = ParseError;

    #[inline(always)]
    fn try_cast(self) -> Result<D<N>, Self::Error> {
        <UInt<M> as TryCast<UInt<N>>>::try_cast(self.digits)
            .map(|digits| D::new(digits, self.cb))
            .map_err(|e| map_error(e, self.sign()))
    }
}

impl<const N: usize, const M: usize> TryCast<UD<N>> for D<M> {
    type Error = ParseError;

    #[inline(always)]
    fn try_cast(self) -> Result<UD<N>, Self::Error> {
        if self.is_negative() {
            Err(ParseError::Signed)
        } else {
            debug_assert!(self.is_positive());

            if N >= M || self.digits().bits() <= UInt::<N>::BITS {
                // SAFETY:
                // - N >= M, so it's safe to transmute.
                // - N < M, but higher bits are zero, so it's safe to transmute.
                #[allow(unsafe_code)]
                {
                    let d = unsafe { self._transmute() };
                    Ok(UD::new(d))
                }
            } else {
                Err(map_overflow(self.sign()))
            }
        }
    }
}

#[inline(always)]
fn map_overflow(sign: Sign) -> ParseError {
    match sign {
        Sign::Plus => ParseError::PosOverflow,
        Sign::Minus => ParseError::NegOverflow,
    }
}

#[inline(always)]
fn map_error(e: bint::ParseError, sign: Sign) -> ParseError {
    match e {
        bint::ParseError::Empty => ParseError::Empty,
        bint::ParseError::InvalidDigit => ParseError::InvalidLiteral,
        bint::ParseError::PosOverflow => map_overflow(sign),
        bint::ParseError::NegOverflow => ParseError::NegOverflow,
        bint::ParseError::Zero => ParseError::Unknown,
        bint::ParseError::Signed => ParseError::Signed,
        bint::ParseError::InvalidRadix => ParseError::InvalidRadix,
        bint::ParseError::Unknown => ParseError::Unknown,
    }
}
