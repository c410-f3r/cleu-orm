use crate::Buffer;
use sqlx_core::postgres::PgRow;

/// Constructs a single instance based on an arbitrary number of rows
pub trait FromRowsSuffix<B>: Sized
where
  B: Buffer,
{
  /// See [crate::Error]
  type Error: From<crate::Error>;

  /// See [FromRowsSuffix].
  fn from_rows_suffix(
    all_rows: &[PgRow],
    buffer: &mut B,
    suffix: u8,
    target_row: &PgRow,
  ) -> Result<(usize, Self), Self::Error>;
}
