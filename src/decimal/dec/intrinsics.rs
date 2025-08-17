use crate::bint::{intrinsics::ExpType, UInt};

pub(crate) struct Intrinsics<const N: usize>;

/// _E<sub>limit</sub> = 32'768_
pub(crate) const E_LIMIT: i32 = -(i16::MIN as i32);

/// _E<sub>min</sub> = -32'767_
pub(crate) const E_MIN: i32 = -(i16::MAX as i32);

impl<const N: usize> Intrinsics<N> {
    /// Max length of the _coefficient_ in decimal digits.
    pub(crate) const MAX_CLENGTH: ExpType = UInt::<N>::MAX.decimal_digits();

    /// _E<sub>max</sub> = E<sub>limit</sub> + (C<sub>length</sub> – 1)_
    pub(crate) const E_MAX: i32 = E_LIMIT + (Self::MAX_CLENGTH as i32 - 1);

    /// _E<sub>subnormal</sub> = E<sub>min</sub> + (C<sub>length</sub> – 1)_
    pub(crate) const E_SUBNORMAL: i32 = E_MIN + (Self::MAX_CLENGTH as i32 - 1);

    pub(crate) const SERIES_MAX_ITERATIONS: u32 = Self::MAX_CLENGTH * 6;
}
