/// Table association and its associated Rust type
#[derive(Debug)]
pub struct Association {
  from_id: &'static str,
  to_id: &'static str,
}

impl Association {
  /// Creates a new instance from all parameters
  #[inline]
  pub const fn new(from_id: &'static str, to_id: &'static str) -> Self {
    Self { from_id, to_id }
  }

  /// Caller id filed name
  #[inline]
  pub const fn from_id(&self) -> &'static str {
    self.from_id
  }

  /// Callee id filed name
  #[inline]
  pub const fn to_id(&self) -> &'static str {
    self.to_id
  }
}
