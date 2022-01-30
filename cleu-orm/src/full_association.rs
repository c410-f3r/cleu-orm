use crate::{Association, Suffix};

/// Contains [Association] plus some parameters gathered from other sources
#[derive(Debug)]
pub struct FullAssociation<'association> {
  association: &'association Association,
  to_table: &'static str,
  to_table_alias: Option<&'static str>,
  to_table_suffix: Suffix,
}

impl<'association> FullAssociation<'association> {
  #[inline]
  pub(crate) const fn new(
    association: &'association Association,
    to_table: &'static str,
    to_table_alias: Option<&'static str>,
    to_table_suffix: Suffix,
  ) -> Self {
    Self { association, to_table, to_table_alias, to_table_suffix }
  }

  /// See [Association].
  #[inline]
  pub const fn association(&self) -> &&'association Association {
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
  pub const fn to_table_suffix(&self) -> Suffix {
    self.to_table_suffix
  }
}
