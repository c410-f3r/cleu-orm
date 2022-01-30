use crate::FullAssociation;

/// Groups tuples that form all associations of a table
pub trait Associations {
  /// See [Associations::full_associations]
  type FullAssociations<'full_associations>: Iterator<Item = FullAssociation<'full_associations>>
  where
    Self: 'full_associations;

  /// Yields all table associations
  fn full_associations(&self) -> Self::FullAssociations<'_>;
}
