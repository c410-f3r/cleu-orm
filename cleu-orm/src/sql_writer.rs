use crate::{
  buffer_try_push_str, buffer_write_fmt, write_select_field, write_select_join,
  write_select_order_by, Associations, Buffer, Fields, TableParams,
};

/// Writes raw SQL commands
pub trait SqlWriter<B>
where
  B: Buffer,
{
  /// See [crate::Error].
  type Error: From<crate::Error>;

  /// Writes an entire SELECT command
  fn write_select(
    &self,
    buffer: &mut B,
    where_cb: impl FnMut(&mut B) -> Result<(), Self::Error> + Clone,
  ) -> Result<(), Self::Error>;

  /// Only writes JOIN commands that belong to SELECT
  fn write_select_associations(&self, buffer: &mut B) -> Result<(), Self::Error>;

  /// Only writes querying fields that belong to SELECT
  fn write_select_fields(&self, buffer: &mut B) -> Result<(), Self::Error>;

  /// Only writes ORDER BY commands that belong to SELECT
  fn write_select_orders_by(&self, buffer: &mut B) -> Result<(), Self::Error>;
}

impl<B, E, T> SqlWriter<B> for T
where
  B: Buffer,
  E: From<crate::Error>,
  T: TableParams<Error = E>,
  T::Associations: SqlWriter<B, Error = E>,
{
  type Error = E;

  #[inline]
  fn write_select(
    &self,
    buffer: &mut B,
    mut where_cb: impl FnMut(&mut B) -> Result<(), Self::Error> + Clone,
  ) -> Result<(), Self::Error> {
    buffer_try_push_str(buffer, "SELECT ")?;
    self.write_select_fields(buffer)?;
    if buffer.as_ref().ends_with(',') {
      buffer.truncate(buffer.as_ref().len().wrapping_sub(1))
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
    buffer_try_push_str(buffer, " WHERE ")?;
    where_cb(buffer)?;
    if buffer.as_ref().ends_with(" WHERE ") {
      buffer.truncate(buffer.as_ref().len().wrapping_sub(7))
    }
    buffer_try_push_str(buffer, " ORDER BY ")?;
    self.write_select_orders_by(buffer)?;
    if buffer.as_ref().ends_with(',') {
      buffer.truncate(buffer.as_ref().len().wrapping_sub(1))
    }
    Ok(())
  }

  #[inline]
  fn write_select_associations(&self, buffer: &mut B) -> Result<(), Self::Error> {
    for full_association in self.associations().full_associations() {
      write_select_join(buffer, Self::table_name(), self.suffix(), full_association)?;
      buffer_try_push_str(buffer, " ")?;
    }
    self.associations().write_select_associations(buffer)?;
    Ok(())
  }

  #[inline]
  fn write_select_fields(&self, buffer: &mut B) -> Result<(), Self::Error> {
    for field in self.fields().field_names() {
      write_select_field(
        buffer,
        Self::table_name(),
        Self::table_name_alias(),
        self.suffix(),
        field,
      )?;
      buffer_try_push_str(buffer, ",")?;
    }
    self.associations().write_select_fields(buffer)?;
    Ok(())
  }

  #[inline]
  fn write_select_orders_by(&self, buffer: &mut B) -> Result<(), Self::Error> {
    write_select_order_by(
      buffer,
      Self::table_name(),
      Self::table_name_alias(),
      self.suffix(),
      self.id_field(),
    )?;
    buffer_try_push_str(buffer, ",")?;
    self.associations().write_select_orders_by(buffer)?;
    Ok(())
  }
}
