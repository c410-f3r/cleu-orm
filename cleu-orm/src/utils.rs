use crate::{FullTableAssociation, Suffix};
use core::fmt::Arguments;

/// Shortcut of `buffer.try_push(...)`
#[inline]
pub fn buffer_try_push_str<B, E>(buffer: &mut B, string: &str) -> Result<(), E>
where
  B: cl_traits::String,
  E: From<crate::Error>,
{
  buffer.push(string).map_err(|err| E::from(err.into()))
}

/// Shortcut of `buffer.write_fmt(...)`
#[inline]
pub fn buffer_write_fmt<B, E>(buffer: &mut B, args: Arguments<'_>) -> Result<(), E>
where
  B: cl_traits::String,
  E: From<crate::Error>,
{
  buffer.write_fmt(args).map_err(|err| E::from(err.into()))
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
  B: cl_traits::String,
{
  buffer.write_fmt(format_args!("{table}{suffix}__{field}",))?;
  Ok(())
}

#[inline]
pub(crate) fn truncate_if_ends_with_char<B>(buffer: &mut B, c: char)
where
  B: cl_traits::String,
{
  if buffer.as_ref().ends_with(c) {
    buffer.truncate(buffer.as_ref().len().wrapping_sub(1))
  }
}

#[inline]
pub(crate) fn truncate_if_ends_with_str<B>(buffer: &mut B, s: &str)
where
  B: cl_traits::String,
{
  if buffer.as_ref().ends_with(s) {
    buffer.truncate(buffer.as_ref().len().wrapping_sub(s.len()))
  }
}

#[inline]
pub(crate) fn write_full_select_field<B>(
  buffer: &mut B,
  table: &str,
  table_alias: Option<&str>,
  suffix: Suffix,
  field: &str,
) -> crate::Result<()>
where
  B: cl_traits::String,
{
  let actual_table = table_alias.unwrap_or(table);
  write_select_field(buffer, table, table_alias, suffix, field)?;
  buffer.write_fmt(format_args!(" AS {actual_table}{suffix}__{field}"))?;
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
  B: cl_traits::String,
{
  let actual_table = table_alias.unwrap_or(table);
  buffer.write_fmt(format_args!("\"{actual_table}{suffix}\".{field}"))?;
  Ok(())
}

#[inline]
pub(crate) fn write_select_join<B>(
  buffer: &mut B,
  from_table: &str,
  from_table_suffix: Suffix,
  full_association: FullTableAssociation,
) -> crate::Result<()>
where
  B: cl_traits::String,
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
  B: cl_traits::String,
{
  let actual_table = table_alias.unwrap_or(table);
  buffer.write_fmt(format_args!("\"{actual_table}{suffix}\".{field}",))?;
  Ok(())
}
