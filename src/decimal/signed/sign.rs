use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::Not;

/// A `Sign` represents `sign` associated with decimal number.
#[derive(Default, PartialEq, PartialOrd, Eq, Ord, Copy, Clone, Debug, Hash)]
pub enum Sign {
    /// Minus "-" sign.
    Minus,
    
    /// No sign.
    #[default]
    NoSign,
    
    /// Explicit Plus "+" sign.
    Plus,
}

impl Sign {
    /// Invert `Sign` value.
    /// Note that inverting [Self::NoSign] gives a [Self::Minus] result.
    ///
    /// # Example
    ///
    /// ```
    /// use fastnum::decimal::signed::Sign;
    /// 
    /// let sign = Sign::Plus;
    /// assert_eq!(sign.not(), Sign::Minus);
    /// assert_eq!(sign.not().not(), sign);
    /// 
    /// let sign = Sign::Minus;
    /// assert_eq!(sign.not(), Sign::Plus);
    /// assert_eq!(sign.not().not(), sign);
    /// 
    /// let sign = Sign::NoSign;
    /// assert_eq!(sign.not(), Sign::Minus);
    /// assert_ne!(sign.not().not(), sign);
    /// 
    /// ```
    #[inline]
    pub const fn not(self) -> Self {
        match self {
            Sign::Minus => Sign::Plus,
            Sign::NoSign => Sign::Minus,
            Sign::Plus => Sign::Minus,
        }
    }
}

impl Not for Sign {
    type Output = Sign;

    #[inline]
    fn not(self) -> Self::Output {
        self.not()
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
