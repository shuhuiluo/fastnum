use core::ops::{Rem, RemAssign};

use crate::decimal::{impls::ops::ops_impl, UnsignedDecimal};

ops_impl!(UnsignedDecimal, U, Rem, rem, RemAssign, rem_assign);
