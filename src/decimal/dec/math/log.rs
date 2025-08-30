use crate::decimal::{
    dec::{
        intrinsics::Intrinsics,
        math::{add::add, consts::Consts, div::div, mul::mul, sqrt::sqrt, sub::sub},
        parse::from_u32,
    },
    Decimal,
};

type D<const N: usize> = Decimal<N>;

#[derive(Copy, Clone)]
enum Base<const N: usize> {
    TWO,
    E,
    TEN,
    Arbitrary(D<N>),
}

#[inline(always)]
pub(crate) const fn ln<const N: usize>(x: D<N>) -> D<N> {
    decimal_log(x, Base::E)
}

#[inline(always)]
pub(crate) const fn ln_1p<const N: usize>(x: D<N>) -> D<N> {
    decimal_log(add(D::ONE, x), Base::E)
}

#[inline(always)]
pub(crate) const fn log2<const N: usize>(x: D<N>) -> D<N> {
    decimal_log(x, Base::TWO)
}

#[inline(always)]
pub(crate) const fn log10<const N: usize>(x: D<N>) -> D<N> {
    decimal_log(x, Base::TEN)
}

#[inline(always)]
pub(crate) const fn log<const N: usize>(x: D<N>, base: D<N>) -> D<N> {
    if base.is_zero() || base.is_one() || base.cb.is_special() || base.cb.is_negative() {
        return x.signaling_nan();
    }

    let base = if base.eq(&D::TWO) {
        Base::TWO
    } else if base.eq(&D::TEN) {
        Base::TEN
    } else if base.eq(&D::E) {
        Base::E
    } else {
        Base::Arbitrary(base)
    };

    decimal_log(x, base)
}

#[inline(never)]
const fn decimal_log<const N: usize>(mut x: D<N>, base: Base<N>) -> D<N> {
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

    let exp = x.cb.get_exponent();
    let extra_digits = x.cb.take_extra_precision_decimal();
    x.cb.set_scale(0);

    add(
        add(decimal_log_exp(exp, base), decimal_log_integral(x, base)),
        decimal_log(add(extra_digits, D::ONE).round_extra_precision(), base),
    )
}

#[inline(always)]
const fn decimal_log_integral<const N: usize>(mut x: D<N>, base: Base<N>) -> D<N> {
    debug_assert!(x.cb.get_exponent() == 0);
    debug_assert!(!x.cb.has_extra_precision());

    match base {
        Base::TWO => {
            let power = x.digits.trailing_zeros();

            if power != 0 {
                x.digits = x.digits.strict_shr(power);
            }

            add(D::from_u32(power), div(ln_(x), Consts::LN_2))
        }
        Base::E => ln_(x),
        Base::TEN => {
            let mut power = 0;
            loop {
                let (q, r) = x.digits.div_rem_digit(10);
                if r == 0 {
                    x.digits = q;
                    power += 1;
                } else {
                    break;
                }
            }

            add(D::from_u32(power), div(ln_(x), Consts::LN_10))
        }
        Base::Arbitrary(base) => div(ln_(x), ln_(base)),
    }
}

#[inline(always)]
const fn decimal_log_exp<const N: usize>(exp: i32, base: Base<N>) -> D<N> {
    let exp_part = D::from_i32(exp);

    match base {
        Base::TWO => mul(Consts::LOG2_10, exp_part),
        Base::E => mul(Consts::LN_10, exp_part),
        Base::TEN => exp_part,
        Base::Arbitrary(base) => {
            let log_base_10 = decimal_log_integral(D::TEN, Base::Arbitrary(base));
            mul(log_base_10, exp_part)
        }
    }
}

#[inline(never)]
const fn ln_<const N: usize>(x: D<N>) -> D<N> {
    if x.is_one() {
        D::ZERO.set_ctx(x.context())
    } else if x.eq(&Consts::E) {
        D::ONE.set_ctx(x.context())
    } else if x.eq(&D::TWO) {
        Consts::LN_2.set_ctx(x.context())
    } else if x.eq(&D::TEN) {
        Consts::LN_10.set_ctx(x.context())
    } else if x.gt(&D::TWO) {
        mul(D::TWO.set_ctx(x.context()), ln_(sqrt(x)))
    } else {
        taylor_series(x).set_ctx(x.context())
    }
}

#[inline(always)]
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
