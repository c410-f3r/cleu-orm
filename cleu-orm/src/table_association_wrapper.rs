use crate::{Table, TableAssociation, TableDefs};
use cl_traits::SingleTypeStorage;

/// A helper structure for people that manually implement [TableAssociations]
#[allow(
  // `Table` derives `Debug` but for some reason such thing is not allowed here
  missing_debug_implementations
)]
pub struct TableAssociationWrapper<'entity, TD, TS>
where
  TD: TableDefs<'entity>,
  TD::Error: From<crate::Error>,
  TS: AsRef<[Table<'entity, TD>]> + SingleTypeStorage<Item = Table<'entity, TD>>,
{
  /// See [TableAssociation]
  pub association: TableAssociation,
  /// Used to construct SELECT operations
  pub guide: Table<'entity, TD>,
  /// A storage of zero, one or many tables used for INSERT and UPDATE operations
  pub tables: TS,
}
