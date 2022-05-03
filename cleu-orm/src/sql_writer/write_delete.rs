use crate::{
  buffer_try_push_str, buffer_write_fmt, node_was_already_visited, sql_writer::SqlWriterLogic,
  AuxNodes, SqlValue, SqlWriter, Table, TableDefs,
};

impl<'entity, B, TD> SqlWriterLogic<'entity, B, TD>
where
  B: cl_traits::String,
  TD: TableDefs<'entity>,
  TD::Associations: SqlWriter<B, Error = TD::Error>,
  TD::Error: From<crate::Error>,
{
  #[inline]
  pub(crate) fn write_delete(
    aux: &mut AuxNodes,
    buffer: &mut B,
    table: &Table<'entity, TD>,
  ) -> Result<(), TD::Error> {
    if node_was_already_visited(aux, table)? {
      return Ok(());
    }
    table.associations().write_delete(aux, buffer)?;
    Self::write_delete_manager(buffer, table)?;
    Ok(())
  }

  fn write_delete_manager(buffer: &mut B, table: &Table<'entity, TD>) -> Result<(), TD::Error> {
    let id_value = if let &Some(ref el) = table.id_field().value() { el } else { return Ok(()) };
    buffer_write_fmt(
      buffer,
      format_args!("DELETE FROM {} WHERE {}=", TD::TABLE_NAME, TD::PRIMARY_KEY_NAME),
    )?;
    id_value.write(buffer)?;
    buffer_try_push_str(buffer, ";")?;
    Ok(())
  }
}
