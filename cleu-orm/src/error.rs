use core::fmt;

/// All possible errors
///
/// Downstream crates should implement `From` to compose more custom errors.
#[derive(Debug)]
pub enum Error {
  /// Insufficient capacity
  CapacityError(arrayvec::CapacityError),
  /// Errors of the `cl-traits` crate
  ClTraits(cl_traits::Error),
  /// Couldn't be a string
  Fmt(fmt::Error),
  /// No row was returned by the database
  NoDatabaseRowResult,
  /// Errors of the `sqlx_core` crate
  Sqlx(sqlx_core::error::Error),
}

impl From<Error> for () {
  #[inline]
  fn from(_: Error) -> Self {}
}

impl<T> From<arrayvec::CapacityError<T>> for Error {
  #[inline]
  fn from(_: arrayvec::CapacityError<T>) -> Self {
    Self::CapacityError(arrayvec::CapacityError::new(()))
  }
}

impl From<cl_traits::Error> for Error {
  #[inline]
  fn from(from: cl_traits::Error) -> Self {
    Self::ClTraits(from)
  }
}

impl From<fmt::Error> for Error {
  #[inline]
  fn from(from: fmt::Error) -> Self {
    Self::Fmt(from)
  }
}

impl From<sqlx_core::error::Error> for Error {
  #[inline]
  fn from(from: sqlx_core::error::Error) -> Self {
    Self::Sqlx(from)
  }
}
