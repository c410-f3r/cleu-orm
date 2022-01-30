use crate::{FullAssociation, Suffix};
use core::fmt;

/// Shortcut of `buffer.try_push(...)`
#[inline]
pub fn buffer_try_push_str<E, S>(buffer: &mut S, string: &str) -> Result<(), E>
where
  E: From<crate::Error>,
  S: cl_traits::String,
{
  buffer.push(string).map_err(|err| E::from(err.into()))
}

/// Shortcut of `buffer.write_fmt(...)`
#[inline]
pub fn buffer_write_fmt<E, S>(buffer: &mut S, args: fmt::Arguments<'_>) -> Result<(), E>
where
  E: From<crate::Error>,
  S: cl_traits::String,
{
  buffer.write_fmt(args).map_err(|err| E::from(err.into()))
}

/// Writes {table}{suffix}__{field}` into a buffer.
#[inline]
pub fn write_column_alias<S>(
  buffer: &mut S,
  table: &str,
  suffix: Suffix,
  field: &str,
) -> crate::Result<()>
where
  S: cl_traits::String,
{
  buffer.write_fmt(format_args!("{table}{suffix}__{field}",))?;
  Ok(())
}

#[inline]
pub(crate) fn write_select_field<S>(
  buffer: &mut S,
  table: &str,
  table_alias: Option<&str>,
  suffix: Suffix,
  field: &str,
) -> crate::Result<()>
where
  S: cl_traits::String,
{
  let actual_table = table_alias.unwrap_or(table);
  write_table_field(buffer, table, table_alias, suffix, field)?;
  buffer.write_fmt(format_args!(" AS {actual_table}{suffix}__{field}"))?;
  Ok(())
}

#[inline]
pub(crate) fn write_select_join<S>(
  buffer: &mut S,
  from_table: &str,
  from_table_suffix: Suffix,
  full_association: FullAssociation<'_>,
) -> crate::Result<()>
where
  S: cl_traits::String,
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
pub(crate) fn write_select_order_by<S>(
  buffer: &mut S,
  table: &str,
  table_alias: Option<&str>,
  suffix: Suffix,
  field: &str,
) -> crate::Result<()>
where
  S: cl_traits::String,
{
  let actual_table = table_alias.unwrap_or(table);
  buffer.write_fmt(format_args!("\"{actual_table}{suffix}\".{field}",))?;
  Ok(())
}

#[inline]
pub(crate) fn write_table_field<S>(
  buffer: &mut S,
  table: &str,
  table_alias: Option<&str>,
  suffix: Suffix,
  field: &str,
) -> crate::Result<()>
where
  S: cl_traits::String,
{
  let actual_table = table_alias.unwrap_or(table);
  buffer.write_fmt(format_args!("\"{actual_table}{suffix}\".{field}"))?;
  Ok(())
}
