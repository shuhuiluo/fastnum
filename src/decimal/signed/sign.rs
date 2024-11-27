use core::{
    fmt,
    fmt::{Display, Formatter},
    hash::{Hash, Hasher},
    ops::{Mul, Not},
};

use crate::decimal::{math::result, signed::Decimal, unsigned::UnsignedDecimal, DecimalResult};

/// A `Sign` represents `sign` associated with decimal number.
#[derive(Copy, Clone, Debug)]
pub enum Sign {
    /// Minus "-" sign.
    Minus,

    /// No sign.
    NoSign,

    /// Explicit Plus "+" sign.
    Plus,
}

impl Sign {
    /// Returns the default `NoSign`.
    #[inline]
    pub const fn default() -> Self {
        Self::NoSign
    }

    /// Tests for `self` and `other` signs to be equal, and is used by `==`
    /// operator.
    #[inline]
    pub const fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Plus, Self::Plus) => true,
            (Self::Minus, Self::Minus) => true,
            (Self::NoSign, Self::NoSign) => true,
            (_, _) => false,
        }
    }

    /// Invert `Sign` value.
    /// Note that inverting [Self::NoSign] gives a [Self::Minus] result.
    ///
    /// # Example
    ///
    /// ```
    /// use fastnum::decimal::Sign;
    ///
    /// let sign = Sign::Plus;
    /// assert_eq!(sign.not(), Sign::Minus);
    /// assert_eq!(sign.not().not(), Sign::NoSign);
    ///
    /// let sign = Sign::Minus;
    /// assert_eq!(sign.not(), Sign::NoSign);
    /// assert_eq!(sign.not().not(), sign);
    ///
    /// let sign = Sign::NoSign;
    /// assert_eq!(sign.not(), Sign::Minus);
    /// assert_eq!(sign.not().not(), sign);
    /// ```
    #[inline]
    pub const fn not(self) -> Self {
        match self {
            Sign::Minus => Sign::NoSign,
            Sign::NoSign => Sign::Minus,
            Sign::Plus => Sign::Minus,
        }
    }

    // TODO
    /// Sign "multiplication".
    #[inline]
    pub const fn mul(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Sign::Plus, Sign::Plus) => Sign::Plus,
            (Sign::Minus, Sign::Minus) => Sign::NoSign,
            (Sign::Minus, _) | (_, Sign::Minus) => Sign::Minus,
            (Sign::Plus, _) | (_, Sign::Plus) => Sign::Plus,
            (_, _) => Sign::NoSign,
        }
    }

    #[inline]
    /// Sign "division".
    pub const fn div(self, rhs: Self) -> Self {
        self.mul(rhs)
    }
}

impl Default for Sign {
    fn default() -> Self {
        Self::default()
    }
}

impl PartialEq for Sign {
    fn eq(&self, other: &Self) -> bool {
        self.eq(other)
    }
}

impl Eq for Sign {}

impl Hash for Sign {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        let s = match self {
            Sign::Minus => -1,
            Sign::NoSign => 1,
            Sign::Plus => 1,
        };
        s.hash(state);
    }
}

impl Not for Sign {
    type Output = Sign;

    #[inline]
    fn not(self) -> Self::Output {
        self.not()
    }
}

impl Mul<Sign> for Sign {
    type Output = Sign;

    #[inline]
    fn mul(self, other: Sign) -> Sign {
        self.mul(other)
    }
}

impl Display for Sign {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Sign::Minus => "-".fmt(f),
            Sign::NoSign => Ok(()),
            Sign::Plus => "+".fmt(f),
        }
    }
}

#[inline]
pub(crate) const fn signify_result<const N: usize>(
    res: DecimalResult<UnsignedDecimal<N>>,
    sign: Sign,
) -> DecimalResult<Decimal<N>> {
    let (value, flags) = res.split();
    result!(Decimal::new(value, sign)).add_flags(flags)
}
