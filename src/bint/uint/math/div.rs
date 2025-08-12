use core::cmp::Ordering;

use crate::{
    bint::{intrinsics::*, math::basecase_div_rem, uint::math::utils::*, UInt},
    utils::err_msg,
};

type U<const N: usize> = UInt<N>;

#[inline(always)]
pub const fn div<const N: usize>(dividend: U<N>, divisor: U<N>) -> U<N> {
    match N {
        1 => uint64(_div_64(as_u64(dividend), as_u64(divisor))),
        2 => uint128(as_u128(dividend).div(as_u128(divisor))),
        _ => div_long(dividend, divisor),
    }
}

#[inline(always)]
pub const fn div_digit<const N: usize>(dividend: U<N>, divisor: Digit) -> U<N> {
    match N {
        1 => uint64(_div_64(as_u64(dividend), divisor)),
        2 => uint128(as_u128(dividend).div_u64(divisor)),
        _ => div_digit_long(dividend, divisor),
    }
}

#[inline(always)]
pub const fn div_rem<const N: usize>(dividend: U<N>, divisor: U<N>) -> (U<N>, U<N>) {
    match N {
        1 => uint_pair64(_div_rem_64(as_u64(dividend), as_u64(divisor))),
        2 => {
            let (q, r) = as_u128(dividend).div_rem(as_u128(divisor));
            (uint128(q), uint128(r))
        }
        _ => div_rem_long(dividend, divisor),
    }
}

#[inline(always)]
pub const fn div_rem_digit<const N: usize>(dividend: U<N>, divisor: Digit) -> (U<N>, Digit) {
    match N {
        1 => {
            let (q, r) = _div_rem_64(as_u64(dividend), divisor);
            (uint64(q), r)
        }
        2 => {
            let (q, r) = as_u128(dividend).div_rem_u64(divisor);
            (uint128(q), r)
        }
        _ => div_rem_digit_long(dividend, divisor),
    }
}

#[inline(always)]
pub const fn mul_div<const N: usize>(lhs: U<N>, rhs: U<N>, divisor: U<N>) -> U<N> {
    let (low, high) = lhs.widening_mul(rhs);

    if high.is_zero() {
        div(low, divisor)
    } else {
        div_wide(low, high, divisor)
    }
}

#[inline(always)]
pub const fn mul_div_rem<const N: usize>(lhs: U<N>, rhs: U<N>, divisor: U<N>) -> (U<N>, U<N>) {
    let (low, high) = lhs.widening_mul(rhs);

    if high.is_zero() {
        div_rem(low, divisor)
    } else {
        div_rem_wide(low, high, divisor)
    }
}

#[inline(always)]
const fn div_long<const N: usize>(lhs: UInt<N>, rhs: UInt<N>) -> UInt<N> {
    UInt(lhs.0.div(rhs.0))
}

#[inline(always)]
const fn div_digit_long<const N: usize>(value: U<N>, rhs: Digit) -> U<N> {
    // TODO
    value.div(U::from_digit(rhs))
}

#[inline]
const fn div_rem_digit_long<const N: usize>(value: U<N>, rhs: Digit) -> (U<N>, Digit) {
    let mut out = [0; N];

    let mut rem: Digit = 0;
    let mut i = N;

    let digits = value.digits();

    while i > 0 {
        i -= 1;
        let (q, r) = _div_rem_128_64(digits[i], rem, rhs);
        rem = r;
        out[i] = q;
    }
    (U::from_digits(out), rem)
}

#[inline(always)]
const fn div_rem_wide<const N: usize>(low: U<N>, high: U<N>, divisor: U<N>) -> (U<N>, U<N>) {
    debug_assert!(!high.is_zero());

    match N {
        1 => {
            let (q, r) = _U128 {
                low: as_u64(low),
                high: as_u64(high),
            }
            .div_rem_u64(as_u64(divisor));
            if q.high != 0 {
                panic!(err_msg!("attempt to divide with overflow"));
            }
            (uint64(q.low), uint64(r))
        }
        _ => div_rem_wide_long(low, high, divisor),
    }
}

#[inline(always)]
const fn div_wide<const N: usize>(low: U<N>, high: U<N>, divisor: U<N>) -> U<N> {
    debug_assert!(!high.is_zero());

    match N {
        1 => {
            let q = _U128 {
                low: as_u64(low),
                high: as_u64(high),
            }
            .div_u64(as_u64(divisor));
            if q.high != 0 {
                panic!(err_msg!("attempt to divide with overflow"));
            }
            uint64(q.low)
        }
        _ => div_wide_long(low, high, divisor),
    }
}

#[inline]
const fn div_wide_long<const N: usize>(low: U<N>, high: U<N>, divisor: U<N>) -> U<N> {
    // TODO
    let (q, _) = div_rem_wide_long(low, high, divisor);
    q
}

#[inline]
const fn div_rem_long<const N: usize>(dividend: U<N>, divisor: U<N>) -> (U<N>, U<N>) {
    match dividend.cmp(&divisor) {
        Ordering::Less => (U::<N>::ZERO, dividend),
        Ordering::Equal => (U::<N>::ONE, U::<N>::ZERO),
        Ordering::Greater => {
            let ldi = divisor.last_digit_index();
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

// TODO(!)
#[inline(always)]
const fn div_rem_wide_long<const N: usize>(low: U<N>, high: U<N>, divisor: U<N>) -> (U<N>, U<N>) {
    debug_assert!(N % 2 == 0);

    let mut d = divisor;
    let mut l = low;
    let mut h = high;

    let lz = divisor.leading_zeros();
    if lz != 0 {
        d = divisor.shl(lz);
        (l, h) = wide_shl(low, high, lz);
    }

    let (a2, a1) = split(h);
    let (a4, a3) = split(l);

    let (b2, b1) = split(d);

    let (q1, r) = div_three_halves_by_two(a1, a2, a3, b1, b2);
    let (r2, r1) = split(r);

    let (q2, s) = div_three_halves_by_two(r1, r2, a4, b1, b2);

    (join(q2, q1), s.strict_shr(lz))
}

#[inline]
const fn split<const N: usize>(value: U<N>) -> (U<N>, U<N>) {
    debug_assert!(N % 2 == 0);

    let mut low = [0; N];
    let mut high = [0; N];

    let mut i = 0;

    let digits = value.digits();

    while i < N {
        if i < N / 2 {
            low[i] = digits[i];
        } else {
            high[i - N / 2] = digits[i];
        }
        i += 1;
    }

    (U::from_digits(low), U::from_digits(high))
}

#[inline]
const fn join<const N: usize>(low: U<N>, high: U<N>) -> U<N> {
    debug_assert!(N % 2 == 0);

    let mut out = [0; N];

    let mut i = 0;

    let low_digits = low.digits();
    let high_digits = high.digits();

    while i < N {
        if i < N / 2 {
            out[i] = low_digits[i];
        } else {
            out[i] = high_digits[i - N / 2];
        }
        i += 1;
    }

    U::from_digits(out)
}

#[inline]
const fn div_three_halves_by_two<const N: usize>(
    a1: U<N>,
    a2: U<N>,
    a3: U<N>,
    b1: U<N>,
    b2: U<N>,
) -> (U<N>, U<N>) {
    if a1.eq(&b1) {
        // TODO: special case
        unimplemented!()
    }

    let (mut q, c) = join(a2, a1).div_rem(b1);
    let mut d = q.strict_mul(b2);
    let cc = join(a3, c);

    let r = if cc.ge(&d) {
        cc.strict_sub(d)
    } else {
        q = q.strict_sub(U::ONE);

        // (cc + join(b2, b1) - d)

        if join(b2, b1).ge(&d) {
            cc.strict_add(join(b2, b1).strict_sub(d))
        } else {
            d = d.strict_sub(join(b2, b1));

            if cc.ge(&d) {
                cc.strict_sub(d)
            } else {
                q = q.strict_sub(U::ONE);

                // (cc + join(b2, b1) + join(b2, b1) - d)
                // (cc + join(b2, b1) - (d - join(b2, b1)))
                // (cc + join(b2, b1) - d*)
                // (cc - (d* - join(b2, b1)))

                if join(b2, b1).ge(&d) {
                    cc.strict_add(join(b2, b1).strict_sub(d))
                } else {
                    cc.strict_sub(d.strict_sub(join(b2, b1)))
                }
            }
        }
    };

    (q, r)
}
