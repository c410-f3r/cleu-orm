use crate::{
  Associations, FullAssociation, Limit, OrderBy, SourceAssociation, SqlWriter, MAX_NODES_NUM,
};
use core::{array, marker::PhantomData};

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
  type FullAssociations<'full_associations>
  where
    E: 'full_associations,
  = array::IntoIter<FullAssociation<'full_associations>, 0>;

  #[inline]
  fn full_associations(&self) -> Self::FullAssociations<'_> {
    [].into_iter()
  }
}

impl<E, S> SqlWriter<S> for (NoAssociation<E>,)
where
  E: From<crate::Error>,
  S: cl_traits::String,
{
  type Error = E;

  #[inline]
  fn write_insert<'value, V>(
    &self,
    _: &mut [Option<&'static str>; MAX_NODES_NUM],
    _: &mut S,
    _: &mut Option<SourceAssociation<'value, V>>,
  ) -> Result<(), Self::Error> {
    Ok(())
  }

  #[inline]
  fn write_select(
    &self,
    _: &mut S,
    _: OrderBy,
    _: Limit,
    _: &mut impl FnMut(&mut S) -> Result<(), Self::Error>,
  ) -> Result<(), Self::Error> {
    Ok(())
  }

  #[inline]
  fn write_select_associations(&self, _: &mut S) -> Result<(), Self::Error> {
    Ok(())
  }

  #[inline]
  fn write_select_fields(&self, _: &mut S) -> Result<(), Self::Error> {
    Ok(())
  }

  #[inline]
  fn write_select_orders_by(&self, _: &mut S) -> Result<(), Self::Error> {
    Ok(())
  }
}
