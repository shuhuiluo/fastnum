pub type Digit = u64;
pub type DoubleDigit = u128;
pub type ExpType = u32;

pub type Digits<const N: usize> = [Digit; N];

pub const POWER: u32 = 19;
pub const BITS: ExpType = Digit::BITS;
pub const BITS_MINUS_1: ExpType = BITS - 1;
pub const BIT_SHIFT: ExpType = BITS.trailing_zeros();

pub struct BInt<const N: usize>;

impl<const N: usize> BInt<N> {
    pub const BITS: ExpType = BITS * N as ExpType;
}
