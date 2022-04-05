use core::marker::PhantomData;

/// An empty entity for testing purposes
#[derive(Debug)]
pub struct NoTableEntity<E>(PhantomData<E>);

impl<E> NoTableEntity<E> {
  /// Creates a new instance regardless of `E`
  #[inline]
  pub const fn new() -> Self {
    Self(PhantomData)
  }
}

#[cfg(any(feature = "sqlx-postgres", feature = "sqlx-runtime-tokio-native-tls"))]
impl<B, E> crate::FromRowsSuffix<B> for NoTableEntity<E>
where
  B: cl_traits::String,
  E: From<crate::Error>,
{
  type Error = E;

  #[inline]
  fn from_rows_suffix(
    _: &[sqlx_core::postgres::PgRow],
    _: &mut B,
    _: crate::Suffix,
    _: &sqlx_core::postgres::PgRow,
  ) -> Result<(usize, Self), Self::Error> {
    Ok((1, Self::new()))
  }
}
