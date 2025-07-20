use core::ops::{Rem, RemAssign};

use crate::bint::{impls::ops::rem::rem_impl, UInt};

rem_impl!(UInt, U);
