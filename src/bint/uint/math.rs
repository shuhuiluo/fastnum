pub(crate) mod ilog;

use core::cmp::Ordering;

use crate::bint::{
    math::{basecase_div_rem, last_digit_index},
    uint::intrinsics::*,
    UInt,
};

type U<const N: usize> = UInt<N>;

#[inline]
pub const fn div_rem<const N: usize>(dividend: U<N>, divisor: U<N>) -> (U<N>, U<N>) {
    // TODO: Maybe performance optimization: ~20ns for 128 bit
    match dividend.cmp(&divisor) {
        Ordering::Less => (U::<N>::ZERO, dividend),
        Ordering::Equal => (U::<N>::ONE, U::<N>::ZERO),
        Ordering::Greater => {
            let ldi = last_digit_index(divisor.digits());
            if ldi == 0 {
                let digits = divisor.digits();
                let (div, rem) = div_rem_digit(dividend, digits[0]);
                (div, U::<N>::from_digit(rem))
            } else {
                let (div, rem) =
                    basecase_div_rem(*(dividend.digits()), *(divisor.digits()), ldi + 1);
                (U::<N>::from_digits(div), U::<N>::from_digits(rem))
            }
        }
    }
}

#[inline]
pub const fn mul_div_rem<const N: usize>(lhs: U<N>, rhs: U<N>, divisor: U<N>) -> (U<N>, U<N>) {
    let (low, high) = lhs.widening_mul(rhs);

    if high.is_zero() {
        div_rem(low, divisor)
    } else {
        div_rem_wide(low, high, divisor)
    }
}

#[inline]
pub const fn div_rem_wide<const N: usize>(low: U<N>, high: U<N>, divisor: U<N>) -> (U<N>, U<N>) {
    let mut quotient;
    let mut remainder;

    (quotient, remainder) = div_rem(low, divisor);

    let (high_quotient, high_remainder) = mul_div_rem(U::MAX.shr(1).add(U::ONE), high, divisor);

    (quotient, remainder) = overflow_remainder(quotient, remainder, high_remainder, divisor);
    (quotient, remainder) = overflow_remainder(quotient, remainder, high_remainder, divisor);

    quotient = quotient.strict_add(high_quotient).strict_add(high_quotient);

    (quotient, remainder)
}

#[inline]
pub const fn div_rem_digit<const N: usize>(value: U<N>, rhs: Digit) -> (U<N>, Digit) {
    let mut out = [0; N];

    let mut rem: Digit = 0;
    let mut i = N;

    let digits = value.digits();

    while i > 0 {
        i -= 1;
        let (q, r) = div_rem_wide_digit(digits[i], rem, rhs);
        rem = r;
        out[i] = q;
    }
    (U::from_digits(out), rem)
}

#[allow(dead_code)]
#[inline]
pub const fn long_mul<const N: usize>(lhs: U<N>, rhs: U<N>) -> (U<N>, bool) {
    let mut overflow = false;
    let mut out = [0; N];
    let mut carry: Digit;

    let lhs_digits = lhs.digits();
    let rhs_digits = rhs.digits();

    let mut i = 0;
    while i < N {
        carry = 0;
        let mut j = 0;
        while j < N {
            let index = i + j;
            if index < N {
                let (prod, c) = carrying_mul_add(lhs_digits[i], rhs_digits[j], carry, out[index]);
                out[index] = prod;
                carry = c;
            } else if lhs_digits[i] != 0 && rhs_digits[j] != 0 {
                overflow = true;
                break;
            }
            j += 1;
        }
        if carry != 0 {
            overflow = true;
        }
        i += 1;
    }
    (U::from_digits(out), overflow)
}

#[inline]
pub const fn mul_digit<const N: usize>(value: U<N>, rhs: Digit) -> (U<N>, bool) {
    let mut out = [0; N];
    let mut carry = 0;

    let mut i = 0;

    let digits = value.digits();

    while i < N {
        let (low, high) = carrying_mul(digits[i], rhs, carry);
        out[i] = low;
        carry = high;
        i += 1;
    }

    (U::from_digits(out), carry != 0)
}

#[inline]
const fn overflow_remainder<const N: usize>(
    mut quotient: U<N>,
    mut remainder: U<N>,
    add: U<N>,
    divisor: U<N>,
) -> (U<N>, U<N>) {
    let overflow;

    (remainder, overflow) = remainder.overflowing_add(add);

    if overflow {
        quotient = quotient.strict_add(UInt::ONE);
        (quotient, remainder) = overflow_remainder(
            quotient,
            remainder,
            UInt::MAX.strict_sub(divisor).strict_add(UInt::ONE),
            divisor,
        );
    }

    let q;
    (q, remainder) = div_rem(remainder, divisor);
    quotient = quotient.strict_add(q);

    (quotient, remainder)
}
