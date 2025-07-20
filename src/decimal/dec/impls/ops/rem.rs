use core::ops::{Rem, RemAssign};

use crate::decimal::{impls::ops::ops_impl, Decimal};

ops_impl!(Decimal, I, Rem, rem, RemAssign, rem_assign);
