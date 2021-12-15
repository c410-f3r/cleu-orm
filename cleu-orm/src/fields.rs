use crate::Buffer;

/// Groups tuples that form all fields of a table
pub trait Fields {
  /// See [crate::Error]
  type Error: From<crate::Error>;
  /// See [Fields::field_names]
  type FieldNames: Iterator<Item = &'static str>;

  /// Yields all table field names
  fn field_names(&self) -> Self::FieldNames;

  /// Writes the table instance values
  fn write_table_values<B>(&self, buffer: &mut B) -> Result<(), Self::Error>
  where
    B: Buffer;
}
