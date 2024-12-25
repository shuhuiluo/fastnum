use crate::int::UInt;

pub(crate) struct Intrinsics<const N: usize>;

/// E<sub>max</sub> = 32'768
pub(crate) const E_MAX: i32 = -(i16::MIN as i32);

/// E<sub>min</sub> = -32'767
pub(crate) const E_MIN: i32 = -(i16::MAX as i32);

impl<const N: usize> Intrinsics<N> {
    pub(crate) const COEFF_MAX: UInt<N> = UInt::<N>::MAX;

    pub(crate) const COEFF_MEDIUM: UInt<N> = Self::COEFF_MAX.div(UInt::<N>::TEN);

    pub(crate) const COEFF_MEDIUM_PLUS_ONE: UInt<N> = Self::COEFF_MEDIUM.strict_add(UInt::ONE);

    /// Max length of the _coefficient_ in decimal digits.
    pub(crate) const MAX_CLENGTH: i32 = clength(UInt::<N>::MAX);
}

#[inline(always)]
pub(crate) const fn clength<const N: usize>(coeff: UInt<N>) -> i32 {
    if coeff.is_zero() {
        return 1;
    }

    coeff.ilog10() as i32 + 1
}
