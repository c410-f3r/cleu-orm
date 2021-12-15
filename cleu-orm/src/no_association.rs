use crate::{Associations, Buffer, FullAssociation, SourceAssociation, SqlWriter, MAX_NODES_NUM};
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
  type FullAssociations<'x>
  where
    E: 'x,
  = array::IntoIter<FullAssociation<'x>, 0>;

  #[inline]
  fn full_associations(&self) -> Self::FullAssociations<'_> {
    [].into_iter()
  }
}

impl<B, E> SqlWriter<B> for (NoAssociation<E>,)
where
  B: Buffer,
  E: From<crate::Error>,
{
  type Error = E;

  #[inline]
  fn write_insert<'value, V>(
    &self,
    _: &mut [Option<&'static str>; MAX_NODES_NUM],
    _: &mut B,
    _: &mut Option<SourceAssociation<'value, V>>,
  ) -> Result<(), Self::Error> {
    Ok(())
  }

  #[inline]
  fn write_select(
    &self,
    _: &mut B,
    _: &mut impl FnMut(&mut B) -> Result<(), Self::Error>,
  ) -> Result<(), Self::Error> {
    Ok(())
  }

  #[inline]
  fn write_select_associations(&self, _: &mut B) -> Result<(), Self::Error> {
    Ok(())
  }

  #[inline]
  fn write_select_fields(&self, _: &mut B) -> Result<(), Self::Error> {
    Ok(())
  }

  #[inline]
  fn write_select_orders_by(&self, _: &mut B) -> Result<(), Self::Error> {
    Ok(())
  }
}
