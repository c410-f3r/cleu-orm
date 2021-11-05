use crate::FullAssociation;

/// Groups tuples that form all associations of a table
pub trait Associations {
  /// See [Associations::full_associations]
  type FullAssociations<'x>: Iterator<Item = FullAssociation<'x>>;

  /// Yields all table associations
  fn full_associations(&self) -> Self::FullAssociations<'_>;
}
