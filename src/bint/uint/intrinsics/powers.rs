use crate::{
    bint::{
        intrinsics::{ExpType, *},
        UInt,
    },
    utils::err_msg,
};

macro_rules! powers_impl {
    ($struct_: ident, $DIGIT_POWER: ident, $BASE: ident, $log: expr) => {
        pub(super) struct $struct_<const N: usize> {
            powers: [[UInt<N>; $DIGIT_POWER as usize + 1]; N],
            max_reduced_by_powers: [[UInt<N>; $DIGIT_POWER as usize + 1]; N],
        }

        impl<const N: usize> $struct_<N> {
            pub(super) const MAX_POWER: ExpType = $log;

            const PREV: UInt<N> = UInt::<N>::MAX.div(UInt::$BASE);

            pub(super) const fn new() -> Self {
                debug_assert!(N > 0);
                debug_assert!(Self::MAX_POWER < ($DIGIT_POWER + 1) * (N as ExpType));

                let mut powers = [[UInt::ZERO; $DIGIT_POWER as usize + 1]; N];
                let mut max_reduced_by_powers = [[UInt::ZERO; $DIGIT_POWER as usize + 1]; N];

                powers[0][0] = UInt::ONE;
                max_reduced_by_powers[0][0] = UInt::MAX;

                let mut v;
                let mut j = 0;
                let mut i = 1;
                v = UInt::ONE;

                while v.le(&Self::PREV) {
                    v = v.strict_mul(UInt::$BASE);

                    powers[j][i] = v;
                    max_reduced_by_powers[j][i] = UInt(UInt::MAX.0.div(v.0));

                    i += 1;

                    if i == $DIGIT_POWER as usize + 1 {
                        i = 0;
                        j += 1;
                    }
                }

                Self {
                    powers,
                    max_reduced_by_powers,
                }
            }

            #[inline(always)]
            pub(super) const fn power(&self, power: ExpType) -> UInt<N> {
                debug_assert!(power <= Self::MAX_POWER);

                let j = (power / ($DIGIT_POWER + 1)) as usize;

                if j >= N {
                    panic!(err_msg!("power is too large"));
                }

                let i = (power % ($DIGIT_POWER + 1)) as usize;
                self.powers[j][i]
            }

            #[inline(always)]
            pub(super) const fn checked_power(&self, power: ExpType) -> Option<UInt<N>> {
                if power > Self::MAX_POWER {
                    None
                } else {
                    Some(self.power(power))
                }
            }

            #[allow(dead_code)]
            #[inline(always)]
            pub(super) const fn max_reduced_by_power(&self, power: ExpType) -> UInt<N> {
                debug_assert!(power <= Self::MAX_POWER);

                let j = (power / ($DIGIT_POWER + 1)) as usize;

                if j >= N {
                    panic!(err_msg!("power is too large"));
                }

                let i = (power % ($DIGIT_POWER + 1)) as usize;
                self.max_reduced_by_powers[j][i]
            }

            #[allow(dead_code)]
            #[inline(always)]
            pub(super) const fn checked_max_reduced_by_power(
                &self,
                power: ExpType,
            ) -> Option<UInt<N>> {
                if power > Self::MAX_POWER {
                    None
                } else {
                    Some(self.max_reduced_by_power(power))
                }
            }
        }
    };
}

powers_impl!(
    PowersOf10,
    DIGIT_POWER_10,
    TEN,
    bnum::BUint::<N>::MAX.ilog10()
);
powers_impl!(
    PowersOf5,
    DIGIT_POWER_5,
    FIVE,
    bnum::BUint::<N>::MAX.ilog(bnum::BUint::FIVE)
);
