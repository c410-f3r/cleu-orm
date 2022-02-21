use crate::{
  buffer_try_push_str, buffer_write_fmt, write_column_alias, write_table_field, FromRowsSuffix,
  Limit, OrderBy, SqlWriter, TableParams,
};
use sqlx_core::{
  postgres::{PgPool, PgRow},
  query::query,
  row::Row,
};

/// Only seeks all rows related to the `T` entity and stops as soon as the primary key changes.
#[inline]
pub fn seek_entity_tables<R, S, T>(
  buffer: &mut S,
  rows: &[PgRow],
  table_params: &T,
  mut cb: impl FnMut(R) -> Result<(), T::Error>,
) -> Result<usize, T::Error>
where
  R: FromRowsSuffix<S, Error = T::Error>,
  S: cl_traits::String,
  T: TableParams,
{
  if rows.is_empty() {
    return Ok(0);
  }

  let first_row = if let Some(elem) = rows.first() {
    elem
  } else {
    return Ok(0);
  };

  let mut counter: usize = 0;
  let mut previous: i64;

  if let Ok((skip, table)) = R::from_rows_suffix(rows, buffer, table_params.suffix(), first_row) {
    write_column_alias(
      buffer,
      T::table_name(),
      table_params.suffix(),
      table_params.id_field().name(),
    )?;
    previous = first_row.try_get(buffer.as_ref()).map_err(|err| err.into())?;
    buffer.clear();
    cb(table)?;
    counter = counter.wrapping_add(skip);
  } else {
    buffer.clear();
    return Ok(1);
  }

  loop {
    if counter >= rows.len() {
      break;
    }

    let row = if let Some(elem) = rows.get(counter) {
      elem
    } else {
      break;
    };

    let (skip, table) = R::from_rows_suffix(
      rows.get(counter..).unwrap_or_default(),
      buffer,
      table_params.suffix(),
      row,
    )?;

    write_column_alias(
      buffer,
      T::table_name(),
      table_params.suffix(),
      table_params.id_field().name(),
    )?;
    let curr: i64 = row.try_get(buffer.as_ref()).map_err(|err| err.into())?;
    buffer.clear();
    if previous == curr {
      cb(table)?;
      counter = counter.wrapping_add(skip);
    } else {
      break;
    }
    previous = curr;
  }

  Ok(counter)
}

#[inline]
pub(crate) async fn read_all<R, S, T>(
  buffer: &mut S,
  pool: &PgPool,
  table_params: &T,
) -> Result<Vec<R>, T::Error>
where
  R: FromRowsSuffix<S, Error = T::Error>,
  S: cl_traits::String,
  T: TableParams,
  T::Associations: SqlWriter<S, Error = T::Error>,
  T::Error: From<crate::Error>,
{
  table_params.write_select(buffer, OrderBy::Ascending, Limit::All, &mut |_| Ok(()))?;
  let rows = query(buffer.as_ref()).fetch_all(pool).await.map_err(|err| err.into())?;
  buffer.clear();
  collect_entities_tables(buffer, &rows, table_params)
}

#[inline]
pub(crate) async fn read_by_id<S, T>(
  buffer: &mut S,
  id: &T::IdValue,
  pool: &PgPool,
  table_params: &T,
) -> Result<T::Table, T::Error>
where
  S: cl_traits::String,
  T: TableParams,
  T::Associations: SqlWriter<S, Error = T::Error>,
  T::Error: From<crate::Error>,
  T::Table: FromRowsSuffix<S, Error = T::Error>,
{
  table_params.write_select(buffer, OrderBy::Ascending, Limit::All, &mut |b| {
    write_table_field(
      b,
      T::table_name(),
      T::table_name_alias(),
      table_params.suffix(),
      table_params.id_field().name(),
    )?;
    buffer_write_fmt(b, format_args!(" = {id}"))
  })?;
  let rows = query(buffer.as_ref()).fetch_all(pool).await.map_err(|err| err.into())?;
  buffer.clear();
  let first_row = rows.first().ok_or(crate::Error::NoDatabaseRowResult)?;
  Ok(T::Table::from_rows_suffix(&rows, buffer, table_params.suffix(), first_row)?.1)
}

#[inline]
pub(crate) async fn read_all_with_params<R, S, T>(
  buffer: &mut S,
  pool: &PgPool,
  table_params: &T,
  order_by: OrderBy,
  limit: Limit,
  where_str: &str,
) -> Result<Vec<R>, T::Error>
where
  R: FromRowsSuffix<S, Error = T::Error>,
  S: cl_traits::String,
  T: TableParams,
  T::Associations: SqlWriter<S, Error = T::Error>,
  T::Error: From<crate::Error>,
{
  table_params.write_select(buffer, order_by, limit, &mut |b| buffer_try_push_str(b, where_str))?;
  let rows = query(buffer.as_ref()).fetch_all(pool).await.map_err(|err| err.into())?;
  buffer.clear();
  collect_entities_tables(buffer, &rows, table_params)
}

/// Collects all entities composed by all different rows.
///
/// One entity can constructed by more than one row.
#[inline]
fn collect_entities_tables<R, S, T>(
  buffer: &mut S,
  rows: &[PgRow],
  table_params: &T,
) -> Result<Vec<R>, T::Error>
where
  R: FromRowsSuffix<S, Error = T::Error>,
  S: cl_traits::String,
  T: TableParams,
{
  let mut rslt = Vec::new();
  let mut counter: usize = 0;

  loop {
    if counter >= rows.len() {
      break;
    }
    let actual_rows = rows.get(counter..).unwrap_or_default();
    let skip = seek_entity_tables(buffer, actual_rows, table_params, |table| {
      rslt.push(table);
      Ok(())
    })?;
    counter = counter.wrapping_add(skip);
  }

  Ok(rslt)
}
