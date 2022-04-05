use crate::{
  buffer_try_push_str, buffer_write_fmt, write_full_select_field, write_insert_field,
  write_select_join, write_select_order_by, SelectLimit, SelectOrderBy, Table, TableAssociations,
  TableDefs, TableFields, TableSourceAssociation, MAX_NODES_NUM,
};
use core::fmt::Display;

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
}

impl<'entity, B, E, TD> SqlWriter<B> for Table<'entity, TD>
where
  B: cl_traits::String,
  E: From<crate::Error>,
  TD: TableDefs<'entity, Error = E>,
  TD::Associations: SqlWriter<B, Error = E>,
{
  type Error = E;

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
    let idx = self.instance_idx();
    let table_name_opt = aux.get_mut(idx).ok_or(crate::Error::UnknownAuxIdx(idx))?;

    if let Some(table_name) = *table_name_opt {
      if table_name == TD::TABLE_NAME {
        return Ok(());
      } else {
        return Err(crate::Error::HashCollision(idx, table_name, TD::TABLE_NAME).into());
      }
    } else {
      *table_name_opt = Some(TD::TABLE_NAME);
    }

    let elem_opt = || {
      if let Some(ref el) = *tsa {
        (el.source_field() != self.id_field().name()).then(|| el)
      } else {
        None
      }
    };

    if let Some(elem) = elem_opt() {
      write_insert_manager(
        buffer,
        self,
        |local| buffer_write_fmt(local, format_args!(",{}", elem.source_field())),
        |local| buffer_write_fmt(local, format_args!("'{}',", elem.source_value())),
      )?;
    } else {
      write_insert_manager(buffer, self, |_| Ok(()), |_| Ok(()))?;
    }
    let mut new_tsa = self.id_field().value().as_ref().map(TableSourceAssociation::new);
    self.associations().write_insert(aux, buffer, &mut new_tsa)?;

    Ok(())
  }

  #[inline]
  fn write_select(
    &self,
    buffer: &mut B,
    order_by: SelectOrderBy,
    select_limit: SelectLimit,
    where_cb: &mut impl FnMut(&mut B) -> Result<(), Self::Error>,
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
        table = TD::TABLE_NAME
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
    match order_by {
      SelectOrderBy::Ascending => buffer_try_push_str(buffer, " ASC")?,
      SelectOrderBy::Descending => buffer_try_push_str(buffer, " DESC")?,
    }
    buffer_try_push_str(buffer, " LIMIT ")?;
    match select_limit {
      SelectLimit::All => buffer_try_push_str(buffer, "ALL")?,
      SelectLimit::Count(n) => buffer_write_fmt(buffer, format_args!("{}", n))?,
    }
    Ok(())
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
}

fn write_insert_manager<'entity, B, E, TD>(
  buffer: &mut B,
  table: &Table<'entity, TD>,
  foreign_key_name_cb: impl Fn(&mut B) -> crate::Result<()>,
  foreign_key_value_cb: impl Fn(&mut B) -> crate::Result<()>,
) -> Result<(), E>
where
  B: cl_traits::String,
  E: From<crate::Error>,
  TD: TableDefs<'entity, Error = E>,
  TD::Associations: SqlWriter<B, Error = E>,
{
  let len_before_insert = buffer.as_ref().len();
  buffer_write_fmt(buffer, format_args!("INSERT INTO \"{}\" (", TD::TABLE_NAME))?;
  buffer_try_push_str(buffer, table.id_field().name())?;
  for field in table.fields().field_names() {
    buffer_write_fmt(buffer, format_args!(",{}", field))?;
  }
  foreign_key_name_cb(&mut *buffer)?;

  buffer_try_push_str(buffer, ") VALUES (")?;
  let len_after_values = buffer.as_ref().len();
  write_insert_field(buffer, table.id_field())?;
  table.fields().write_values(buffer)?;

  if buffer.as_ref().len() == len_after_values {
    buffer.truncate(len_before_insert);
  } else {
    foreign_key_value_cb(&mut *buffer)?;
    if buffer.as_ref().ends_with(',') {
      buffer.truncate(buffer.as_ref().len().wrapping_sub(1))
    }
    buffer_try_push_str(buffer, ");")?;
  }
  Ok(())
}
