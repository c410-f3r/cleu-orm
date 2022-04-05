use crate::{
  FullTableAssociation, SelectLimit, SelectOrderBy, SqlWriter, TableAssociations,
  TableSourceAssociation, MAX_NODES_NUM,
};
use core::{array, marker::PhantomData};

/// For entities that don't have associations
#[derive(Debug)]
pub struct NoTableAssociation<E>(PhantomData<E>);

impl<E> NoTableAssociation<E> {
  /// Creates a new instance regardless of `E`
  #[inline]
  pub const fn new() -> Self {
    Self(PhantomData)
  }
}

impl<E> TableAssociations for NoTableAssociation<E> {
  type FullTableAssociations = array::IntoIter<FullTableAssociation, 0>;

  #[inline]
  fn full_associations(&self) -> Self::FullTableAssociations {
    [].into_iter()
  }
}

impl<B, E> SqlWriter<B> for NoTableAssociation<E>
where
  B: cl_traits::String,
  E: From<crate::Error>,
{
  type Error = E;

  #[inline]
  fn write_insert<'value, V>(
    &self,
    _: &mut [Option<&'static str>; MAX_NODES_NUM],
    _: &mut B,
    _: &mut Option<TableSourceAssociation<'value, V>>,
  ) -> Result<(), Self::Error> {
    Ok(())
  }

  #[inline]
  fn write_select(
    &self,
    _: &mut B,
    _: SelectOrderBy,
    _: SelectLimit,
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
