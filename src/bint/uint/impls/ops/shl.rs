use core::ops::{Shl, ShlAssign};

use crate::bint::{impls::ops::shift::shift_impl, intrinsics::ExpType, UInt};

shift_impl!(UInt, U, Shl, shl, ShlAssign, shl_assign);
