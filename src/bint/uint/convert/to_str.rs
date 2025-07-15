use crate::bint::UInt;

impl<const N: usize> UInt<N> {
    /// Returns the integer as a string in the given radix.
    ///
    /// # Panics
    ///
    /// This function panics if `radix` is not in the range from 2 to 36
    /// inclusive.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastnum::U512;
    ///
    /// let src = "934857djkfghhkdfgbf9345hdfkh";
    /// let n = U512::from_str_radix(src, 36).unwrap();
    /// assert_eq!(n.to_str_radix(36), src);
    /// ```
    #[inline(always)]
    pub fn to_str_radix(&self, radix: u32) -> String {
        self.0.to_str_radix(radix)
    }
}
