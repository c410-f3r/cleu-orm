use crate::{
  buffer_try_push_str, buffer_write_fmt, write_select_field, write_select_join,
  write_select_order_by, Associations, Fields, SourceAssociation, TableParams, MAX_NODES_NUM,
};
use core::fmt;

/// Writes raw SQL commands
pub trait SqlWriter<S>
where
  S: cl_traits::String,
{
  /// See [crate::Error].
  type Error: From<crate::Error>;

  /// Writes an entire INSERT command
  fn write_insert<'value, V>(
    &self,
    aux: &mut [Option<&'static str>; MAX_NODES_NUM],
    buffer: &mut S,
    source_association: &mut Option<SourceAssociation<'value, V>>,
  ) -> Result<(), Self::Error>
  where
    V: fmt::Display;

  /// Writes an entire SELECT command
  fn write_select(
    &self,
    buffer: &mut S,
    where_cb: &mut impl FnMut(&mut S) -> Result<(), Self::Error>,
  ) -> Result<(), Self::Error>;

  /// Only writes JOIN commands that belong to SELECT
  fn write_select_associations(&self, buffer: &mut S) -> Result<(), Self::Error>;

  /// Only writes querying fields that belong to SELECT
  fn write_select_fields(&self, buffer: &mut S) -> Result<(), Self::Error>;

  /// Only writes ORDER BY commands that belong to SELECT
  fn write_select_orders_by(&self, buffer: &mut S) -> Result<(), Self::Error>;
}

impl<E, S, T> SqlWriter<S> for T
where
  E: From<crate::Error>,
  S: cl_traits::String,
  T: TableParams<Error = E>,
  T::Associations: SqlWriter<S, Error = E>,
{
  type Error = E;

  #[inline]
  fn write_insert<'value, V>(
    &self,
    aux: &mut [Option<&'static str>; MAX_NODES_NUM],
    buffer: &mut S,
    source_association: &mut Option<SourceAssociation<'value, V>>,
  ) -> Result<(), Self::Error>
  where
    V: fmt::Display,
  {
    let idx = self.instance_idx();
    let table_name_opt = aux.get_mut(idx).ok_or(crate::Error::UnknownAuxIdx(idx))?;
    if let Some(table_name) = *table_name_opt {
      if table_name == T::table_name() {
        return Ok(());
      } else {
        return Err(crate::Error::HashCollision(table_name, T::table_name()).into());
      }
    } else {
      *table_name_opt = Some(T::table_name());
    }

    macro_rules! write {
      (
        $foreign_key_name_cb:expr,
        $foreign_key_value_cb:expr
      ) => {
        let len_before_insert = buffer.as_ref().len();
        buffer_write_fmt(
          buffer,
          format_args!("INSERT INTO \"{table}\" (", table = Self::table_name()),
        )?;
        let mut field_names = self.fields().field_names();
        if let Some(first) = field_names.next() {
          buffer_try_push_str(buffer, first)?;
        }
        for field in field_names {
          buffer_write_fmt(buffer, format_args!(",{}", field))?;
        }
        let foreign_key_name: crate::Result<()> = $foreign_key_name_cb(&mut *buffer);
        foreign_key_name?;

        buffer_try_push_str(buffer, ") VALUES (")?;
        let len_after_values = buffer.as_ref().len();
        self.fields().write_table_values(buffer)?;

        if buffer.as_ref().len() == len_after_values {
          buffer.truncate(len_before_insert);
        } else {
          let foreign_key_value: crate::Result<()> = $foreign_key_value_cb(&mut *buffer);
          foreign_key_value?;
          if buffer.as_ref().ends_with(',') {
            buffer.truncate(buffer.as_ref().len().wrapping_sub(1))
          }
          buffer_try_push_str(buffer, ");")?;
        }
      };
    }

    let mut new_source_assocition = self.id_field().value().as_ref().map(SourceAssociation::new);

    if let Some(ref elem) = *source_association {
      if elem.source_field() != self.id_field().name() {
        write!(
          |local| buffer_write_fmt(local, format_args!(",{}", elem.source_field())),
          |local| buffer_write_fmt(local, format_args!("'{}',", elem.source_value()))
        );
        self.associations().write_insert(aux, buffer, &mut new_source_assocition)?;
        return Ok(());
      }
    }
    write!(|_| Ok(()), |_| Ok(()));
    self.associations().write_insert(aux, buffer, &mut new_source_assocition)?;
    Ok(())
  }

  #[inline]
  fn write_select(
    &self,
    buffer: &mut S,
    where_cb: &mut impl FnMut(&mut S) -> Result<(), Self::Error>,
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
  fn write_select_associations(&self, buffer: &mut S) -> Result<(), Self::Error> {
    for full_association in self.associations().full_associations() {
      write_select_join(buffer, Self::table_name(), self.suffix(), full_association)?;
      buffer_try_push_str(buffer, " ")?;
    }
    self.associations().write_select_associations(buffer)?;
    Ok(())
  }

  #[inline]
  fn write_select_fields(&self, buffer: &mut S) -> Result<(), Self::Error> {
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
  fn write_select_orders_by(&self, buffer: &mut S) -> Result<(), Self::Error> {
    write_select_order_by(
      buffer,
      Self::table_name(),
      Self::table_name_alias(),
      self.suffix(),
      self.id_field().name(),
    )?;
    buffer_try_push_str(buffer, ",")?;
    self.associations().write_select_orders_by(buffer)?;
    Ok(())
  }
}
