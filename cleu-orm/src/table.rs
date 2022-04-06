use crate::{FxHasher, Suffix, TableDefs, TableField, MAX_NODES_NUM};
use core::{
  hash::{Hash, Hasher},
  marker::PhantomData,
};

/// A wrapper of instance values build based on [TableDefs].
#[derive(Debug, PartialEq)]
pub struct Table<'entity, TD>
where
  TD: TableDefs<'entity>,
{
  associations: TD::Associations,
  fields: TD::Fields,
  id_field: TableField<TD::Error, TD::PrimaryKeyValue>,
  phantom: PhantomData<TD>,
  suffix: Suffix,
}

impl<'entity, TD> Table<'entity, TD>
where
  TD: TableDefs<'entity>,
{
  /// A new instance with all related table definition values created automatically.
  #[inline]
  pub fn new(suffix: Suffix) -> Self {
    let (associations, fields) = TD::type_instances(suffix);
    Self {
      associations,
      fields,
      id_field: TableField::new(TD::PRIMARY_KEY_NAME),
      phantom: PhantomData,
      suffix,
    }
  }

  /// Table instance associations
  #[inline]
  pub fn associations(&self) -> &TD::Associations {
    &self.associations
  }

  /// Mutable version of [associations]
  #[inline]
  pub fn associations_mut(&mut self) -> &mut TD::Associations {
    &mut self.associations
  }

  /// Table instance fields
  #[inline]
  pub fn fields(&self) -> &TD::Fields {
    &self.fields
  }

  /// Mutable version of [fields]
  #[inline]
  pub fn fields_mut(&mut self) -> &mut TD::Fields {
    &mut self.fields
  }

  /// Field information related to the entity ID
  #[inline]
  pub fn id_field(&self) -> &TableField<TD::Error, TD::PrimaryKeyValue> {
    &self.id_field
  }

  /// Mutable version of [id_field]
  #[inline]
  pub fn id_field_mut(&mut self) -> &mut TableField<TD::Error, TD::PrimaryKeyValue> {
    &mut self.id_field
  }

  /// Used to write internal SQL operations
  #[inline]
  pub fn suffix(&self) -> Suffix {
    self.suffix
  }

  /// Shortcut for `<T as TableDefs<'_>>::update_all_table_fields(&entity, &mut table)`
  #[inline]
  pub fn update_all_table_fields(&mut self, entity: &'entity TD::Entity) {
    TD::update_all_table_fields(entity, self)
  }

  #[inline]
  pub(crate) fn instance_idx(&self) -> usize {
    let mut fx_hasher = FxHasher::default();
    TD::PRIMARY_KEY_NAME.hash(&mut fx_hasher);
    TD::TABLE_NAME.hash(&mut fx_hasher);
    TD::TABLE_NAME_ALIAS.hash(&mut fx_hasher);
    self.id_field().value().hash(&mut fx_hasher);
    let [a, b, c, d, e, f, g, h] = fx_hasher.finish().to_ne_bytes();
    #[cfg(target_pointer_width = "16")]
    let rslt = usize::from_ne_bytes([
      a.wrapping_add(b).wrapping_add(c).wrapping_add(d),
      e.wrapping_add(f).wrapping_add(g).wrapping_add(h),
    ]);
    #[cfg(target_pointer_width = "32")]
    let rslt = usize::from_ne_bytes([
      a.wrapping_add(b),
      c.wrapping_add(d),
      e.wrapping_add(f),
      g.wrapping_add(h),
    ]);
    #[cfg(target_pointer_width = "64")]
    let rslt = usize::from_ne_bytes([a, b, c, d, e, f, g, h]);
    rslt.wrapping_rem(MAX_NODES_NUM)
  }
}

impl<'entity, TD> Default for Table<'entity, TD>
where
  TD: TableDefs<'entity>,
{
  #[inline]
  fn default() -> Self {
    Self::new(0)
  }
}
