/// Sql select `ORDER BY` clause
#[derive(Clone, Copy, Debug)]
pub enum OrderBy {
  /// ORDER BY ... ASC
  Ascending,
  /// ORDER BY ... DESC
  Descending,
}
