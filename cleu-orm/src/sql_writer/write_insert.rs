use crate::{
  buffer_try_push_str, buffer_write_fmt, node_was_already_visited, sql_writer::SqlWriterLogic,
  truncate_if_ends_with_char, AuxNodes, SqlValue, SqlWriter, Table, TableDefs, TableFields,
  TableSourceAssociation,
};
use core::fmt::Display;

impl<'entity, B, TD> SqlWriterLogic<'entity, B, TD>
where
  B: cl_traits::String,
  TD: TableDefs<'entity>,
  TD::Associations: SqlWriter<B, Error = TD::Error>,
  TD::Error: From<crate::Error>,
{
  #[inline]
  pub(crate) fn write_insert<'value, V>(
    aux: &mut AuxNodes,
    buffer: &mut B,
    table: &Table<'entity, TD>,
    tsa: &mut Option<TableSourceAssociation<'value, V>>,
  ) -> Result<(), TD::Error>
  where
    V: Display,
  {
    if node_was_already_visited(aux, table)? {
      return Ok(());
    }

    let elem_opt = || {
      if let Some(ref el) = *tsa {
        (el.source_field() != table.id_field().name()).then(|| el)
      } else {
        None
      }
    };

    if let Some(elem) = elem_opt() {
      Self::write_insert_manager(
        buffer,
        table,
        |local| buffer_write_fmt(local, format_args!(",{}", elem.source_field())),
        |local| buffer_write_fmt(local, format_args!("'{}',", elem.source_value())),
      )?;
    } else {
      Self::write_insert_manager(buffer, table, |_| Ok(()), |_| Ok(()))?;
    }

    let mut new_tsa = table.id_field().value().as_ref().map(TableSourceAssociation::new);
    table.associations().write_insert(aux, buffer, &mut new_tsa)?;

    Ok(())
  }

  fn write_insert_manager(
    buffer: &mut B,
    table: &Table<'entity, TD>,
    foreign_key_name_cb: impl Fn(&mut B) -> crate::Result<()>,
    foreign_key_value_cb: impl Fn(&mut B) -> crate::Result<()>,
  ) -> Result<(), TD::Error> {
    let len_before_insert = buffer.as_ref().len();

    buffer_write_fmt(buffer, format_args!("INSERT INTO \"{}\" (", TD::TABLE_NAME))?;
    buffer_try_push_str(buffer, table.id_field().name())?;
    for field in table.fields().field_names() {
      buffer_write_fmt(buffer, format_args!(",{}", field))?;
    }
    foreign_key_name_cb(&mut *buffer)?;

    buffer_try_push_str(buffer, ") VALUES (")?;
    let len_before_values = buffer.as_ref().len();
    if let &Some(ref elem) = table.id_field().value() {
      elem.write(buffer)?;
      buffer_try_push_str(buffer, ",")?;
    }
    table.fields().write_insert_values(buffer)?;

    if buffer.as_ref().len() == len_before_values {
      buffer.truncate(len_before_insert);
    } else {
      foreign_key_value_cb(&mut *buffer)?;
      truncate_if_ends_with_char(buffer, ',');
      buffer_try_push_str(buffer, ");")?;
    }
    Ok(())
  }
}
