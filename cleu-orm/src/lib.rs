//! # Cleu ORM

pub mod crud;
mod error;
#[cfg(any(feature = "sqlx-postgres", feature = "sqlx-runtime-tokio-native-tls"))]
mod from_rows_suffix;
mod full_table_association;
mod fx_hasher;
mod no_table_association;
mod no_table_entity;
mod no_table_field;
mod select_limit;
mod select_order_by;
mod sql_value;
mod sql_writer;
mod table;
mod table_association;
mod table_association_wrapper;
mod table_associations;
mod table_defs;
mod table_field;
mod table_fields;
mod table_source_association;
#[cfg(test)]
mod tests;
mod tuple_impls;
mod utils;

pub use cl_traits::String;
#[cfg(feature = "derive")]
pub use cleu_orm_derive::*;
pub use error::*;
#[cfg(any(feature = "sqlx-postgres", feature = "sqlx-runtime-tokio-native-tls"))]
pub use from_rows_suffix::*;
pub use full_table_association::*;
pub(crate) use fx_hasher::*;
pub use no_table_association::*;
pub use no_table_entity::*;
pub use no_table_field::*;
pub use select_limit::*;
pub use select_order_by::*;
pub use sql_value::*;
pub use sql_writer::*;
pub use table::*;
pub use table_association::*;
pub use table_association_wrapper::*;
pub use table_associations::*;
pub use table_defs::*;
pub use table_field::*;
pub use table_fields::*;
pub use table_source_association::*;
pub use utils::*;

/// Shortcut to avoid having to manually type the result of [TableDefs::new]
pub type FromSuffixRslt<'entity, TD> =
  (<TD as TableDefs<'entity>>::Associations, <TD as TableDefs<'entity>>::Fields);
/// Used by initial calls of [SqlWriter::write_insert]
pub type InitialInsertValue = i32;
/// Alias of [core::result::Result<T, cleu_orm::Error>].
pub type Result<T> = core::result::Result<T, Error>;
/// Used by some operations to identify different tables
pub type Suffix = u32;

/// The maximum number of nested associations some inner operations are allowed to have
pub const MAX_NODES_NUM: usize = 1024 * 16;
