use arrayvec::ArrayString;
use core::{array, marker::PhantomData};

use crate::{Associations, FullAssociation, SqlWriter};

/// For entities that don't have associations
#[derive(Debug)]
pub struct NoAssociation<E>(PhantomData<E>);

impl<E> NoAssociation<E> {
  /// Creates a new instance regardless of `E`
  #[inline]
  pub const fn new() -> Self {
    Self(PhantomData)
  }
}

impl<E> Associations for (NoAssociation<E>,) {
  type FullAssociations<'x> = array::IntoIter<FullAssociation<'x>, 0>;

  #[inline]
  fn full_associations(&self) -> Self::FullAssociations<'_> {
    [].into_iter()
  }
}

impl<E, const N: usize> SqlWriter<N> for (NoAssociation<E>,)
where
  E: From<crate::Error>,
{
  type Error = E;

  #[inline]
  fn write_select(&self, _: &mut ArrayString<N>, _: &str) -> Result<(), Self::Error> {
    Ok(())
  }

  #[inline]
  fn write_select_associations(&self, _: &mut ArrayString<N>) -> Result<(), Self::Error> {
    Ok(())
  }

  #[inline]
  fn write_select_fields(&self, _: &mut ArrayString<N>) -> Result<(), Self::Error> {
    Ok(())
  }

  #[inline]
  fn write_select_orders_by(&self, _: &mut ArrayString<N>) -> Result<(), Self::Error> {
    Ok(())
  }
}
