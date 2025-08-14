use crate::{
    bint::UInt,
    decimal::{dec::scale::rescale, Decimal},
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn resize<const N: usize, const M: usize>(mut d: D<N>) -> D<M> {
    if M >= N {
        // SAFETY: M >= N
        #[allow(unsafe_code)]
        unsafe {
            d._transmute()
        }
    } else {
        let power = d
            .digits()
            .decimal_digits()
            .saturating_sub(UInt::<M>::MAX_POWER_OF_TEN + 1);
        if power > 0 {
            let scale = d.cb.get_scale() - power as i16;
            rescale(&mut d, scale);
        }

        if UInt::<N>::BITS - d.digits.leading_zeros() > UInt::<M>::BITS {
            let scale = d.cb.get_scale() - 1;
            rescale(&mut d, scale);
        }

        debug_assert!(d.digits.last_digit_index() < M);

        // SAFETY: d.digits now contains at most M digits
        #[allow(unsafe_code)]
        unsafe {
            d._transmute()
        }
    }
}
