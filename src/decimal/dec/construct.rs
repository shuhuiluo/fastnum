use crate::{
    decimal::{
        dec::{
            intrinsics::{clength, Intrinsics, E_LIMIT, E_MIN},
            math::utils::{overflow, underflow},
            ControlBlock,
        },
        Decimal, Signal,
    },
    int::UInt,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn construct<const N: usize>(digits: UInt<N>, exp: i32, cb: ControlBlock) -> D<N> {
    construct_with_clength(digits, exp, cb, clength(digits))
}

#[inline]
pub(crate) const fn construct_with_clength<const N: usize>(
    mut digits: UInt<N>,
    mut exp: i32,
    mut cb: ControlBlock,
    clength: u32,
) -> D<N> {
    // Overflow exp > Emax
    if exp > Intrinsics::<N>::E_MAX {
        return overflow(cb);
    }

    // Underflow exp < Emin
    if exp < E_MIN {
        return underflow(cb);
    }

    if exp <= E_LIMIT {
        if exp < E_MIN + (clength as i32 - 1) {
            cb = cb.raise_signal(Signal::OP_SUBNORMAL);
        }

        return D::new(digits, -exp as i16, cb);
    }

    cb = cb
        .raise_signal(Signal::OP_CLAMPED)
        .raise_signal(Signal::OP_ROUNDED);

    while exp > E_LIMIT {
        if digits.gt(&Intrinsics::<N>::COEFF_MEDIUM) {
            return D::INFINITY.with_cb(cb.raise_signal(Signal::OP_OVERFLOW));
        } else {
            digits = digits.strict_mul(UInt::<N>::TEN);
            exp -= 1;
        }
    }

    D::new(digits, -exp as i16, cb)
}
