use core::fmt;

/// All possible errors
///
/// Downstream crates should implement `From` to compose more custom errors.
#[derive(Debug)]
pub enum Error {
  /// Errors of the `cl-traits` crate
  ClTraits(cl_traits::Error),
  /// Couldn't be a string
  Fmt(fmt::Error),
  /// Some internal operation found a hash collision of two table ids (likely) or a hash collision
  /// due to a number of nested associations larger than `MAX_NODES_NUM` (unlikely).
  HashCollision(u64, &'static str, &'static str),
  /// No row was returned by the database
  NoDatabaseRowResult,
  /// Errors of the `sqlx_core` crate
  #[cfg(any(feature = "sqlx-postgres", feature = "sqlx-runtime-tokio-native-tls"))]
  Sqlx(sqlx_core::error::Error),
}

impl From<Error> for () {
  #[inline]
  fn from(_: Error) -> Self {}
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

#[cfg(any(feature = "sqlx-postgres", feature = "sqlx-runtime-tokio-native-tls"))]
impl From<sqlx_core::error::Error> for Error {
  #[inline]
  fn from(from: sqlx_core::error::Error) -> Self {
    Self::Sqlx(from)
  }
}
