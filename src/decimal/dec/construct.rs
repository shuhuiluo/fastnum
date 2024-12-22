use crate::{
    decimal::{
        dec::{
            math::utils::{clength, overflow, underflow},
            ControlBlock,
        },
        Decimal, Signal,
    },
    int::UInt,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn construct<const N: usize>(
    mut digits: UInt<N>,
    mut exp: i32,
    mut cb: ControlBlock,
) -> D<N> {
    // Overflow exp > Emax
    if exp > D::<N>::E_MAX + (D::<N>::MAX_CLENGTH - 1) {
        return overflow(cb);
    }

    // Underflow exp < Emin
    if exp < D::<N>::E_MIN {
        return underflow(cb);
    }

    let c_length = clength(digits);

    if exp <= D::<N>::E_MAX {
        if exp < D::<N>::E_MIN + (c_length - 1) {
            cb = cb.raise_signal(Signal::OP_SUBNORMAL);
        }

        return D::new(digits, -exp as i16, cb);
    }

    cb = cb
        .raise_signal(Signal::OP_CLAMPED)
        .raise_signal(Signal::OP_ROUNDED);

    while exp > D::<N>::E_MAX {
        if digits.gt(&D::<N>::COEFF_MEDIUM) {
            return D::INFINITY.with_cb(cb.raise_signal(Signal::OP_OVERFLOW));
        } else {
            digits = digits.strict_mul(UInt::<N>::TEN);
            exp -= 1;
        }
    }

    D::new(digits, -exp as i16, cb)
}
