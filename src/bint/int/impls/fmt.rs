use core::fmt::{self, Debug, Display, Formatter};

use crate::bint::{impls::fmt::fmt_impl, Int};

fmt_impl!(Int, I);
