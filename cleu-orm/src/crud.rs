use crate::{read_all, read_by_id, Buffer, FromRowsSuffix, SqlWriter, TableParams};
use core::fmt;
use sqlx_core::postgres::PgPool;

/// Create, read, update and delete SQL operations
#[async_trait::async_trait]
pub trait Crud<B>: TableParams + Sized
where
  B: Buffer + Send + Sync,
  Self::Table: FromRowsSuffix<B, Error = Self::Error> + Send + Unpin,
  Self::Associations: SqlWriter<B, Error = Self::Error>,
{
  /// Gets all stored entities.
  #[inline]
  async fn read_all(&self, buffer: &mut B, pool: &PgPool) -> Result<Vec<Self::Table>, Self::Error> {
    Ok(read_all(buffer, pool, self).await?)
  }

  /// Gets a single stored entity based on its id.
  #[inline]
  async fn read_by_id<F>(
    &self,
    buffer: &mut B,
    id: F,
    pool: &PgPool,
  ) -> Result<Self::Table, Self::Error>
  where
    F: fmt::Display + Send,
  {
    Ok(read_by_id(buffer, id, pool, self).await?)
  }
}

#[async_trait::async_trait]
impl<B, T> Crud<B> for T
where
  B: Buffer + Send + Sync,
  T: TableParams,
  T::Table: FromRowsSuffix<B, Error = T::Error> + Send + Unpin,
  T::Associations: SqlWriter<B, Error = Self::Error>,
{
}
