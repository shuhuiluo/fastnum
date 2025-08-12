use core::cmp::Ordering;

use crate::{
    bint::{intrinsics::ExpType, UInt},
    decimal::{dec::scale::reduce, Decimal},
};

type D<const N: usize> = Decimal<N>;

#[inline(always)]
pub(crate) const fn eq<const N: usize>(lhs: &D<N>, rhs: &D<N>) -> bool {
    if lhs.is_nan() || rhs.is_nan() {
        false
    } else if lhs.is_zero() && rhs.is_zero() {
        true
    } else {
        eq_not_nan(lhs, rhs)
    }
}

#[inline(always)]
pub(crate) const fn ne<const N: usize>(lhs: &D<N>, rhs: &D<N>) -> bool {
    if lhs.is_nan() || rhs.is_nan() {
        true
    } else if lhs.is_zero() && rhs.is_zero() {
        false
    } else {
        !eq_not_nan(lhs, rhs)
    }
}

#[inline(always)]
pub(crate) const fn cmp<const N: usize>(lhs: &D<N>, rhs: &D<N>) -> Ordering {
    match (lhs.is_negative(), rhs.is_negative()) {
        (false, true) => Ordering::Greater,
        (true, false) => Ordering::Less,
        (true, true) => cmp_magnitude(lhs, rhs).reverse(),
        (false, false) => cmp_magnitude(lhs, rhs),
    }
}

#[inline(never)]
const fn eq_not_nan<const N: usize>(lhs: &D<N>, rhs: &D<N>) -> bool {
    if lhs.is_negative() ^ rhs.is_negative() {
        return false;
    }

    match (lhs.is_infinite(), rhs.is_infinite()) {
        (false, true) => {
            return false;
        }
        (true, false) => {
            return false;
        }
        (true, true) => {
            return true;
        }
        (false, false) => {}
    }

    let lhs = reduce(*lhs);
    let rhs = reduce(*rhs);

    match (lhs.has_extra_precision(), rhs.has_extra_precision()) {
        (true, true) => eq_rounded(&lhs, &rhs) && lhs.cb.eq_extra_precision(&rhs.cb),
        (true, false) => false,
        (false, true) => false,
        (false, false) => eq_rounded(&lhs, &rhs),
    }
}

#[inline(always)]
const fn eq_rounded<const N: usize>(lhs: &D<N>, rhs: &D<N>) -> bool {
    (lhs.cb.get_scale() == rhs.cb.get_scale()) && (lhs.digits.eq(&rhs.digits))
}

#[inline(never)]
const fn cmp_magnitude<const N: usize>(lhs: &D<N>, rhs: &D<N>) -> Ordering {
    match (lhs.is_nan(), rhs.is_nan()) {
        (true, true) => {
            return Ordering::Equal;
        }
        (true, false) => {
            return Ordering::Greater;
        }
        (false, true) => {
            return Ordering::Less;
        }
        (false, false) => {}
    }

    match (lhs.is_infinite(), rhs.is_infinite()) {
        (false, true) => {
            return Ordering::Less;
        }
        (true, false) => {
            return Ordering::Greater;
        }
        (true, true) => {
            return Ordering::Equal;
        }
        (false, false) => {}
    }

    match (lhs.is_zero(), rhs.is_zero()) {
        (true, true) => {
            return Ordering::Equal;
        }
        (true, false) => {
            return Ordering::Less;
        }
        (false, true) => {
            return Ordering::Greater;
        }
        (_, _) => {}
    }

    let lhs_power = lhs.decimal_power();
    let rhs_power = rhs.decimal_power();

    if lhs_power == rhs_power {
        let lhs = reduce(*lhs);
        let rhs = reduce(*rhs);

        match cmp_rounded(&lhs, &rhs) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => match (lhs.has_extra_precision(), rhs.has_extra_precision()) {
                (true, true) => lhs.cb.cmp_extra_precision(&rhs.cb),
                (true, false) => Ordering::Greater,
                (false, true) => Ordering::Less,
                (false, false) => Ordering::Equal,
            },
            Ordering::Greater => Ordering::Greater,
        }
    } else if lhs_power < rhs_power {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

#[inline(always)]
const fn cmp_rounded<const N: usize>(a: &D<N>, b: &D<N>) -> Ordering {
    if a.cb.get_scale() == b.cb.get_scale() {
        return a.digits.cmp(&b.digits);
    }

    let a_exp = a.decimal_power();
    let b_exp = b.decimal_power();

    if a_exp == b_exp {
        if a.cb.get_scale() > b.cb.get_scale() {
            let power = (a.cb.get_scale() - b.cb.get_scale()) as ExpType;

            if power > b.digits.remaining_decimal_digits() {
                return Ordering::Less;
            }

            // SAFETY: `power` is less than or equal to
            // `b.digits.remaining_decimal_digits()`
            #[allow(unsafe_code)]
            {
                let value = unsafe { b.digits.unchecked_mul(UInt::power_of_ten(power)) };
                a.digits.cmp(&value)
            }
        } else {
            let power = (b.cb.get_scale() - a.cb.get_scale()) as ExpType;

            if power > a.digits.remaining_decimal_digits() {
                return Ordering::Greater;
            }

            // SAFETY: `power` is less than or equal to
            // `b.digits.remaining_decimal_digits()`
            #[allow(unsafe_code)]
            {
                let value = unsafe { a.digits.unchecked_mul(UInt::power_of_ten(power)) };
                value.cmp(&b.digits)
            }
        }
    } else if a_exp > b_exp {
        Ordering::Greater
    } else {
        Ordering::Less
    }
}
