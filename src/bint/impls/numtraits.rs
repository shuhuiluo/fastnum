macro_rules! from_primitive_impl {
    ($Ty: ident, $sign: ident, $($from_int:ident <- $int:ident $(#$try: ident)?),*) => {
        impl<const N: usize> FromPrimitive for $Ty<N> {
            $(
                #[inline]
                fn $from_int(n: $int) -> Option<Self> {
                    from_primitive_impl!(@ $($try)? $Ty, n $from_int)
                }
            )*
        }
    };
    (@ $Ty: ident, $n: ident $from_int: ident) => {
        Some($Ty::$from_int($n))
    };
    (@ TRY $Ty: ident, $n: ident $from_int: ident) => {
        $Ty::$from_int($n).ok()
    };
}

pub(crate) use from_primitive_impl;

macro_rules! numtraits_impl {
    ($Ty: ident, $sign: ident) => {
        impl<const N: usize> Bounded for $Ty<N> {
            #[inline]
            fn min_value() -> Self {
                Self::MIN
            }

            #[inline]
            fn max_value() -> Self {
                Self::MAX
            }
        }

        impl<const N: usize> CheckedAdd for $Ty<N> {
            #[inline]
            fn checked_add(&self, rhs: &Self) -> Option<Self> {
                Self::checked_add(*self, *rhs)
            }
        }

        impl<const N: usize> CheckedDiv for $Ty<N> {
            #[inline]
            fn checked_div(&self, rhs: &Self) -> Option<Self> {
                Self::checked_div(*self, *rhs)
            }
        }

        impl<const N: usize> CheckedMul for $Ty<N> {
            #[inline]
            fn checked_mul(&self, rhs: &Self) -> Option<Self> {
                Self::checked_mul(*self, *rhs)
            }
        }

        impl<const N: usize> CheckedRem for $Ty<N> {
            #[inline]
            fn checked_rem(&self, rhs: &Self) -> Option<Self> {
                Self::checked_rem(*self, *rhs)
            }
        }

        impl<const N: usize> CheckedSub for $Ty<N> {
            #[inline]
            fn checked_sub(&self, rhs: &Self) -> Option<Self> {
                Self::checked_sub(*self, *rhs)
            }
        }

        impl<const N: usize> SaturatingAdd for $Ty<N> {
            #[inline]
            fn saturating_add(&self, rhs: &Self) -> Self {
                Self::saturating_add(*self, *rhs)
            }
        }

        impl<const N: usize> SaturatingMul for $Ty<N> {
            #[inline]
            fn saturating_mul(&self, rhs: &Self) -> Self {
                Self::saturating_mul(*self, *rhs)
            }
        }

        impl<const N: usize> SaturatingSub for $Ty<N> {
            #[inline]
            fn saturating_sub(&self, rhs: &Self) -> Self {
                Self::saturating_sub(*self, *rhs)
            }
        }

        impl<const N: usize> WrappingAdd for $Ty<N> {
            #[inline]
            fn wrapping_add(&self, rhs: &Self) -> Self {
                Self::wrapping_add(*self, *rhs)
            }
        }

        impl<const N: usize> WrappingMul for $Ty<N> {
            #[inline]
            fn wrapping_mul(&self, rhs: &Self) -> Self {
                Self::wrapping_mul(*self, *rhs)
            }
        }

        impl<const N: usize> WrappingSub for $Ty<N> {
            #[inline]
            fn wrapping_sub(&self, rhs: &Self) -> Self {
                Self::wrapping_sub(*self, *rhs)
            }
        }

        impl<const N: usize> CheckedNeg for $Ty<N> {
            #[inline]
            fn checked_neg(&self) -> Option<Self> {
                Self::checked_neg(*self)
            }
        }

        impl<const N: usize> CheckedShl for $Ty<N> {
            #[inline]
            fn checked_shl(&self, rhs: u32) -> Option<Self> {
                Self::checked_shl(*self, rhs as ExpType)
            }
        }

        impl<const N: usize> CheckedShr for $Ty<N> {
            #[inline]
            fn checked_shr(&self, rhs: u32) -> Option<Self> {
                Self::checked_shr(*self, rhs as ExpType)
            }
        }

        impl<const N: usize> CheckedEuclid for $Ty<N> {
            #[inline]
            fn checked_div_euclid(&self, rhs: &Self) -> Option<Self> {
                Self::checked_div_euclid(*self, *rhs)
            }

            #[inline]
            fn checked_rem_euclid(&self, rhs: &Self) -> Option<Self> {
                Self::checked_rem_euclid(*self, *rhs)
            }
        }

        impl<const N: usize> Euclid for $Ty<N> {
            #[inline]
            fn div_euclid(&self, rhs: &Self) -> Self {
                Self::div_euclid(*self, *rhs)
            }

            #[inline]
            fn rem_euclid(&self, rhs: &Self) -> Self {
                Self::rem_euclid(*self, *rhs)
            }
        }

        impl<const N: usize> WrappingNeg for $Ty<N> {
            #[inline]
            fn wrapping_neg(&self) -> Self {
                Self::wrapping_neg(*self)
            }
        }

        impl<const N: usize> WrappingShl for $Ty<N> {
            #[inline]
            fn wrapping_shl(&self, rhs: u32) -> Self {
                Self::wrapping_shl(*self, rhs as ExpType)
            }
        }

        impl<const N: usize> WrappingShr for $Ty<N> {
            #[inline]
            fn wrapping_shr(&self, rhs: u32) -> Self {
                Self::wrapping_shr(*self, rhs as ExpType)
            }
        }

        impl<const N: usize> Pow<ExpType> for $Ty<N> {
            type Output = Self;

            #[inline]
            fn pow(self, exp: ExpType) -> Self {
                Self::pow(self, exp)
            }
        }

        impl<const N: usize> Saturating for $Ty<N> {
            #[inline]
            fn saturating_add(self, rhs: Self) -> Self {
                Self::saturating_add(self, rhs)
            }

            #[inline]
            fn saturating_sub(self, rhs: Self) -> Self {
                Self::saturating_sub(self, rhs)
            }
        }

        impl<const N: usize> MulAdd for $Ty<N> {
            type Output = Self;

            #[inline]
            fn mul_add(self, a: Self, b: Self) -> Self {
                (self * a) + b
            }
        }

        impl<const N: usize> MulAddAssign for $Ty<N> {
            #[inline]
            fn mul_add_assign(&mut self, a: Self, b: Self) {
                *self = self.mul_add(a, b);
            }
        }

        impl<const N: usize> Num for $Ty<N> {
            type FromStrRadixErr = ParseError;

            #[inline]
            fn from_str_radix(string: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
                Self::from_str_radix(string, radix)
            }
        }

        impl<const N: usize> num_traits::NumCast for $Ty<N> {
            #[inline]
            fn from<T: ToPrimitive>(_n: T) -> Option<Self> {
                panic!(crate::utils::err_msg!(concat!("`num_traits::NumCast` trait is not supported for ", stringify!($Ty))))
            }
        }

        impl<const N: usize> One for $Ty<N> {
            #[inline]
            fn one() -> Self {
                Self::ONE
            }

            #[inline]
            fn is_one(&self) -> bool {
                self.is_one()
            }
        }

        impl<const N: usize> Zero for $Ty<N> {
            #[inline]
            fn zero() -> Self {
                Self::ZERO
            }

            #[inline]
            fn is_zero(&self) -> bool {
                self.is_zero()
            }
        }

        impl<const N: usize> Integer for $Ty<N> {
            #[inline]
            fn div_floor(&self, other: &Self) -> Self {
                Self(self.0.div_floor(other.0))
            }

            #[inline]
            fn mod_floor(&self, other: &Self) -> Self {
                Self(self.0.mod_floor(&other.0))
            }

            #[inline]
            fn gcd(&self, other: &Self) -> Self {
                 Self(self.0.gcd(&other.0))
            }

            #[inline]
            fn lcm(&self, other: &Self) -> Self {
                Self(self.0.lcm(&other.0))
            }

            #[inline]
            fn is_multiple_of(&self, other: &Self) -> bool {
                self.0.is_multiple_of(&other.0)
            }

            #[inline]
            fn is_even(&self) -> bool {
                self.0.is_even()
            }

            #[inline]
            fn is_odd(&self) -> bool {
                self.0.is_odd()
            }

            #[inline]
            fn div_rem(&self, other: &Self) -> (Self, Self) {
                Self::div_rem(*self, *other)
            }
        }

        impl<const N: usize> Roots for $Ty<N> {
            #[inline]
            fn sqrt(&self) -> Self {
                Self(self.0.sqrt())
            }

            #[inline]
            fn cbrt(&self) -> Self {
                Self(self.0.cbrt())
            }

            #[inline]
            fn nth_root(&self, n: u32) -> Self {
                Self(self.0.nth_root(n))
            }
        }

        numtraits_impl!(ToPrimitive $Ty, $sign);
        numtraits_impl!(AsPrimitive $Ty, $sign);
        numtraits_impl!(PrimInt $Ty, $sign);
    };
    (ToPrimitive $Ty: ident, $sign: ident) => {
        impl<const N: usize> ToPrimitive for $Ty<N> {
            numtraits_impl!(
                ToPrimitive $Ty, $sign,

                to_u8 -> u8,
                to_u16 -> u16,
                to_u32 -> u32,
                to_u64 -> u64,
                to_u128 -> u128,
                to_usize -> usize,

                to_i8 -> i8,
                to_i16 -> i16,
                to_i32 -> i32,
                to_i64 -> i64,
                to_i128 -> i128,
                to_isize -> isize
            );

            #[inline]
            fn to_f32(&self) -> Option<f32> {
                Some(self.as_())
            }

            #[inline]
            fn to_f64(&self) -> Option<f64> {
                Some(self.as_())
            }
        }
    };
    (ToPrimitive $Ty: ident, $sign: ident, $($name: ident -> $int: ty),*)  => {
        $(
            #[inline]
            fn $name(&self) -> Option<$int> {
                $Ty::$name(*self).ok()
            }
        )*
    };
    (AsPrimitive $Ty: ident, $sign: ident) => {
        numtraits_impl!(AsPrimitive $Ty, $sign, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
    };
    (AsPrimitive $Ty: ident, $sign: ident, $($ty: ident),*) => {
        $(
            impl<const N: usize> AsPrimitive<$ty> for $Ty<N> {
                #[inline]
                fn as_(self) -> $ty {
                    bnum::cast::CastFrom::cast_from(self.0)
                }
            }
        )*
    };
    (PrimInt $Ty: ident, $sign: ident) => {
        impl<const N: usize> PrimInt for $Ty<N> {

            #[inline]
            fn signed_shl(self, n: u32) -> Self {
                Self(self.0.signed_shl(n))
            }

            #[inline]
            fn signed_shr(self, n: u32) -> Self {
                Self(self.0.signed_shr(n))
            }

            #[inline]
            fn unsigned_shl(self, n: u32) -> Self {
                Self(self.0.unsigned_shl(n))
            }

            #[inline]
            fn unsigned_shr(self, n: u32) -> Self {
                Self(self.0.unsigned_shr(n))
            }

            numtraits_impl! {
                PrimInt $Ty, $sign,
                fn count_ones(self) -> u32;
                fn count_zeros(self) -> u32;
                fn leading_zeros(self) -> u32;
                fn trailing_zeros(self) -> u32;
                fn rotate_left(self, n: u32) -> Self;
                fn rotate_right(self, n: u32) -> Self;
                fn swap_bytes(self) -> Self;
                fn from_be(x: Self) -> Self;
                fn from_le(x: Self) -> Self;
                fn to_be(self) -> Self;
                fn to_le(self) -> Self;
                fn pow(self, exp: u32) -> Self;
                fn leading_ones(self) -> u32;
                fn trailing_ones(self) -> u32;
                fn reverse_bits(self) -> Self;
            }
        }
    };
    {PrimInt $Ty: ident, $sign: ident, $(fn $name: ident ($($arg: ident $(: $ty: ty)?), *) -> $ret: ty;)* } => {
        $(
            #[inline]
            fn $name($($arg $(: $ty)?), *) -> $ret {
                Self::$name($($arg), *)
            }
        )*
    };
}

pub(crate) use numtraits_impl;
