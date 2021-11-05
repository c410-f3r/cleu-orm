use crate::{read_all, read_by_id, FromRowsSuffix, SqlWriter, TableParams};
use arrayvec::ArrayString;
use core::fmt;
use sqlx_core::postgres::PgPool;

/// Create, read, update and delete SQL operations
#[async_trait::async_trait]
pub trait Crud<const N: usize>: TableParams + Sized
where
  Self::Table: FromRowsSuffix<N, Error = Self::Error> + Send + Unpin,
  Self::Associations: SqlWriter<N, Error = Self::Error>,
{
  /// Gets all stored entities.
  #[inline]
  async fn read_all(
    &self,
    buffer: &mut ArrayString<N>,
    pool: &PgPool,
  ) -> Result<Vec<Self::Table>, Self::Error> {
    Ok(read_all(buffer, self, pool).await?)
  }

  /// Gets a single stored entity based on its id.
  #[inline]
  async fn read_by_id<F>(
    &self,
    buffer: &mut ArrayString<N>,
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
impl<T, const N: usize> Crud<N> for T
where
  T: TableParams,
  T::Table: FromRowsSuffix<N, Error = T::Error> + Send + Unpin,
  T::Associations: SqlWriter<N, Error = Self::Error>,
{
}
