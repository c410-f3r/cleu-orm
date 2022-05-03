//! Operations for different types of databases

#![cfg(any(feature = "sqlx-postgres", feature = "sqlx-runtime-tokio-native-tls"))]

mod utils;

use crate::{
  FromRowsSuffix, InitialInsertValue, SelectLimit, SelectOrderBy, SqlWriter, Table, TableDefs,
};
use sqlx_core::{executor::Executor, postgres::PgPool};
pub use utils::*;

pub(crate) type TdEntity<'entity, TD> = <TD as TableDefs<'entity>>::Entity;
pub(crate) type TdError<'entity, TD> = <TD as TableDefs<'entity>>::Error;

impl<'entity, TD> Table<'entity, TD>
where
  TD: TableDefs<'entity>,
{
  /// Creates a new table on the database
  #[inline]
  pub async fn create<B>(
    &mut self,
    buffer: &mut B,
    pool: &PgPool,
    table: &'entity TD::Entity,
  ) -> Result<(), TdError<'entity, TD>>
  where
    B: cl_traits::String,
    TD::Entity: FromRowsSuffix<B, Error = TD::Error>,
    TD::Associations: SqlWriter<B, Error = TD::Error>,
  {
    self.update_all_table_fields(table);
    self.write_insert::<InitialInsertValue>(&mut <_>::default(), buffer, &mut None)?;
    let _ = pool.execute(buffer.as_ref()).await.map_err(Into::into)?;
    Ok(())
  }

  /// Gets all stored entities.
  #[inline]
  pub async fn read_all<B>(
    &self,
    buffer: &mut B,
    pool: &PgPool,
  ) -> Result<Vec<TdEntity<'entity, TD>>, TdError<'entity, TD>>
  where
    B: cl_traits::String,
    TD::Entity: FromRowsSuffix<B, Error = TD::Error>,
    TD::Associations: SqlWriter<B, Error = TD::Error>,
  {
    read_all(buffer, pool, self).await
  }

  /// Auxiliary method that gets all stored entities filtered by a field.
  #[inline]
  pub async fn read_all_with_params<B>(
    &self,
    buffer: &mut B,
    pool: &PgPool,
    order_by: SelectOrderBy,
    limit: SelectLimit,
    where_str: &str,
  ) -> Result<Vec<TdEntity<'entity, TD>>, TdError<'entity, TD>>
  where
    B: cl_traits::String,
    TD::Entity: FromRowsSuffix<B, Error = TD::Error>,
    TD::Associations: SqlWriter<B, Error = TD::Error>,
  {
    read_all_with_params(buffer, pool, self, order_by, limit, where_str).await
  }

  /// Gets a single stored entity based on its id.
  #[inline]
  pub async fn read_by_id<B>(
    &self,
    buffer: &mut B,
    id: &TD::PrimaryKeyValue,
    pool: &PgPool,
  ) -> Result<TdEntity<'entity, TD>, TdError<'entity, TD>>
  where
    B: cl_traits::String,
    TD::Entity: FromRowsSuffix<B, Error = TD::Error>,
    TD::Associations: SqlWriter<B, Error = TD::Error>,
  {
    read_by_id(buffer, id, pool, self).await
  }
}
