use core::fmt::{self, Debug, Display, Formatter};

use crate::bint::{impls::fmt::fmt_impl, UInt};

fmt_impl!(UInt, U);
