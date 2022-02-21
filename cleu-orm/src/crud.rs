//! Operations for different types of databases

#![cfg(any(feature = "with-sqlx-postgres", feature = "with-sqlx-runtime-tokio-native-tls"))]

mod utils;

use crate::{
  FromRowsSuffix, InitialInsertValue, Limit, OrderBy, SqlWriter, TableParams, UpdateFieldValues,
  MAX_NODES_NUM,
};
use sqlx_core::{executor::Executor, postgres::PgPool};
pub use utils::*;

/// Create, read, update and delete SQL operations
#[async_trait::async_trait]
pub trait Crud<S>: TableParams + Sized
where
  S: Send + cl_traits::String + Sync,
  Self::Table: FromRowsSuffix<S, Error = Self::Error> + Unpin,
  Self::Associations: SqlWriter<S, Error = Self::Error>,
{
  /// Creates a new table on the database
  #[inline]
  async fn create<'table>(
    &mut self,
    buffer: &mut S,
    pool: &PgPool,
    table: &'table Self::Table,
  ) -> Result<(), Self::Error>
  where
    Self: UpdateFieldValues<&'table <Self as TableParams>::Table>,
    Self::Table: Sync,
  {
    self.update_field_values(table);
    self.write_insert::<InitialInsertValue>(
      &mut [Default::default(); MAX_NODES_NUM],
      buffer,
      &mut None,
    )?;
    let _ = pool.execute(buffer.as_ref()).await.map_err(|err| err.into())?;
    Ok(())
  }

  /// Gets all stored entities.
  #[inline]
  async fn read_all(&self, buffer: &mut S, pool: &PgPool) -> Result<Vec<Self::Table>, Self::Error> {
    Ok(read_all(buffer, pool, self).await?)
  }

  /// Gets a single stored entity based on its id.
  #[inline]
  async fn read_by_id(
    &self,
    buffer: &mut S,
    id: &Self::IdValue,
    pool: &PgPool,
  ) -> Result<Self::Table, Self::Error>
  where
    Self::IdValue: Sync,
  {
    Ok(read_by_id(buffer, id, pool, self).await?)
  }

  /// Auxiliary method that gets all stored entities filtered by a field.
  #[inline]
  async fn read_all_with_params(
    &self,
    buffer: &mut S,
    pool: &PgPool,
    order_by: OrderBy,
    limit: Limit,
    where_str: &str,
  ) -> Result<Vec<Self::Table>, Self::Error> {
    Ok(read_all_with_params(buffer, pool, self, order_by, limit, where_str).await?)
  }
}

#[async_trait::async_trait]
impl<S, T> Crud<S> for T
where
  S: cl_traits::String + Send + Sync,
  T: TableParams,
  T::Table: FromRowsSuffix<S, Error = T::Error> + Unpin,
  T::Associations: SqlWriter<S, Error = Self::Error>,
{
}
