use crate::{
  read_all, read_by_id, Buffer, FromRowsSuffix, InitialInsertValue, SqlWriter, TableParams,
  UpdateFieldValues, MAX_NODES_NUM,
};
use sqlx_core::{executor::Executor, postgres::PgPool};

/// Create, read, update and delete SQL operations
#[async_trait::async_trait]
pub trait Crud<B>: TableParams + Sized
where
  B: Buffer + Send + Sync,
  Self::Table: FromRowsSuffix<B, Error = Self::Error> + Unpin,
  Self::Associations: SqlWriter<B, Error = Self::Error>,
{
  /// Creates a new table on the database
  #[inline]
  async fn create<'table>(
    &mut self,
    buffer: &mut B,
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
  async fn read_all(&self, buffer: &mut B, pool: &PgPool) -> Result<Vec<Self::Table>, Self::Error> {
    Ok(read_all(buffer, pool, self).await?)
  }

  /// Gets a single stored entity based on its id.
  #[inline]
  async fn read_by_id(
    &self,
    buffer: &mut B,
    id: &Self::IdValue,
    pool: &PgPool,
  ) -> Result<Self::Table, Self::Error>
  where
    Self::IdValue: Sync,
  {
    Ok(read_by_id(buffer, id, pool, self).await?)
  }
}

#[async_trait::async_trait]
impl<B, T> Crud<B> for T
where
  B: Buffer + Send + Sync,
  T: TableParams,
  T::Table: FromRowsSuffix<B, Error = T::Error> + Unpin,
  T::Associations: SqlWriter<B, Error = Self::Error>,
{
}
