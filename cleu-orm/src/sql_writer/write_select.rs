use crate::{
  buffer_try_push_str, buffer_write_fmt, sql_writer::SqlWriterLogic, truncate_if_ends_with_char,
  truncate_if_ends_with_str, SelectLimit, SelectOrderBy, SqlWriter, Table, TableDefs,
};

impl<'entity, B, TD> SqlWriterLogic<'entity, B, TD>
where
  B: cl_traits::String,
  TD: TableDefs<'entity>,
  TD::Associations: SqlWriter<B, Error = TD::Error>,
  TD::Error: From<crate::Error>,
{
  #[inline]
  pub(crate) fn write_select(
    buffer: &mut B,
    order_by: SelectOrderBy,
    select_limit: SelectLimit,
    table: &Table<'entity, TD>,
    where_cb: &mut impl FnMut(&mut B) -> Result<(), TD::Error>,
  ) -> Result<(), TD::Error> {
    buffer_try_push_str(buffer, "SELECT ")?;
    table.write_select_fields(buffer)?;
    truncate_if_ends_with_char(buffer, ',');
    buffer_write_fmt(
      buffer,
      format_args!(
        " FROM \"{table}\" AS \"{table}{suffix}\" ",
        suffix = table.suffix(),
        table = TD::TABLE_NAME
      ),
    )?;
    table.write_select_associations(buffer)?;
    buffer_try_push_str(buffer, " WHERE ")?;
    where_cb(buffer)?;
    truncate_if_ends_with_str(buffer, " WHERE ");
    buffer_try_push_str(buffer, " ORDER BY ")?;
    table.write_select_orders_by(buffer)?;
    truncate_if_ends_with_char(buffer, ',');
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
}
