// C --> A/B

use crate::{
  FromSuffixRslt, InitialInsertValue, NoTableAssociation, SelectLimit, SelectOrderBy, SqlWriter,
  Suffix, Table, TableAssociation, TableDefs, TableField, MAX_NODES_NUM,
};
use core::mem;

#[derive(Debug)]
struct A {
  id: i32,
  name: &'static str,
}

#[derive(Debug)]
struct ATableDefs;

impl<'entity> TableDefs<'entity> for ATableDefs {
  const PRIMARY_KEY_NAME: &'static str = "id";
  const TABLE_NAME: &'static str = "a";

  type Associations = NoTableAssociation<()>;
  type Entity = A;
  type Error = ();
  type Fields = (TableField<(), &'static str>,);
  type PrimaryKeyValue = &'entity i32;

  fn type_instances(_: Suffix) -> FromSuffixRslt<'entity, Self> {
    (NoTableAssociation::new(), (TableField::new("name"),))
  }

  fn update_table_fields(entity: &'entity Self::Entity, table: &mut Table<'entity, Self>) {
    *table.id_field_mut().value_mut() = Some(&entity.id);

    *table.fields_mut().0.value_mut() = Some(&entity.name);
  }
}

struct B {
  id: i32,
  name: &'static str,
}

#[derive(Debug)]
struct BTableDefs;

impl<'entity> TableDefs<'entity> for BTableDefs {
  const PRIMARY_KEY_NAME: &'static str = "id";
  const TABLE_NAME: &'static str = "b";

  type Associations = NoTableAssociation<()>;
  type Entity = B;
  type Error = ();
  type Fields = (TableField<(), &'static str>,);
  type PrimaryKeyValue = &'entity i32;

  fn type_instances(_: Suffix) -> FromSuffixRslt<'entity, Self> {
    (NoTableAssociation::new(), (TableField::new("name"),))
  }

  fn update_table_fields(entity: &'entity Self::Entity, table: &mut Table<'entity, Self>) {
    *table.id_field_mut().value_mut() = Some(&entity.id);

    *table.fields_mut().0.value_mut() = Some(&entity.name);
  }
}

struct C {
  r#as: Vec<A>,
  bs: Vec<B>,
  id: i32,
  name: &'static str,
}

#[derive(Debug)]
struct CTableDefs;

impl<'entity> TableDefs<'entity> for CTableDefs {
  const PRIMARY_KEY_NAME: &'static str = "id";
  const TABLE_NAME: &'static str = "c";

  type Associations = (
    (Table<'entity, ATableDefs>, Vec<Table<'entity, ATableDefs>>, TableAssociation),
    (Table<'entity, BTableDefs>, Vec<Table<'entity, BTableDefs>>, TableAssociation),
  );
  type Entity = C;
  type Error = ();
  type Fields = (TableField<(), &'static str>,);
  type PrimaryKeyValue = &'entity i32;

  fn type_instances(suffix: Suffix) -> FromSuffixRslt<'entity, Self> {
    (
      (
        (Table::new(suffix + 1), vec![], TableAssociation::new("id", "id_a")),
        (Table::new(suffix + 2), vec![], TableAssociation::new("id", "id_b")),
      ),
      (TableField::new("name"),),
    )
  }

  fn update_table_fields(entity: &'entity Self::Entity, table: &mut Table<'entity, Self>) {
    *table.id_field_mut().value_mut() = Some(&entity.id);

    *table.fields_mut().0.value_mut() = Some(&entity.name);

    table.associations_mut().0.1.clear();
    for a in entity.r#as.iter() {
      let mut elem = Table::new(0);
      elem.update_table_fields(a);
      table.associations_mut().0.1.push(elem);
    }

    table.associations_mut().1.1.clear();
    for b in entity.bs.iter() {
      let mut elem = Table::new(0);
      elem.update_table_fields(b);
      table.associations_mut().1.1.push(elem);
    }
  }
}

#[cfg(all(target_arch = "x86_64", target_pointer_width = "64"))]
#[test]
fn assert_sizes() {
  assert_eq!(mem::size_of::<Table<'_, ATableDefs>>(), 64);
  assert_eq!(mem::size_of::<Table<'_, BTableDefs>>(), 64);
  assert_eq!(mem::size_of::<Table<'_, CTableDefs>>(), 304);
}

#[test]
fn write_collection_has_correct_params() {
  let a1 = A { id: 1, name: "foo1" };
  let a2 = A { id: 2, name: "foo2" };
  let c3 = C { r#as: vec![a1, a2], bs: vec![], id: 3, name: "foo3" };

  let mut buffer = String::new();
  let mut c_table_defs = Table::<CTableDefs>::default();

  c_table_defs
    .write_select(&mut buffer, SelectOrderBy::Ascending, SelectLimit::All, &mut |_| Ok(()))
    .unwrap();
  assert_eq!(
    &buffer,
    r#"SELECT "c0".id AS c0__id,"c0".name AS c0__name,"a1".id AS a1__id,"a1".name AS a1__name,"b2".id AS b2__id,"b2".name AS b2__name FROM "c" AS "c0" LEFT JOIN "a" AS "a1" ON "c0".id = "a1".id_a LEFT JOIN "b" AS "b2" ON "c0".id = "b2".id_b  ORDER BY "c0".id,"a1".id,"b2".id ASC LIMIT ALL"#
  );

  buffer.clear();
  c_table_defs
    .write_insert::<InitialInsertValue>(
      &mut [Default::default(); MAX_NODES_NUM],
      &mut buffer,
      &mut None,
    )
    .unwrap();
  assert_eq!(&buffer, r#""#);

  c_table_defs.update_table_fields(&c3);

  buffer.clear();
  c_table_defs
    .write_select(&mut buffer, SelectOrderBy::Ascending, SelectLimit::All, &mut |_| Ok(()))
    .unwrap();
  assert_eq!(
    &buffer,
    r#"SELECT "c0".id AS c0__id,"c0".name AS c0__name,"a1".id AS a1__id,"a1".name AS a1__name,"b2".id AS b2__id,"b2".name AS b2__name FROM "c" AS "c0" LEFT JOIN "a" AS "a1" ON "c0".id = "a1".id_a LEFT JOIN "b" AS "b2" ON "c0".id = "b2".id_b  ORDER BY "c0".id,"a1".id,"b2".id ASC LIMIT ALL"#
  );

  buffer.clear();
  c_table_defs
    .write_insert::<InitialInsertValue>(
      &mut [Default::default(); MAX_NODES_NUM],
      &mut buffer,
      &mut None,
    )
    .unwrap();
  assert_eq!(
    &buffer,
    r#"INSERT INTO "c" (id,name) VALUES ('3','foo3');INSERT INTO "a" (id,name,id_a) VALUES ('1','foo1','3');INSERT INTO "a" (id,name,id_a) VALUES ('2','foo2','3');"#
  );
}
