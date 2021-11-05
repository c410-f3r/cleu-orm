use crate::{Associations, Fields};

/// All SQL parameters related to [TableParams::Table].
pub trait TableParams {
  /// Table associations
  type Associations: Associations;
  /// See [crate::Error]
  type Error: From<crate::Error>;
  /// Table field
  type Fields: Fields;
  /// Target table
  type Table;

  /// Table instance associations
  fn associations(&self) -> &Self::Associations;

  /// Table instance fields
  fn fields(&self) -> &Self::Fields;

  /// The instance field intended to be the id
  fn id_field(&self) -> &str;

  /// Used to write SQL operations
  fn suffix(&self) -> u8;

  /// Table name
  fn table_name() -> &'static str;

  /// Optional table name alias
  #[inline]
  fn table_name_alias() -> Option<&'static str> {
    None
  }
}
