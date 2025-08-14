mod add;
mod cmp;
mod digits;
mod div;
mod ilog;
mod mul;
mod powers;
mod transmute;
mod u128;

pub use add::*;
pub use cmp::*;
pub use digits::*;
pub use div::*;
pub use ilog::*;
pub use mul::*;
pub use powers::*;
pub use transmute::*;
pub use u128::*;

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
