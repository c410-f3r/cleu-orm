//! # Cleu ORM

#![allow(clippy::shadow_same)]
#![feature(generic_associated_types)]

mod association;
mod associations;
mod buffer;
mod crud;
mod error;
mod field;
mod fields;
mod from_rows_suffix;
mod full_association;
mod no_association;
mod sql_writer;
mod table_params;
#[cfg(test)]
mod tests;
mod tuple_impls;
mod utils;

pub use association::*;
pub use associations::*;
pub use buffer::*;
#[cfg(feature = "derive")]
pub use cleu_orm_derive::*;
pub use crud::*;
pub use error::*;
pub use field::*;
pub use fields::*;
pub use from_rows_suffix::*;
pub use full_association::*;
pub use no_association::*;
pub use sql_writer::*;
pub use table_params::*;
pub use utils::*;

/// Alias of [core::result::Result<T, cleu_orm::Error>].
pub type Result<T> = core::result::Result<T, Error>;
