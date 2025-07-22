mod div;
mod mul;

pub use div::div_rem_wide_digit;
pub use mul::{carrying_mul, carrying_mul_add};

pub type Digit = u64;
pub type DoubleDigit = u128;
pub type ExpType = u32;

pub type Digits<const N: usize> = [Digit; N];

pub const DIGIT_POWER_10: u32 = 19;
pub const DIGIT_POWER_5: u32 = 27;
pub const DIGIT_BITS: ExpType = Digit::BITS;
pub const DIGIT_BITS_MINUS_1: ExpType = DIGIT_BITS - 1;
pub const DIGIT_BIT_SHIFT: ExpType = DIGIT_BITS.trailing_zeros();

pub struct BInt<const N: usize>;

impl<const N: usize> BInt<N> {
    pub const BITS: ExpType = DIGIT_BITS * N as ExpType;
}

pub const DIGIT_POWERS_10: [Digit; 20] = [
    1,
    10,
    100,
    1000,
    10000,
    100000,
    1000000,
    10000000,
    100000000,
    1000000000,
    10000000000,
    100000000000,
    1000000000000,
    10000000000000,
    100000000000000,
    1000000000000000,
    10000000000000000,
    100000000000000000,
    1000000000000000000,
    10000000000000000000,
];
