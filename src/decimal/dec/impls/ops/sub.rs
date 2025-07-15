use core::ops::{Sub, SubAssign};

use crate::decimal::{impls::ops::ops_impl, Decimal};

ops_impl!(Decimal, I, Sub, sub, SubAssign, sub_assign);