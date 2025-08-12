macro_rules! num_impl {
    ($Ty: ident, $sign: ident) => {
        impl<const N: usize> $Ty<N> {
            #[doc = doc::num::count_ones!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn count_ones(self) -> ExpType {
                self.0.count_ones()
            }

            #[doc = doc::num::count_zeros!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn count_zeros(self) -> ExpType {
                self.0.count_zeros()
            }

            #[doc = doc::num::leading_zeros!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn leading_zeros(self) -> ExpType {
                self.0.leading_zeros()
            }

            #[doc = doc::num::trailing_zeros!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn trailing_zeros(self) -> ExpType {
                self.0.trailing_zeros()
            }

            #[doc = doc::num::leading_ones!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn leading_ones(self) -> ExpType {
                self.0.leading_ones()
            }

            #[doc = doc::num::trailing_ones!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn trailing_ones(self) -> ExpType {
                self.0.trailing_ones()
            }

            #[doc = doc::num::rotate_left!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn rotate_left(self, n: ExpType) -> Self {
                Self(self.0.rotate_left(n))
            }

            #[doc = doc::num::rotate_right!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn rotate_right(self, n: ExpType) -> Self {
                Self(self.0.rotate_right(n))
            }

            #[doc = doc::num::swap_bytes!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn swap_bytes(self) -> Self {
                Self(self.0.swap_bytes())
            }

            #[doc = doc::num::reverse_bits!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn reverse_bits(self) -> Self {
                Self(self.0.reverse_bits())
            }

            #[doc = doc::num::pow!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn pow(self, exp: ExpType) -> Self {
                Self(self.0.pow(exp))
            }

            #[doc = doc::num::add!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn add(self, rhs: Self) -> Self {
                Self(self.0.add(rhs.0))
            }

            #[doc = doc::num::shl!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn shl(self, rhs: ExpType) -> Self {
                Self(self.0.shl(rhs))
            }

            #[doc = doc::num::shr!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn shr(self, rhs: ExpType) -> Self {
                Self(self.0.shr(rhs))
            }

            #[doc = doc::num::sub!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn sub(self, rhs: Self) -> Self {
                Self(self.0.sub(rhs.0))
            }

            #[doc = doc::num::rem!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn rem(self, rhs: Self) -> Self {
                Self(self.0.rem(rhs.0))
            }

            #[doc = doc::num::div_euclid!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn div_euclid(self, rhs: Self) -> Self {
                Self(self.0.div_euclid(rhs.0))
            }

            #[doc = doc::num::rem_euclid!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn rem_euclid(self, rhs: Self) -> Self {
                Self(self.0.rem_euclid(rhs.0))
            }

            #[doc = doc::num::is_power_of_two!($sign 256)]
            #[must_use]
            #[inline(always)]
            pub const fn is_power_of_two(self) -> bool {
                self.0.is_power_of_two()
            }

            #[doc = doc::num::midpoint!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn midpoint(self, rhs: Self) -> Self {
                Self(self.0.midpoint(rhs.0))
            }

            #[doc = doc::num::ilog2!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn ilog2(self) -> ExpType {
                self.0.ilog2()
            }

            #[doc = doc::num::ilog!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn ilog(self, base: Self) -> ExpType {
                self.0.ilog(base.0)
            }

            #[doc = doc::num::ilog10!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn ilog10(self) -> u32 {
                self.checked_ilog10().expect(crate::utils::err_msg!(
                    "argument of integer logarithm must be positive"
                ))
            }

            #[doc = doc::num::next_multiple_of!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn next_multiple_of(self, rhs: Self) -> Self {
                Self(self.0.next_multiple_of(rhs.0))
            }

            #[doc = doc::num::div_floor!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn div_floor(self, rhs: Self) -> Self {
                Self(self.0.div_floor(rhs.0))
            }

            #[doc = doc::num::div_ceil!($sign 256)]
            #[must_use = doc::must_use_op!()]
            #[inline(always)]
            pub const fn div_ceil(self, rhs: Self) -> Self {
                Self(self.0.div_ceil(rhs.0))
            }

            #[doc = doc::num::bits!($sign 256)]
            #[must_use]
            #[inline(always)]
            pub const fn bits(&self) -> ExpType {
                self.0.bits()
            }

            #[doc = doc::num::bit!($sign 256)]
            #[must_use]
            #[inline(always)]
            pub const fn bit(&self, b: ExpType) -> bool {
                self.0.bit(b)
            }
        }
    };
}

pub(crate) use num_impl;
