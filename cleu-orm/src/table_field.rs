use crate::SqlValue;
use core::marker::PhantomData;

/// Table field name and its associated Rust type
#[derive(Debug, PartialEq)]
pub struct TableField<E, T> {
  name: &'static str,
  phantom: PhantomData<E>,
  value: Option<T>,
}

impl<E, T> TableField<E, T>
where
  T: SqlValue,
{
  /// Creates a new instance from the table field name
  #[inline]
  pub const fn new(name: &'static str) -> Self {
    Self { name, phantom: PhantomData, value: None }
  }

  /// Table field name
  #[inline]
  pub const fn name(&self) -> &'static str {
    self.name
  }

  /// Table field value
  #[inline]
  pub const fn value(&self) -> &Option<T> {
    &self.value
  }

  /// Mutable version of [value]
  #[inline]
  pub fn value_mut(&mut self) -> &mut Option<T> {
    &mut self.value
  }
}
