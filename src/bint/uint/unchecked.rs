use crate::bint::{
    intrinsics::ExpType,
    uint::{math, powers},
    UInt,
};

impl<const N: usize> UInt<N> {
    #[allow(unsafe_code)]
    #[inline(always)]
    pub(crate) const unsafe fn unchecked_add_digit(self, digit: u64) -> Self {
        math::add::unchecked_add_digit(self, digit)
    }

    #[allow(unsafe_code)]
    #[inline(always)]
    pub(crate) const unsafe fn unchecked_mul(self, other: Self) -> Self {
        math::mul::unchecked_mul(self, other)
    }

    #[allow(unsafe_code)]
    #[inline(always)]
    pub(crate) const unsafe fn unchecked_mul_digit(self, digit: u64) -> Self {
        math::mul::unchecked_mul_digit(self, digit)
    }

    #[inline(always)]
    pub(crate) const fn unchecked_power_of_ten(power: ExpType) -> Self {
        powers::unchecked_power_of_ten(power)
    }

    #[allow(dead_code)]
    #[inline(always)]
    pub(crate) const fn unchecked_power_of_five(power: ExpType) -> Self {
        powers::unchecked_power_of_five(power)
    }
}
