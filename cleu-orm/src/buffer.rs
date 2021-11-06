use cl_traits::{Clear, Push, Truncate};
use core::fmt::Write;

/// Buffer used to construct internal commands. Can be a [String] or [arrayvec::ArrayString]
pub trait Buffer:
  AsRef<str>
  + Clear
  + for<'x> Push<Error = cl_traits::Error, Input<'x> = &'x str, Output = ()>
  + Truncate<Input = usize, Output = ()>
  + Write
{
}

impl<T> Buffer for T where
  T: AsRef<str>
    + Clear
    + for<'x> Push<Error = cl_traits::Error, Input<'x> = &'x str, Output = ()>
    + Truncate<Input = usize, Output = ()>
    + Write
{
}
