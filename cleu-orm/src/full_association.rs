use crate::Association;

/// Contains [Association] plus some parameters gathered from other sources
#[derive(Debug)]
pub struct FullAssociation<'a> {
  association: &'a Association,
  to_table: &'static str,
  to_table_alias: Option<&'static str>,
  to_table_suffix: u8,
}

impl<'a> FullAssociation<'a> {
  #[inline]
  pub(crate) const fn new(
    association: &'a Association,
    to_table: &'static str,
    to_table_alias: Option<&'static str>,
    to_table_suffix: u8,
  ) -> Self {
    Self { association, to_table, to_table_alias, to_table_suffix }
  }

  /// See [Association].
  #[inline]
  pub const fn association(&self) -> &&'a Association {
    &self.association
  }

  /// Referenced table
  #[inline]
  pub const fn to_table(&self) -> &'static str {
    self.to_table
  }

  /// Referenced table alias
  #[inline]
  pub const fn to_table_alias(&self) -> Option<&'static str> {
    self.to_table_alias
  }

  /// Referenced table suffix
  #[inline]
  pub const fn to_table_suffix(&self) -> u8 {
    self.to_table_suffix
  }
}