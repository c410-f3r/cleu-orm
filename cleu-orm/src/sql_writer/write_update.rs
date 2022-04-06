use crate::{
  buffer_try_push_str, buffer_write_fmt, sql_writer::SqlWriterLogic, truncate_if_ends_with_char,
  SqlValue, SqlWriter, Table, TableDefs, TableFields, MAX_NODES_NUM,
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
    aux: &mut [Option<&'static str>; MAX_NODES_NUM],
    buffer: &mut B,
    table: &Table<'entity, TD>,
  ) -> Result<(), TD::Error> {
    let idx = table.instance_idx();
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
