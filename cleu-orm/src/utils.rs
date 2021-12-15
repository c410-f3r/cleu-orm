use crate::{Buffer, FromRowsSuffix, FullAssociation, SqlWriter, Suffix, TableParams};
use core::fmt::Arguments;
use sqlx_core::{
  postgres::{PgPool, PgRow},
  query::query,
  row::Row,
};

/// Shortcut of `buffer.try_push(...)`
#[inline]
pub fn buffer_try_push_str<B, E>(buffer: &mut B, string: &str) -> Result<(), E>
where
  B: Buffer,
  E: From<crate::Error>,
{
  buffer.push(string).map_err(|err| E::from(err.into()))
}

/// Shortcut of `buffer.write_fmt(...)`
#[inline]
pub fn buffer_write_fmt<B, E>(buffer: &mut B, args: Arguments<'_>) -> Result<(), E>
where
  B: Buffer,
  E: From<crate::Error>,
{
  buffer.write_fmt(args).map_err(|err| E::from(err.into()))
}

/// Auxiliary method that gets all stored entities filtered by a field.
#[inline]
pub async fn read_all<B, R, T>(
  buffer: &mut B,
  pool: &PgPool,
  table_params: &T,
) -> Result<Vec<R>, T::Error>
where
  B: Buffer,
  R: FromRowsSuffix<B, Error = T::Error>,
  T: TableParams,
  T::Associations: SqlWriter<B, Error = T::Error>,
  T::Error: From<crate::Error>,
{
  table_params.write_select(buffer, &mut |_| Ok(()))?;
  let rows = query(buffer.as_ref()).fetch_all(pool).await.map_err(|err| err.into())?;
  buffer.clear();
  collect_entities_tables(buffer, &rows, table_params)
}

/// Auxiliary method that gets all stored entities filtered by a field.
#[inline]
pub async fn read_all_with_where<B, R, T>(
  buffer: &mut B,
  pool: &PgPool,
  table_params: &T,
  where_str: &str,
) -> Result<Vec<R>, T::Error>
where
  B: Buffer,
  R: FromRowsSuffix<B, Error = T::Error>,
  T: TableParams,
  T::Associations: SqlWriter<B, Error = T::Error>,
  T::Error: From<crate::Error>,
{
  table_params.write_select(buffer, &mut |b| buffer_try_push_str(b, where_str))?;
  let rows = query(buffer.as_ref()).fetch_all(pool).await.map_err(|err| err.into())?;
  buffer.clear();
  collect_entities_tables(buffer, &rows, table_params)
}

/// Auxiliary method that only gets a single entity based on its id.
#[inline]
pub async fn read_by_id<B, T>(
  buffer: &mut B,
  id: &T::IdValue,
  pool: &PgPool,
  table_params: &T,
) -> Result<T::Table, T::Error>
where
  B: Buffer,
  T: TableParams,
  T::Associations: SqlWriter<B, Error = T::Error>,
  T::Error: From<crate::Error>,
  T::Table: FromRowsSuffix<B, Error = T::Error>,
{
  table_params.write_select(buffer, &mut |b| {
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

/// Only seeks all rows related to the `T` entity and stops as soon as the primary key changes.
#[inline]
pub fn seek_entity_tables<B, R, T>(
  buffer: &mut B,
  rows: &[PgRow],
  table_params: &T,
  mut cb: impl FnMut(R) -> Result<(), T::Error>,
) -> Result<usize, T::Error>
where
  B: Buffer,
  T: TableParams,
  R: FromRowsSuffix<B, Error = T::Error>,
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

/// Writes {table}{suffix}__{field}` into a buffer.
#[inline]
pub fn write_column_alias<B>(
  buffer: &mut B,
  table: &str,
  suffix: Suffix,
  field: &str,
) -> crate::Result<()>
where
  B: Buffer,
{
  buffer.write_fmt(format_args!("{table}{suffix}__{field}",))?;
  Ok(())
}

#[inline]
pub(crate) fn write_select_field<B>(
  buffer: &mut B,
  table: &str,
  table_alias: Option<&str>,
  suffix: Suffix,
  field: &str,
) -> crate::Result<()>
where
  B: Buffer,
{
  let actual_table = table_alias.unwrap_or(table);
  write_table_field(buffer, table, table_alias, suffix, field)?;
  buffer.write_fmt(format_args!(" AS {actual_table}{suffix}__{field}"))?;
  Ok(())
}

#[inline]
pub(crate) fn write_select_join<B>(
  buffer: &mut B,
  from_table: &str,
  from_table_suffix: Suffix,
  full_association: FullAssociation<'_>,
) -> crate::Result<()>
where
  B: Buffer,
{
  let association = full_association.association();
  buffer.write_fmt(format_args!(
    "LEFT JOIN \"{table_relationship}\" AS \"{table_relationship_alias}{to_table_suffix}\" ON \
     \"{from_table}{from_table_suffix}\".{table_id} = \
     \"{table_relationship_alias}{to_table_suffix}\".{table_relationship_id}",
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
pub(crate) fn write_select_order_by<B>(
  buffer: &mut B,
  table: &str,
  table_alias: Option<&str>,
  suffix: Suffix,
  field: &str,
) -> crate::Result<()>
where
  B: Buffer,
{
  let actual_table = table_alias.unwrap_or(table);
  buffer.write_fmt(format_args!("\"{actual_table}{suffix}\".{field}",))?;
  Ok(())
}

/// Collects all entities composed by all different rows.
///
/// One entity can constructed by more than one row.
#[inline]
fn collect_entities_tables<B, T, R>(
  buffer: &mut B,
  rows: &[PgRow],
  table_params: &T,
) -> Result<Vec<R>, T::Error>
where
  B: Buffer,
  R: FromRowsSuffix<B, Error = T::Error>,
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

#[inline]
pub(crate) fn write_table_field<B>(
  buffer: &mut B,
  table: &str,
  table_alias: Option<&str>,
  suffix: Suffix,
  field: &str,
) -> crate::Result<()>
where
  B: Buffer,
{
  let actual_table = table_alias.unwrap_or(table);
  buffer.write_fmt(format_args!("\"{actual_table}{suffix}\".{field}"))?;
  Ok(())
}
