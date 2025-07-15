use core::ops::{Add, AddAssign};

use crate::decimal::{impls::ops::ops_impl, Decimal};

ops_impl!(Decimal, I, Add, add, AddAssign, add_assign);
