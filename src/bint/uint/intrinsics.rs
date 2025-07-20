use crate::{bint::UInt, utils::err_msg};

pub use crate::bint::intrinsics::*;

macro_rules! powers_impl {
    ($struct_: ident, $DIGIT_POWER: ident, $BASE: ident, $log: expr) => {
        #[repr(transparent)]
        struct $struct_<const N: usize>([[UInt<N>; $DIGIT_POWER as usize + 1]; N]);

        impl<const N: usize> $struct_<N> {
            const MAX_POWER: u32 = $log;
            const PREV: UInt<N> = UInt::<N>::MAX.div(UInt::$BASE);

            const fn new() -> Self {
                debug_assert!(N > 0);
                debug_assert!(Self::MAX_POWER < ($DIGIT_POWER + 1) * (N as u32));

                let mut res = [[UInt::ZERO; $DIGIT_POWER as usize + 1]; N];
                res[0][0] = UInt::ONE;

                let mut v;
                let mut j = 0;
                let mut i = 1;
                v = UInt::ONE;

                while v.le(&Self::PREV) {
                    v = v.strict_mul(UInt::$BASE);
                    res[j][i] = v;
                    i += 1;

                    if i == $DIGIT_POWER as usize + 1 {
                        i = 0;
                        j += 1;
                    }
                }

                Self(res)
            }

            #[inline(always)]
            const fn lookup(&self, power: u32) -> UInt<N> {
                debug_assert!(power <= Self::MAX_POWER);

                let j = (power / ($DIGIT_POWER + 1)) as usize;

                if j >= N {
                    panic!(err_msg!("power is too large"));
                }

                let i = (power % ($DIGIT_POWER + 1)) as usize;
                self.0[j][i]
            }

            #[inline(always)]
            const fn checked_lookup(&self, power: u32) -> Option<UInt<N>> {
                if power > Self::MAX_POWER {
                    None
                } else {
                    Some(self.lookup(power))
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

pub struct Intrinsics<const N: usize>;

impl<const N: usize> Intrinsics<N> {
    const POWERS_OF_FIVE: PowersOf5<N> = PowersOf5::new();
    const POWERS_OF_TEN: PowersOf10<N> = PowersOf10::new();

    pub(crate) const MAX_POWER_OF_FIVE: u32 = PowersOf5::<N>::MAX_POWER;

    #[inline(always)]
    pub(crate) const fn checked_power_of_five(power: u32) -> Option<UInt<N>> {
        Self::POWERS_OF_FIVE.checked_lookup(power)
    }

    #[allow(dead_code)]
    #[inline(always)]
    pub(crate) const fn unchecked_power_of_five(power: u32) -> UInt<N> {
        Self::POWERS_OF_FIVE.lookup(power)
    }

    #[inline(always)]
    pub(crate) const fn checked_power_of_ten(power: u32) -> Option<UInt<N>> {
        Self::POWERS_OF_TEN.checked_lookup(power)
    }

    #[inline(always)]
    pub(crate) const fn unchecked_power_of_ten(power: u32) -> UInt<N> {
        Self::POWERS_OF_TEN.lookup(power)
    }
}
