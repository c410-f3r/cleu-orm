use crate::{
  buffer_try_push_str, buffer_write_fmt,
  crud::{TdEntity, TdError},
  write_column_alias, write_select_field, FromRowsSuffix, SelectLimit, SelectOrderBy, SqlWriter,
  Suffix, Table, TableDefs,
};
use sqlx_core::{
  postgres::{PgPool, PgRow},
  query::query,
  row::Row,
};

/// Seeks all rows that equals `TD`'s primary key and suffix. Can be `TD` itself or any other
/// associated/related entity.
#[inline]
pub fn seek_related_entities<'entity, B, F, R, TD>(
  buffer: &mut B,
  rows: &[PgRow],
  suffix: Suffix,
  suffix_related: Suffix,
  mut cb: F,
) -> Result<usize, TD::Error>
where
  B: cl_traits::String,
  F: FnMut(R) -> Result<(), TD::Error>,
  R: FromRowsSuffix<B, Error = TD::Error>,
  TD: TableDefs<'entity>,
{
  if rows.is_empty() {
    return Ok(0);
  }

  let first_row = if let Some(elem) = rows.first() {
    elem
  } else {
    return Ok(0);
  };

  let first_rslt = R::from_rows_suffix(rows, buffer, suffix_related, first_row);
  let (mut counter, mut previous) = if let Ok((skip, entity)) = first_rslt {
    write_column_alias(buffer, TD::TABLE_NAME, suffix, TD::PRIMARY_KEY_NAME)?;
    let previous = first_row.try_get(buffer.as_ref()).map_err(Into::into)?;
    buffer.clear();
    cb(entity)?;
    (skip, previous)
  } else {
    buffer.clear();
    return Ok(1);
  };

  loop {
    if counter >= rows.len() {
      break;
    }

    let row = if let Some(elem) = rows.get(counter) {
      elem
    } else {
      break;
    };

    let curr_rows = rows.get(counter..).unwrap_or_default();
    let (skip, entity) = R::from_rows_suffix(curr_rows, buffer, suffix_related, row)?;

    write_column_alias(buffer, TD::TABLE_NAME, suffix, TD::PRIMARY_KEY_NAME)?;
    let curr: i64 = row.try_get(buffer.as_ref()).map_err(Into::into)?;
    buffer.clear();
    if previous == curr {
      cb(entity)?;
      counter = counter.wrapping_add(skip);
    } else {
      break;
    }
    previous = curr;
  }

  Ok(counter)
}

#[inline]
pub(crate) async fn read_all<'entity, R, B, TD>(
  buffer: &mut B,
  pool: &PgPool,
  table: &Table<'entity, TD>,
) -> Result<Vec<R>, TdError<'entity, TD>>
where
  B: cl_traits::String,
  R: FromRowsSuffix<B, Error = TD::Error>,
  TD: TableDefs<'entity>,
  TD::Associations: SqlWriter<B, Error = TD::Error>,
  TD::Error: From<crate::Error>,
{
  table.write_select(buffer, SelectOrderBy::Ascending, SelectLimit::All, &mut |_| Ok(()))?;
  let rows = query(buffer.as_ref()).fetch_all(pool).await.map_err(Into::into)?;
  buffer.clear();
  collect_entities_tables(buffer, &rows, table)
}

#[inline]
pub(crate) async fn read_by_id<'entity, B, TD>(
  buffer: &mut B,
  id: &TD::PrimaryKeyValue,
  pool: &PgPool,
  table: &Table<'entity, TD>,
) -> Result<TdEntity<'entity, TD>, TdError<'entity, TD>>
where
  B: cl_traits::String,
  TD: TableDefs<'entity>,
  TD::Associations: SqlWriter<B, Error = TD::Error>,
  TD::Entity: FromRowsSuffix<B, Error = TD::Error>,
  TD::Error: From<crate::Error>,
{
  table.write_select(buffer, SelectOrderBy::Ascending, SelectLimit::All, &mut |b| {
    write_select_field(
      b,
      TD::TABLE_NAME,
      TD::TABLE_NAME_ALIAS,
      table.suffix(),
      table.id_field().name(),
    )?;
    buffer_write_fmt(b, format_args!(" = {id}"))
  })?;
  let rows = query(buffer.as_ref()).fetch_all(pool).await.map_err(Into::into)?;
  buffer.clear();
  let first_row = rows.first().ok_or(crate::Error::NoDatabaseRowResult)?;
  Ok(TD::Entity::from_rows_suffix(&rows, buffer, table.suffix(), first_row)?.1)
}

#[inline]
pub(crate) async fn read_all_with_params<'entity, R, B, TD>(
  buffer: &mut B,
  pool: &PgPool,
  table: &Table<'entity, TD>,
  order_by: SelectOrderBy,
  select_limit: SelectLimit,
  where_str: &str,
) -> Result<Vec<R>, TdError<'entity, TD>>
where
  B: cl_traits::String,
  R: FromRowsSuffix<B, Error = TD::Error>,
  TD: TableDefs<'entity>,
  TD::Associations: SqlWriter<B, Error = TD::Error>,
  TD::Error: From<crate::Error>,
{
  table.write_select(buffer, order_by, select_limit, &mut |b| buffer_try_push_str(b, where_str))?;
  let rows = query(buffer.as_ref()).fetch_all(pool).await.map_err(Into::into)?;
  buffer.clear();
  collect_entities_tables(buffer, &rows, table)
}

/// Collects all entities composed by all different rows.
///
/// One entity can constructed by more than one row.
#[inline]
fn collect_entities_tables<'entity, R, B, TD>(
  buffer: &mut B,
  rows: &[PgRow],
  table: &Table<'entity, TD>,
) -> Result<Vec<R>, TD::Error>
where
  B: cl_traits::String,
  R: FromRowsSuffix<B, Error = TD::Error>,
  TD: TableDefs<'entity>,
{
  let mut rslt = Vec::new();
  let mut counter: usize = 0;

  loop {
    if counter >= rows.len() {
      break;
    }
    let actual_rows = rows.get(counter..).unwrap_or_default();
    let skip = seek_related_entities::<_, _, _, TD>(
      buffer,
      actual_rows,
      table.suffix(),
      table.suffix(),
      |entity| {
        rslt.push(entity);
        Ok(())
      },
    )?;
    counter = counter.wrapping_add(skip);
  }

  Ok(rslt)
}
