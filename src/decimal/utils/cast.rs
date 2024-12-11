/// A drop-in replacement for [ToPrimitive](https://docs.rs/num-traits/latest/num_traits/cast/trait.ToPrimitive.html)
pub trait ToPrimitive {
    /// Converts the value of `self` to a `usize`. If the value cannot be
    /// represented by a `usize`, then `None` is returned.
    fn to_usize(&self) -> Option<usize>;

    /// Converts the value of `self` to a `u64`. If the value cannot be
    /// represented by a `u64`, then `None` is returned.
    fn to_u64(&self) -> Option<u64>;
}

impl ToPrimitive for i16 {
    #[inline]
    fn to_usize(&self) -> Option<usize> {
        if 0 <= *self {
            Some(*self as usize)
        } else {
            None
        }
    }

    #[inline]
    fn to_u64(&self) -> Option<u64> {
        if 0 <= *self {
            Some(*self as u64)
        } else {
            None
        }
    }
}

impl ToPrimitive for u64 {
    #[inline]
    fn to_usize(&self) -> Option<usize> {
        Some(*self as usize)
    }

    #[inline]
    fn to_u64(&self) -> Option<u64> {
        Some(*self)
    }
}

impl ToPrimitive for usize {
    #[inline]
    fn to_usize(&self) -> Option<usize> {
        Some(*self)
    }

    #[inline]
    fn to_u64(&self) -> Option<u64> {
        Some(*self as u64)
    }
}

impl ToPrimitive for i128 {
    #[inline]
    fn to_usize(&self) -> Option<usize> {
        let max = isize::MAX as i128;
        if 0 <= *self && *self <= max {
            Some(*self as usize)
        } else {
            None
        }
    }

    #[inline]
    fn to_u64(&self) -> Option<u64> {
        let max = u64::MAX as i128;
        if 0 <= *self && *self <= max {
            Some(*self as u64)
        } else {
            None
        }
    }
}
