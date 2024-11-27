use core::{
    fmt,
    fmt::{Debug, Display, Formatter},
};

use crate::utils::err_prefix;

/// Enum to store the various types of errors that can cause any operation with
/// decimal to fail.
/// 
/// For more information: see [`ArithmeticResult`](crate#arithmetic-result).
///
/// # Example
///
/// ```
/// use fastnum::decimal::RoundingMode;
/// use fastnum::udec128;
///
/// if let Err(e) = udec128!(129.42).sub(udec128!(129.421), RoundingMode::Down).ok_or_err() {
///     println!("Failed to subtract: {e}");
/// }
/// ```
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ArithmeticError {
    /// Division by zero.
    DivideByZero,
    
    /// Indicates that the decimal result of an operation is too large to fit the target type. 
    Overflow,
    
    /// Rounding was performed during the operation. The result may not be exact.
    Inexact,
    
    /// The negative result cannot be represented by an unsigned type.
    Signed,
}

impl ArithmeticError {
    pub(crate) const fn description(&self) -> &str {
        use ArithmeticError::*;
        match self {
            DivideByZero => "divide by zero",
            Overflow => "overflow occurred while performing arithmetic operation",
            Inexact => "result may be inexact",
            Signed => "number would be signed for unsigned type",
        }
    }
}

impl Display for ArithmeticError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} {}", err_prefix!(), self.description())
    }
}

impl Debug for ArithmeticError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self, f)
    }
}

impl core::error::Error for ArithmeticError {
    #[inline]
    fn description(&self) -> &str {
        self.description()
    }
}
