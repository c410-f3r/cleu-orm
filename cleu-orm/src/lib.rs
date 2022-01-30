//! # Cleu ORM

#![feature(generic_associated_types)]

mod association;
mod associations;
pub mod crud;
mod error;
mod field;
mod fields;
mod from_rows_suffix;
mod full_association;
mod fx_hasher;
mod no_association;
mod source_association;
mod sql_value;
mod sql_writer;
mod table_params;
#[cfg(test)]
mod tests;
mod to_field_values;
mod tuple_impls;
mod utils;

pub use association::*;
pub use associations::*;
#[cfg(feature = "derive")]
pub use cleu_orm_derive::*;
pub use error::*;
pub use field::*;
pub use fields::*;
#[cfg(any(feature = "with-sqlx-postgres", feature = "with-sqlx-runtime-tokio-native-tls"))]
pub use from_rows_suffix::*;
pub use full_association::*;
pub(crate) use fx_hasher::*;
pub use no_association::*;
pub use source_association::*;
pub use sql_value::*;
pub use sql_writer::*;
pub use table_params::*;
pub use to_field_values::*;
pub use utils::*;

/// Alias of [core::result::Result<T, cleu_orm::Error>].
pub type Result<T> = core::result::Result<T, Error>;
/// Used by some operations to identify different tables
pub type Suffix = u8;
/// Used by initial calls of [SqlWriter::write_insert]
pub type InitialInsertValue = i32;

/// The maximum number of nested associations some inner operations are allowed to have
#[allow(
    // `Into` is not constant
    clippy::as_conversions
)]
pub const MAX_NODES_NUM: usize = Suffix::MAX as _;
