/// Groups tuples that form all fields of a table
pub trait Fields {
  /// See [Fields::field_names]
  type FieldNames: Iterator<Item = &'static str>;

  /// Yields all table field names
  fn field_names(&self) -> Self::FieldNames;
}
