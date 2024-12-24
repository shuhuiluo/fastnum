use num_traits::FloatConst;

use crate::decimal::Decimal;

#[allow(non_snake_case)]
impl<const N: usize> FloatConst for Decimal<N> {
    #[inline]
    fn E() -> Self {
        Self::E
    }

    #[inline]
    fn FRAC_1_PI() -> Self {
        Self::FRAC_1_PI
    }

    #[inline]
    fn FRAC_1_SQRT_2() -> Self {
        Self::FRAC_1_SQRT_2
    }

    #[inline]
    fn FRAC_2_PI() -> Self {
        Self::FRAC_2_PI
    }

    #[inline]
    fn FRAC_2_SQRT_PI() -> Self {
        Self::FRAC_2_SQRT_PI
    }

    #[inline]
    fn FRAC_PI_2() -> Self {
        Self::FRAC_PI_2
    }

    #[inline]
    fn FRAC_PI_3() -> Self {
        Self::FRAC_PI_3
    }

    #[inline]
    fn FRAC_PI_4() -> Self {
        Self::FRAC_PI_4
    }

    #[inline]
    fn FRAC_PI_6() -> Self {
        Self::FRAC_PI_6
    }

    #[inline]
    fn FRAC_PI_8() -> Self {
        Self::FRAC_PI_8
    }

    #[inline]
    fn LN_10() -> Self {
        Self::LN_10
    }

    #[inline]
    fn LN_2() -> Self {
        Self::LN_2
    }

    #[inline]
    fn LOG10_E() -> Self {
        Self::LOG10_E
    }

    #[inline]
    fn LOG2_E() -> Self {
        Self::LOG2_E
    }

    #[inline]
    fn PI() -> Self {
        Self::PI
    }

    #[inline]
    fn SQRT_2() -> Self {
        Self::SQRT_2
    }

    #[inline]
    fn TAU() -> Self {
        Self::TAU
    }

    #[inline]
    fn LOG10_2() -> Self {
        Self::LOG10_2
    }

    #[inline]
    fn LOG2_10() -> Self {
        Self::LOG2_10
    }
}
