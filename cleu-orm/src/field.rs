use std::marker::PhantomData;

/// Table field name and its associated Rust type
#[derive(Debug)]
pub struct Field<T> {
  name: &'static str,
  ty: PhantomData<T>,
}

impl<T> Field<T> {
  /// Creates a new instance from the table field name
  #[inline]
  pub const fn new(name: &'static str) -> Self {
    Self { name, ty: PhantomData }
  }

  /// Table field name
  #[inline]
  pub const fn name(&self) -> &'static str {
    self.name
  }
}
