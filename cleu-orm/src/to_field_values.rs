use crate::TableParams;

/// Updates all optional table elements at once
pub trait UpdateFieldValues<T>
where
  Self: TableParams,
{
  /// See [UpdateFieldValues]
  fn update_field_values(&mut self, from: T);
}
