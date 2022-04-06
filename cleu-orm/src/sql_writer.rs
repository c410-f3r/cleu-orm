mod write_insert;
mod write_select;
mod write_update;

use crate::{
  buffer_try_push_str, write_full_select_field, write_select_join, write_select_order_by,
  SelectLimit, SelectOrderBy, Table, TableAssociations, TableDefs, TableFields,
  TableSourceAssociation, MAX_NODES_NUM,
};
use core::{fmt::Display, marker::PhantomData};

/// Writes raw SQL commands
pub trait SqlWriter<B>
where
  B: cl_traits::String,
{
  /// See [crate::Error].
  type Error: From<crate::Error>;

  /// Writes an entire INSERT command
  fn write_insert<'value, V>(
    &self,
    aux: &mut [Option<&'static str>; MAX_NODES_NUM],
    buffer: &mut B,
    table_source_association: &mut Option<TableSourceAssociation<'value, V>>,
  ) -> Result<(), Self::Error>
  where
    V: Display;

  /// Writes an entire SELECT command
  fn write_select(
    &self,
    buffer: &mut B,
    order_by: SelectOrderBy,
    limit: SelectLimit,
    where_cb: &mut impl FnMut(&mut B) -> Result<(), Self::Error>,
  ) -> Result<(), Self::Error>;

  /// Only writes JOIN commands that belong to SELECT
  fn write_select_associations(&self, buffer: &mut B) -> Result<(), Self::Error>;

  /// Only writes querying fields that belong to SELECT
  fn write_select_fields(&self, buffer: &mut B) -> Result<(), Self::Error>;

  /// Only writes ORDER BY commands that belong to SELECT
  fn write_select_orders_by(&self, buffer: &mut B) -> Result<(), Self::Error>;

  /// Writes an entire UPDATE command
  fn write_update(
    &self,
    aux: &mut [Option<&'static str>; MAX_NODES_NUM],
    buffer: &mut B,
  ) -> Result<(), Self::Error>;
}

impl<'entity, B, TD> SqlWriter<B> for Table<'entity, TD>
where
  B: cl_traits::String,
  TD: TableDefs<'entity>,
  TD::Associations: SqlWriter<B, Error = TD::Error>,
  TD::Error: From<crate::Error>,
{
  type Error = TD::Error;

  #[inline]
  fn write_insert<'value, V>(
    &self,
    aux: &mut [Option<&'static str>; MAX_NODES_NUM],
    buffer: &mut B,
    tsa: &mut Option<TableSourceAssociation<'value, V>>,
  ) -> Result<(), Self::Error>
  where
    V: Display,
  {
    SqlWriterLogic::write_insert(aux, buffer, self, tsa)
  }

  #[inline]
  fn write_select(
    &self,
    buffer: &mut B,
    order_by: SelectOrderBy,
    select_limit: SelectLimit,
    where_cb: &mut impl FnMut(&mut B) -> Result<(), Self::Error>,
  ) -> Result<(), Self::Error> {
    SqlWriterLogic::write_select(buffer, order_by, select_limit, self, where_cb)
  }

  #[inline]
  fn write_select_associations(&self, buffer: &mut B) -> Result<(), Self::Error> {
    for full_association in self.associations().full_associations() {
      write_select_join(buffer, TD::TABLE_NAME, self.suffix(), full_association)?;
      buffer_try_push_str(buffer, " ")?;
    }
    self.associations().write_select_associations(buffer)?;
    Ok(())
  }

  #[inline]
  fn write_select_fields(&self, buffer: &mut B) -> Result<(), Self::Error> {
    write_full_select_field(
      buffer,
      TD::TABLE_NAME,
      TD::TABLE_NAME_ALIAS,
      self.suffix(),
      self.id_field().name(),
    )?;
    buffer_try_push_str(buffer, ",")?;
    for field in self.fields().field_names() {
      write_full_select_field(buffer, TD::TABLE_NAME, TD::TABLE_NAME_ALIAS, self.suffix(), field)?;
      buffer_try_push_str(buffer, ",")?;
    }
    self.associations().write_select_fields(buffer)?;
    Ok(())
  }

  #[inline]
  fn write_select_orders_by(&self, buffer: &mut B) -> Result<(), Self::Error> {
    write_select_order_by(
      buffer,
      TD::TABLE_NAME,
      TD::TABLE_NAME_ALIAS,
      self.suffix(),
      self.id_field().name(),
    )?;
    buffer_try_push_str(buffer, ",")?;
    self.associations().write_select_orders_by(buffer)?;
    Ok(())
  }

  #[inline]
  fn write_update(
    &self,
    aux: &mut [Option<&'static str>; MAX_NODES_NUM],
    buffer: &mut B,
  ) -> Result<(), Self::Error> {
    SqlWriterLogic::write_update(aux, buffer, self)
  }
}

struct SqlWriterLogic<'entity, B, TD>(PhantomData<(&'entity (), B, TD)>)
where
  B: cl_traits::String,
  TD: TableDefs<'entity>,
  TD::Associations: SqlWriter<B, Error = TD::Error>,
  TD::Error: From<crate::Error>;
