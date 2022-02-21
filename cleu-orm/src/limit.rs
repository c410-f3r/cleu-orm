/// Sql select `LIMIT` clause
#[derive(Clone, Copy, Debug)]
pub enum Limit {
  /// LIMIT ALL
  All,
  /// LIMIT `n`
  Count(u32),
}
