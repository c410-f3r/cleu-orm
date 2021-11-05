use crate::{
  buffer_try_push, buffer_try_push_str, buffer_write_fmt, write_select_field, write_select_join,
  write_select_order_by, Associations, Fields, TableParams,
};
use arrayvec::ArrayString;

/// Writes raw SQL commands
pub trait SqlWriter<const N: usize> {
  /// See [crate::Error].
  type Error: From<crate::Error>;

  /// Writes an entire SELECT command
  fn write_select(&self, buffer: &mut ArrayString<N>, where_str: &str) -> Result<(), Self::Error>;

  /// Only writes JOIN commands that belong to SELECT
  fn write_select_associations(&self, buffer: &mut ArrayString<N>) -> Result<(), Self::Error>;

  /// Only writes querying fields that belong to SELECT
  fn write_select_fields(&self, buffer: &mut ArrayString<N>) -> Result<(), Self::Error>;

  /// Only writes ORDER BY commands that belong to SELECT
  fn write_select_orders_by(&self, buffer: &mut ArrayString<N>) -> Result<(), Self::Error>;
}

impl<E, T, const N: usize> SqlWriter<N> for T
where
  E: From<crate::Error>,
  T: TableParams<Error = E>,
  T::Associations: SqlWriter<N, Error = E>,
{
  type Error = E;

  #[inline]
  fn write_select(&self, buffer: &mut ArrayString<N>, where_str: &str) -> Result<(), Self::Error> {
    buffer_try_push_str(buffer, "SELECT ")?;
    self.write_select_fields(buffer)?;
    if buffer.ends_with(',') {
      buffer.truncate(buffer.len().wrapping_sub(1))
    }
    buffer_write_fmt(
      buffer,
      format_args!(
        " FROM \"{table}\" AS \"{table}{suffix}\" ",
        suffix = self.suffix(),
        table = Self::table_name()
      ),
    )?;
    self.write_select_associations(buffer)?;
    buffer_try_push_str(buffer, where_str)?;
    buffer_try_push_str(buffer, " ORDER BY ")?;
    self.write_select_orders_by(buffer)?;
    if buffer.ends_with(',') {
      buffer.truncate(buffer.len().wrapping_sub(1))
    }
    Ok(())
  }

  #[inline]
  fn write_select_associations(&self, buffer: &mut ArrayString<N>) -> Result<(), Self::Error> {
    for full_association in self.associations().full_associations() {
      write_select_join(buffer, Self::table_name(), self.suffix(), full_association)?;
      buffer_try_push(buffer, ' ')?;
    }
    self.associations().write_select_associations(buffer)?;
    Ok(())
  }

  #[inline]
  fn write_select_fields(&self, buffer: &mut ArrayString<N>) -> Result<(), Self::Error> {
    for field in self.fields().field_names() {
      write_select_field(
        buffer,
        Self::table_name(),
        Self::table_name_alias(),
        self.suffix(),
        field,
      )?;
      buffer_try_push(buffer, ',')?;
    }
    self.associations().write_select_fields(buffer)?;
    Ok(())
  }

  #[inline]
  fn write_select_orders_by(&self, buffer: &mut ArrayString<N>) -> Result<(), Self::Error> {
    write_select_order_by(
      buffer,
      Self::table_name(),
      Self::table_name_alias(),
      self.suffix(),
      self.id_field(),
    )?;
    buffer_try_push(buffer, ',')?;
    //for _ in  self.associations().full_associations() {
    //  write_select_order_by(buffer, Self::table_name(), Self::alias(), self.suffix(), self.id_field())?;
    //  buffer_try_push(buffer, ',')?;
    //}
    self.associations().write_select_orders_by(buffer)?;
    Ok(())
  }
}
