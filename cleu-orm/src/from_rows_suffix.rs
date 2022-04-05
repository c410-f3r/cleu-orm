#![cfg(any(feature = "sqlx-postgres", feature = "sqlx-runtime-tokio-native-tls"))]

use crate::Suffix;
use sqlx_core::postgres::PgRow;

/// Constructs a single instance based on an arbitrary number of rows
pub trait FromRowsSuffix<B>: Sized
where
  B: cl_traits::String,
{
  /// See [crate::Error]
  type Error: From<crate::Error>;

  /// See [FromRowsSuffix].
  fn from_rows_suffix(
    all_rows: &[PgRow],
    buffer: &mut B,
    suffix: Suffix,
    target_row: &PgRow,
  ) -> Result<(usize, Self), Self::Error>;
}
