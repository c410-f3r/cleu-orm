use core::fmt;

/// All possible errors
///
/// Downstream crates should implement `From` to compose more custom errors.
#[derive(Debug)]
pub enum Error {
  /// Insufficient capacity
  CapacityError(arrayvec::CapacityError),
  /// Couldn't be a string
  Fmt(fmt::Error),
  /// No row was returned by the database
  NoDatabaseRowResult,
  /// All SQL-related errors
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
