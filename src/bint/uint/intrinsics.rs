mod powers;

use powers::*;

use crate::bint::UInt;

pub use crate::bint::intrinsics::*;

pub struct Intrinsics<const N: usize>;

impl<const N: usize> Intrinsics<N> {
    const POWERS_OF_FIVE: PowersOf5<N> = PowersOf5::new();
    const POWERS_OF_TEN: PowersOf10<N> = PowersOf10::new();

    pub(crate) const MAX_POWER_OF_TEN: u32 = PowersOf10::<N>::MAX_POWER;
    pub(crate) const MAX_POWER_OF_FIVE: u32 = PowersOf5::<N>::MAX_POWER;

    #[inline(always)]
    pub(crate) const fn checked_power_of_five(power: u32) -> Option<UInt<N>> {
        Self::POWERS_OF_FIVE.checked_power(power)
    }

    #[allow(dead_code)]
    #[inline(always)]
    pub(crate) const fn unchecked_power_of_five(power: u32) -> UInt<N> {
        Self::POWERS_OF_FIVE.power(power)
    }

    #[inline(always)]
    pub(crate) const fn checked_power_of_ten(power: u32) -> Option<UInt<N>> {
        Self::POWERS_OF_TEN.checked_power(power)
    }

    #[inline(always)]
    pub(crate) const fn unchecked_power_of_ten(power: u32) -> UInt<N> {
        Self::POWERS_OF_TEN.power(power)
    }

    #[allow(dead_code)]
    #[inline(always)]
    pub(crate) const fn checked_max_reduced_by_power_of_five(power: u32) -> Option<UInt<N>> {
        Self::POWERS_OF_FIVE.checked_max_reduced_by_power(power)
    }

    #[allow(dead_code)]
    #[inline(always)]
    pub(crate) const fn unchecked_max_reduced_by_power_of_five(power: u32) -> UInt<N> {
        Self::POWERS_OF_FIVE.max_reduced_by_power(power)
    }

    #[allow(dead_code)]
    #[inline(always)]
    pub(crate) const fn checked_max_reduced_by_power_of_ten(power: u32) -> Option<UInt<N>> {
        Self::POWERS_OF_TEN.checked_max_reduced_by_power(power)
    }

    #[inline(always)]
    pub(crate) const fn unchecked_max_reduced_by_power_of_ten(power: u32) -> UInt<N> {
        Self::POWERS_OF_TEN.max_reduced_by_power(power)
    }
}

#[cfg(debug_assertions)]
mod __asserts {
    use super::*;

    use crate::{
        bint::UInt,
        utils::{const_assert, const_assert_eq},
    };

    const_assert!(bnum::BUint::<1>::MAX.ilog10() == 19);

    const_assert_eq!(Intrinsics::unchecked_power_of_ten(0), &UInt::<1>::ONE);
    const_assert_eq!(Intrinsics::unchecked_power_of_ten(1), &UInt::<1>::TEN);
    const_assert_eq!(
        Intrinsics::unchecked_power_of_ten(19),
        &UInt::<1>::ONE.mul_digit(DIGIT_POWERS_10[19])
    );
}
