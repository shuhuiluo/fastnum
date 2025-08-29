use core::cmp::Ordering;

use crate::decimal::{
    dec::{
        construct::construct,
        intrinsics::Intrinsics,
        math::{add::add, consts::Consts, div::div, mul::mul, sqrt::sqrt, sub::sub},
        parse::from_u32,
        ExtraPrecision,
    },
    Decimal,
};

type D<const N: usize> = Decimal<N>;

#[inline(never)]
pub(crate) const fn ln<const N: usize>(x: D<N>) -> D<N> {
    if x.is_nan() {
        return x.op_invalid();
    }

    if x.is_zero() {
        return D::NEG_INFINITY.op_invalid().set_ctx(x.context());
    }

    if x.is_negative() {
        return x.signaling_nan();
    }

    if x.is_infinite() {
        return x;
    }

    if x.is_one() {
        return D::ZERO.set_ctx(x.context());
    }

    decimal_ln(x)
}

#[inline(always)]
pub(crate) const fn ln_1p<const N: usize>(x: D<N>) -> D<N> {
    ln(add(D::ONE, x))
}

#[inline]
const fn decimal_ln<const N: usize>(x: D<N>) -> D<N> {
    add(
        argument_reduction(construct(
            x.digits,
            0,
            x.sign(),
            x.signals(),
            x.context(),
            ExtraPrecision::new(),
        )),
        mul(D::LN_10, D::from_i32(x.cb.get_exponent())),
    )
}

#[inline]
const fn argument_reduction<const N: usize>(x: D<N>) -> D<N> {
    match x.cmp(&D::TWO) {
        Ordering::Less => taylor_series(x),
        Ordering::Equal => Consts::LN_2,
        Ordering::Greater => mul(D::TWO, argument_reduction(sqrt(x))),
    }
}

#[inline]
const fn taylor_series<const N: usize>(x: D<N>) -> D<N> {
    let mut result_next;
    let mut result = div(sub(x, D::ONE), add(x, D::ONE));
    let base = mul(result, result);

    let mut item = mul(result, base);

    let mut i = 3;

    while i < Intrinsics::<N>::SERIES_MAX_ITERATIONS * 2 {
        result_next = add(result, div(item, from_u32(i)));

        if result.eq(&result_next) {
            break;
        }

        item = mul(item, base);

        result = result_next;
        i += 2;
    }

    mul(result, D::TWO)
}
