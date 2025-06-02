use crate::decimal::{
    dec::math::{div::div, mul::mul},
    Decimal, Sign,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn powi<const N: usize>(d: D<N>, n: i32) -> D<N> {
    if d.is_nan() {
        return d.op_invalid();
    }

    let sign = if d.is_negative() && (n % 2 != 0) {
        Sign::Minus
    } else {
        Sign::Plus
    };

    if d.is_infinite() {
        return if n > 0 {
            if sign.is_negative() ^ d.is_negative() {
                d.neg()
            } else {
                d
            }
        } else if n == 0 {
            D::ONE
        } else {
            D::ZERO.set_sign(sign)
        };
    }

    if n == 0 {
        return if d.is_zero() {
            d.signaling_nan()
        } else {
            D::ONE
        };
    }

    if d.is_zero() {
        return if n < 0 {
            D::INFINITY.set_ctx(d.context()).set_sign(sign)
        } else {
            D::ZERO.set_ctx(d.context()).set_sign(sign)
        };
    }

    if n < 0 {
        div(D::ONE, powi_integral(d, n.overflowing_neg().0 as u32))
    } else {
        powi_integral(d, n as u32)
    }
}

#[inline]
const fn powi_integral<const N: usize>(mut d: D<N>, mut n: u32) -> D<N> {
    debug_assert!(n > 0);

    let mut result = D::ONE;
    while n > 1 {
        if n & 1 == 1 {
            result = mul(result, d);
        }
        d = mul(d, d);
        n >>= 1;
    }

    mul(result, d)
}
