use crate::{Associations, Field, Fields, FxHasher, Suffix, MAX_NODES_NUM};
use core::{
  fmt,
  hash::{Hash, Hasher},
};

/// All SQL parameters related to [TableParams::Table].
pub trait TableParams {
  /// Table associations
  type Associations: Associations;
  /// See [crate::Error]
  type Error: From<crate::Error>;
  /// Table field
  type Fields: Fields<Error = Self::Error>;
  /// Table id value
  type IdValue: Copy + Hash + fmt::Display;
  /// Target table
  type Table;

  /// Table instance associations
  fn associations(&self) -> &Self::Associations;

  /// Mutable version of [associations]
  fn associations_mut(&mut self) -> &mut Self::Associations;

  /// Table instance fields
  fn fields(&self) -> &Self::Fields;

  /// Mutable version of [fields]
  fn fields_mut(&mut self) -> &mut Self::Fields;

  /// The instance field intended to be the id
  fn id_field(&self) -> &Field<Self::Error, Self::IdValue>;

  /// Index used for internal operations
  #[inline]
  fn instance_idx(&self) -> usize {
    let mut fx_hasher = FxHasher::default();
    Self::table_name().hash(&mut fx_hasher);
    self.id_field().value().hash(&mut fx_hasher);
    let opt: Option<usize> = fx_hasher.finish().try_into().ok();
    opt.unwrap_or_default().wrapping_rem(MAX_NODES_NUM)
  }

  /// Used to write SQL operations
  fn suffix(&self) -> Suffix;

  /// Table name
  fn table_name() -> &'static str;

  /// Optional table name alias
  #[inline]
  fn table_name_alias() -> Option<&'static str> {
    None
  }
}

impl<'a, T> TableParams for &'a mut T
where
  T: TableParams,
{
  type Associations = T::Associations;
  type Error = T::Error;
  type Fields = T::Fields;
  type IdValue = T::IdValue;
  type Table = T::Table;

  #[inline]
  fn associations(&self) -> &Self::Associations {
    (**self).associations()
  }

  #[inline]
  fn associations_mut(&mut self) -> &mut Self::Associations {
    (**self).associations_mut()
  }

  #[inline]
  fn fields(&self) -> &Self::Fields {
    (**self).fields()
  }

  #[inline]
  fn fields_mut(&mut self) -> &mut Self::Fields {
    (**self).fields_mut()
  }

  #[inline]
  fn id_field(&self) -> &Field<Self::Error, Self::IdValue> {
    (**self).id_field()
  }

  #[inline]
  fn suffix(&self) -> Suffix {
    (**self).suffix()
  }

  #[inline]
  fn table_name() -> &'static str {
    T::table_name()
  }
}
