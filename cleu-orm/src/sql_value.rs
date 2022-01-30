use crate::{buffer_try_push_str, buffer_write_fmt};

/// Raw SQL representation of a type
pub trait SqlValue {
  /// See [SqlValue]
  fn write<S>(&self, buffer: &mut S) -> crate::Result<()>
  where
    S: cl_traits::String;
}

impl<T> SqlValue for &'_ T
where
  T: SqlValue,
{
  #[inline]
  fn write<S>(&self, buffer: &mut S) -> crate::Result<()>
  where
    S: cl_traits::String,
  {
    (**self).write(buffer)
  }
}

impl<T> SqlValue for Option<T>
where
  T: SqlValue,
{
  #[inline]
  fn write<S>(&self, buffer: &mut S) -> crate::Result<()>
  where
    S: cl_traits::String,
  {
    if let Some(ref elem) = *self {
      elem.write(buffer)
    } else {
      buffer_try_push_str(buffer, "null")
    }
  }
}

macro_rules! impl_display {
  ($ty:ty $(, $($bounds:tt)+)?) => {
    impl<$($($bounds)+)?> SqlValue for $ty {
      #[inline]
      fn write<S>(&self, buffer: &mut S) -> crate::Result<()>
      where
        S: cl_traits::String,
      {
        buffer_write_fmt(buffer, format_args!("'{}'", self))
      }
    }
  }
}

impl_display!(&'_ str);
impl_display!(String);
impl_display!(bool);
impl_display!(i32);
impl_display!(i64);
impl_display!(u32);
impl_display!(u64);

#[cfg(feature = "with-arrayvec")]
impl_display!(arrayvec::ArrayString<N>, const N: usize);
#[cfg(feature = "with-rust_decimal")]
impl_display!(rust_decimal::Decimal);
