use crate::{FromRowsSuffix, FullAssociation, SqlWriter, TableParams};
use arrayvec::ArrayString;
use core::{
  fmt::{self, Arguments, Write},
  marker::Unpin,
};
use sqlx_core::{
  postgres::{PgPool, PgRow},
  query::query,
  row::Row,
};

/// Auxiliary method that gets all stored entities.
#[inline]
pub async fn read_all<R, T, const N: usize>(
  buffer: &mut ArrayString<N>,
  table_params: &T,
  pool: &PgPool,
) -> Result<Vec<R>, T::Error>
where
  R: FromRowsSuffix<N, Error = T::Error> + Send + Unpin,
  T: TableParams,
  T::Associations: SqlWriter<N, Error = T::Error>,
  T::Error: From<crate::Error>,
{
  table_params.write_select(buffer, "")?;
  let rows = query(buffer.as_str()).fetch_all(pool).await.map_err(|err| err.into())?;
  buffer.clear();
  collect_entities_tables(buffer, &rows, table_params)
}

/// Auxiliary method that only gets a single entity based on its id.
#[inline]
pub async fn read_by_id<F, T, const N: usize>(
  buffer: &mut ArrayString<N>,
  id: F,
  pool: &PgPool,
  table_params: &T,
) -> Result<T::Table, T::Error>
where
  F: fmt::Display,
  T: TableParams,
  T::Associations: SqlWriter<N, Error = T::Error>,
  T::Error: From<crate::Error>,
  T::Table: FromRowsSuffix<N, Error = T::Error> + Send + Unpin,
{
  let where_str_rslt = ArrayString::<128>::try_from(format_args!(
    " WHERE \"{table}{suffix}\".{id_name} = {id_value}",
    id_name = table_params.id_field(),
    id_value = id,
    table = T::table_name(),
    suffix = table_params.suffix()
  ));
  table_params.write_select(buffer, where_str_rslt.map_err(|err| err.into())?.as_str())?;
  let rows = query(buffer.as_str()).fetch_all(pool).await.map_err(|err| err.into())?;
  buffer.clear();
  let first_row = rows.first().ok_or(crate::Error::NoDatabaseRowResult)?;
  Ok(T::Table::from_rows_suffix(&rows, buffer, table_params.suffix(), first_row)?.1)
}

/// Only seeks all rows related to the `T` entity and stops as soon as the primary key changes.
#[inline]
pub fn seek_entity_tables<R, T, const N: usize>(
  buffer: &mut ArrayString<N>,
  rows: &[PgRow],
  table_params: &T,
  mut cb: impl FnMut(R) -> Result<(), T::Error>,
) -> Result<usize, T::Error>
where
  T: TableParams,
  R: FromRowsSuffix<N, Error = T::Error>,
{
  let mut counter: usize = 0;

  if counter >= rows.len() {
    return Ok(counter);
  }

  let first_row = if let Some(elem) = rows.get(counter) {
    elem
  } else {
    return Ok(counter);
  };

  let mut previous: i64;

  if let Ok((skip, table)) = R::from_rows_suffix(rows, buffer, table_params.suffix(), first_row) {
    write_column_alias(buffer, T::table_name(), table_params.suffix(), table_params.id_field())?;
    previous = first_row.try_get(buffer.as_str()).map_err(|err| err.into())?;
    buffer.clear();
    cb(table)?;
    counter = counter.wrapping_add(skip);
  } else {
    return Ok(counter);
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

    let table_rslt = R::from_rows_suffix(
      rows.get(counter..).unwrap_or_default(),
      buffer,
      table_params.suffix(),
      row,
    );

    if let Ok((skip, table)) = table_rslt {
      write_column_alias(buffer, T::table_name(), table_params.suffix(), table_params.id_field())?;
      let curr: i64 = row.try_get(buffer.as_str()).map_err(|err| err.into())?;
      buffer.clear();
      if previous == curr {
        cb(table)?;
        counter = counter.wrapping_add(skip);
      } else {
        break;
      }
      previous = curr;
    }
  }

  Ok(counter)
}

/// Writes {table}{suffix}__{field}` into a buffer.
#[inline]
pub fn write_column_alias<const N: usize>(
  buffer: &mut ArrayString<N>,
  table: &str,
  suffix: u8,
  field: &str,
) -> crate::Result<()> {
  buffer.write_fmt(format_args!(
    "{table}{suffix}__{field}",
    field = field,
    suffix = suffix,
    table = table
  ))?;
  Ok(())
}

#[inline]
pub(crate) fn buffer_try_push<E, const N: usize>(
  buffer: &mut ArrayString<N>,
  char: char,
) -> Result<(), E>
where
  E: From<crate::Error>,
{
  buffer.try_push(char).map_err(|err| E::from(err.into()))
}

#[inline]
pub(crate) fn buffer_try_push_str<E, const N: usize>(
  buffer: &mut ArrayString<N>,
  string: &str,
) -> Result<(), E>
where
  E: From<crate::Error>,
{
  buffer.try_push_str(string).map_err(|err| E::from(err.into()))
}

#[inline]
pub(crate) fn buffer_write_fmt<E, const N: usize>(
  buffer: &mut ArrayString<N>,
  args: Arguments<'_>,
) -> Result<(), E>
where
  E: From<crate::Error>,
{
  buffer.write_fmt(args).map_err(|err| E::from(err.into()))
}

#[inline]
pub(crate) fn write_select_field<const N: usize>(
  buffer: &mut ArrayString<N>,
  table: &str,
  table_alias: Option<&str>,
  suffix: u8,
  field: &str,
) -> crate::Result<()> {
  let actual_table = table_alias.unwrap_or(table);
  buffer.write_fmt(format_args!(
    "\"{actual_table}{suffix}\".{field} AS {actual_table}{suffix}__{field}",
    field = field,
    suffix = suffix,
    actual_table = actual_table
  ))?;
  Ok(())
}

#[inline]
pub(crate) fn write_select_join<const N: usize>(
  buffer: &mut ArrayString<N>,
  from_table: &str,
  from_table_suffix: u8,
  full_association: FullAssociation<'_>,
) -> crate::Result<()> {
  let association = full_association.association();
  buffer.write_fmt(format_args!(
    "LEFT JOIN \"{table_relationship}\" AS \"{table_relationship_alias}{to_table_suffix}\" ON \
     \"{from_table}{from_table_suffix}\".{table_id} = \
     \"{table_relationship_alias}{to_table_suffix}\".{table_relationship_id}",
    from_table = from_table,
    from_table_suffix = from_table_suffix,
    table_id = association.from_id(),
    table_relationship = full_association.to_table(),
    table_relationship_alias =
      full_association.to_table_alias().unwrap_or_else(|| full_association.to_table()),
    table_relationship_id = association.to_id(),
    to_table_suffix = full_association.to_table_suffix(),
  ))?;
  Ok(())
}

#[inline]
pub(crate) fn write_select_order_by<const N: usize>(
  buffer: &mut ArrayString<N>,
  table: &str,
  table_alias: Option<&str>,
  suffix: u8,
  field: &str,
) -> crate::Result<()> {
  let actual_table = table_alias.unwrap_or(table);
  buffer.write_fmt(format_args!(
    "\"{actual_table}{suffix}\".{field}",
    field = field,
    suffix = suffix,
    actual_table = actual_table
  ))?;
  Ok(())
}

/// Collects all entities composed by all different rows.
///
/// One entity can constructed by more than one row.
#[inline]
fn collect_entities_tables<T, R, const N: usize>(
  buffer: &mut ArrayString<N>,
  rows: &[PgRow],
  table_params: &T,
) -> Result<Vec<R>, T::Error>
where
  R: FromRowsSuffix<N, Error = T::Error>,
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
