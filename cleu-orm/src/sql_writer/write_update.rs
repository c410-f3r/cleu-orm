use crate::{
  buffer_try_push_str, buffer_write_fmt, node_was_already_visited, sql_writer::SqlWriterLogic,
  truncate_if_ends_with_char, AuxNodes, SqlValue, SqlWriter, Table, TableDefs, TableFields,
};

impl<'entity, B, TD> SqlWriterLogic<'entity, B, TD>
where
  B: cl_traits::String,
  TD: TableDefs<'entity>,
  TD::Associations: SqlWriter<B, Error = TD::Error>,
  TD::Error: From<crate::Error>,
{
  #[inline]
  pub(crate) fn write_update(
    aux: &mut AuxNodes,
    buffer: &mut B,
    table: &Table<'entity, TD>,
  ) -> Result<(), TD::Error> {
    if node_was_already_visited(aux, table)? {
      return Ok(());
    }
    Self::write_update_manager(buffer, table)?;
    table.associations().write_update(aux, buffer)?;
    Ok(())
  }

  fn write_update_manager(buffer: &mut B, table: &Table<'entity, TD>) -> Result<(), TD::Error> {
    let id_value = if let &Some(ref el) = table.id_field().value() { el } else { return Ok(()) };

    buffer_write_fmt(buffer, format_args!("UPDATE {} SET ", TD::TABLE_NAME))?;

    buffer_write_fmt(buffer, format_args!("{}=", table.id_field().name()))?;
    id_value.write(buffer)?;
    buffer_try_push_str(buffer, ",")?;
    table.fields().write_update_values(buffer)?;
    truncate_if_ends_with_char(buffer, ',');

    buffer_try_push_str(buffer, " WHERE ")?;
    buffer_write_fmt(buffer, format_args!("{}=", TD::PRIMARY_KEY_NAME))?;
    id_value.write(buffer)?;
    buffer_try_push_str(buffer, ";")?;

    Ok(())
  }
}
