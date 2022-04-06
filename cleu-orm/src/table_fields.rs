/// Groups tuples that form all fields of a table
pub trait TableFields {
  /// See [crate::Error]
  type Error: From<crate::Error>;
  /// See [Fields::field_names]
  type FieldNames: Iterator<Item = &'static str>;

  /// Yields all table field names
  fn field_names(&self) -> Self::FieldNames;

  /// Writes the table instance values for INSERT statements
  fn write_insert_values<B>(&self, buffer: &mut B) -> Result<(), Self::Error>
  where
    B: cl_traits::String;

  /// Writes the table instance values for UPDATE statements
  fn write_update_values<B>(&self, buffer: &mut B) -> Result<(), Self::Error>
  where
    B: cl_traits::String;
}
