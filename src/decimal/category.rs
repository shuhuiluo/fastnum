/// More about decimal categories and special values:
/// [documentation](crate#special-values)
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Category {
    /// `NaN` (not a number): this value results from calculations like
    /// `(-1.0).sqrt()`.
    ///
    /// See [the documentation](crate#special-values) for more information on
    /// the unusual properties of `NaN`.
    Nan,

    /// Positive or negative infinity, which often results from dividing a
    /// nonzero number by zero.
    Infinite,

    /// Positive or negative zero.
    ///
    /// See [the documentation](crate#special-values) for more information on
    /// the signedness of zeroes.
    Zero,

    /// “Subnormal” or _“denormal”_ decimal representation (less precise,
    /// relative to their magnitude, than [`Normal`]).
    ///
    /// Subnormal numbers are larger in magnitude than [`Zero`] but smaller in
    /// magnitude than all [`Normal`] numbers.
    ///
    /// [`Normal`]: Self::Normal
    /// [`Zero`]: Self::Zero
    Subnormal,

    /// A regular decimal number, not any of the exceptional categories.
    Normal,
}
