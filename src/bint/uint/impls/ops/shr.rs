use core::ops::{Shr, ShrAssign};

use crate::bint::{impls::ops::shift::shift_impl, intrinsics::ExpType, UInt};

shift_impl!(UInt, U, Shr, shr, ShrAssign, shr_assign);
