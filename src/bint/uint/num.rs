use crate::bint::{
    doc,
    intrinsics::last_digit_index,
    num::num_impl,
    uint::{
        intrinsics,
        intrinsics::{Digit, Digits, ExpType},
        math, powers,
    },
    Int, UInt,
};

num_impl!(UInt, U);

impl<const N: usize> UInt<N> {
    #[doc = doc::num::mul!(U 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn mul(self, rhs: Self) -> Self {
        #[cfg(debug_assertions)]
        return self.strict_mul(rhs);

        #[cfg(not(debug_assertions))]
        self.wrapping_mul(rhs)
    }

    #[doc = doc::num::div!(U 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn div(self, rhs: Self) -> Self {
        math::div::div(self, rhs)
    }

    #[doc = doc::num::neg!(U 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn neg(self) -> Self {
        #[cfg(debug_assertions)]
        return self.strict_neg();

        #[cfg(not(debug_assertions))]
        self.wrapping_neg()
    }

    #[doc = doc::num::digits!(U 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn digits(&self) -> &Digits<N> {
        self.0.digits()
    }

    #[doc = doc::num::digits_mut!(U 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub fn digits_mut(&mut self) -> &mut Digits<N> {
        self.0.digits_mut()
    }

    #[doc = doc::num::cast_signed!(256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn cast_signed(self) -> Int<N> {
        Int(self.0.cast_signed())
    }

    #[doc = doc::num::div_digit!(U 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn div_digit(self, rhs: Digit) -> Self {
        math::div::div_digit(self, rhs)
    }

    #[doc = doc::num::div_rem!(U 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn div_rem(self, rhs: Self) -> (Self, Self) {
        math::div::div_rem(self, rhs)
    }

    #[doc = doc::num::div_rem_digit!(U 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn div_rem_digit(self, rhs: Digit) -> (Self, Digit) {
        math::div::div_rem_digit(self, rhs)
    }

    #[doc = doc::num::mul_div_rem!(U 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn mul_div_rem(self, rhs: Self, divisor: Self) -> (Self, Self) {
        math::div::mul_div_rem(self, rhs, divisor)
    }

    #[doc = doc::num::mul_div_rem!(U 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn mul_div(self, rhs: Self, divisor: Self) -> Self {
        math::div::mul_div(self, rhs, divisor)
    }

    #[doc = doc::num::from_digits!(U 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn from_digits(digits: Digits<N>) -> Self {
        Self(bnum::BUint::from_digits(digits))
    }

    #[doc = doc::num::from_digit!(U 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn from_digit(digit: Digit) -> Self {
        Self(bnum::BUint::from_digit(digit))
    }

    #[doc = doc::num::power_of_two!(U 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn power_of_two(power: ExpType) -> Self {
        Self(bnum::BUint::power_of_two(power))
    }

    #[doc = doc::num::power_of_five!(U 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn power_of_five(power: ExpType) -> Self {
        Self::strict_power_of_five(power)
    }

    #[doc = doc::num::power_of_ten!(U 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn power_of_ten(power: ExpType) -> Self {
        Self::strict_power_of_ten(power)
    }

    #[doc = doc::num::abs_diff!(U 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn abs_diff(self, other: Self) -> Self {
        Self(self.0.abs_diff(other.0))
    }

    #[doc = doc::num::mul_digit!(U 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn mul_digit(self, digit: u64) -> Self {
        #[cfg(debug_assertions)]
        return self.strict_mul_digit(digit);

        #[cfg(not(debug_assertions))]
        self.wrapping_mul_digit(digit)
    }

    #[doc = doc::num::decimal_digits!(U 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn decimal_digits(&self) -> ExpType {
        powers::decimal_digits(self)
    }

    #[doc = doc::num::remaining_decimal_digits!(U 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn remaining_decimal_digits(&self) -> ExpType {
        powers::remaining_decimal_digits(self)
    }

    #[doc = doc::num::can_scaled_by_power_of_ten!(U 256)]
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn can_scaled_by_power_of_ten(&self, power: ExpType) -> bool {
        powers::can_scaled_by_power_of_ten(self, power)
    }

    #[inline(always)]
    pub(crate) const fn last_digit_index(&self) -> usize {
        last_digit_index(self.digits())
    }

    #[allow(unsafe_code)]
    #[inline(always)]
    pub(crate) const unsafe fn _transmute<const M: usize>(self) -> UInt<M> {
        intrinsics::transmute(self)
    }
}
