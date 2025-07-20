use core::ops::{Rem, RemAssign};

use crate::bint::{impls::ops::rem::rem_impl, Int};

rem_impl!(Int, I);
