// D
// |--> B |
// |--> C |
//        |--> A

use crate::{
  FromSuffixRslt, InitialInsertValue, NoTableAssociation, SelectLimit, SelectOrderBy, SqlWriter,
  Suffix, Table, TableAssociation, TableDefs, TableField, MAX_NODES_NUM,
};
use core::mem;

const A: A = A { id: 1, name: "foo1" };
const B: B = B { a: A, id: 2, name: "foo2" };
const C: C = C { a: A, id: 3, name: "foo3" };
const D: D = D { b: B, c: C, id: 4, name: "foo4" };

#[derive(Debug)]
struct A {
  id: i32,
  name: &'static str,
}

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
  a: A,
  id: i32,
  name: &'static str,
}

struct BTableDefs;

impl<'entity> TableDefs<'entity> for BTableDefs {
  const PRIMARY_KEY_NAME: &'static str = "id";
  const TABLE_NAME: &'static str = "b";

  type Associations =
    ((Table<'entity, ATableDefs>, [Table<'entity, ATableDefs>; 0], TableAssociation),);
  type Entity = B;
  type Error = ();
  type Fields = (TableField<(), &'static str>,);
  type PrimaryKeyValue = &'entity i32;

  fn type_instances(suffix: Suffix) -> FromSuffixRslt<'entity, Self> {
    (
      ((Table::new(suffix + 1), [], TableAssociation::new("id", "id_b")),),
      (TableField::new("name"),),
    )
  }

  fn update_table_fields(entity: &'entity Self::Entity, table: &mut Table<'entity, Self>) {
    *table.id_field_mut().value_mut() = Some(&entity.id);

    *table.fields_mut().0.value_mut() = Some(&entity.name);

    table.associations_mut().0.0.update_table_fields(&entity.a);
  }
}

struct C {
  a: A,
  id: i32,
  name: &'static str,
}

struct CTableDefs;

impl<'entity> TableDefs<'entity> for CTableDefs {
  const PRIMARY_KEY_NAME: &'static str = "id";
  const TABLE_NAME: &'static str = "c";

  type Associations =
    ((Table<'entity, ATableDefs>, [Table<'entity, ATableDefs>; 0], TableAssociation),);
  type Entity = C;
  type Error = ();
  type Fields = (TableField<(), &'static str>,);
  type PrimaryKeyValue = &'entity i32;

  fn type_instances(suffix: Suffix) -> FromSuffixRslt<'entity, Self> {
    (
      ((Table::new(suffix + 1), [], TableAssociation::new("id", "id_c")),),
      (TableField::new("name"),),
    )
  }

  fn update_table_fields(entity: &'entity Self::Entity, table: &mut Table<'entity, Self>) {
    *table.id_field_mut().value_mut() = Some(&entity.id);

    *table.fields_mut().0.value_mut() = Some(&entity.name);

    table.associations_mut().0.0.update_table_fields(&entity.a);
  }
}

struct D {
  b: B,
  c: C,
  id: i32,
  name: &'static str,
}

struct DTableDefs;

impl<'entity> TableDefs<'entity> for DTableDefs {
  const PRIMARY_KEY_NAME: &'static str = "id";
  const TABLE_NAME: &'static str = "d";

  type Associations = (
    (Table<'entity, BTableDefs>, [Table<'entity, BTableDefs>; 0], TableAssociation),
    (Table<'entity, CTableDefs>, [Table<'entity, CTableDefs>; 0], TableAssociation),
  );
  type Entity = D;
  type Error = ();
  type Fields = (TableField<(), &'static str>,);
  type PrimaryKeyValue = &'entity i32;

  fn type_instances(suffix: Suffix) -> FromSuffixRslt<'entity, Self> {
    (
      (
        (Table::new(suffix + 1), [], TableAssociation::new("id", "id_d")),
        (Table::new(suffix + 2), [], TableAssociation::new("id", "id_d")),
      ),
      (TableField::new("name"),),
    )
  }

  fn update_table_fields(entity: &'entity Self::Entity, table: &mut Table<'entity, Self>) {
    *table.id_field_mut().value_mut() = Some(&entity.id);

    *table.fields_mut().0.value_mut() = Some(&entity.name);

    table.associations_mut().0.0.update_table_fields(&entity.b);
    table.associations_mut().1.0.update_table_fields(&entity.c);
  }
}

#[cfg(all(target_arch = "x86_64", target_pointer_width = "64"))]
#[test]
fn assert_sizes() {
  assert_eq!(mem::size_of::<Table<'_, ATableDefs>>(), 64);
  assert_eq!(mem::size_of::<Table<'_, BTableDefs>>(), 160);
  assert_eq!(mem::size_of::<Table<'_, CTableDefs>>(), 160);
  assert_eq!(mem::size_of::<Table<'_, DTableDefs>>(), 448);
}

#[test]
fn multi_referred_table_has_correct_statements() {
  let mut buffer = String::new();
  let mut d_table_defs = Table::<DTableDefs>::default();

  d_table_defs
    .write_select(&mut buffer, SelectOrderBy::Ascending, SelectLimit::All, &mut |_| Ok(()))
    .unwrap();
  assert_eq!(
    &buffer,
    r#"SELECT "d0".id AS d0__id,"d0".name AS d0__name,"b1".id AS b1__id,"b1".name AS b1__name,"a2".id AS a2__id,"a2".name AS a2__name,"c2".id AS c2__id,"c2".name AS c2__name,"a3".id AS a3__id,"a3".name AS a3__name FROM "d" AS "d0" LEFT JOIN "b" AS "b1" ON "d0".id = "b1".id_d LEFT JOIN "c" AS "c2" ON "d0".id = "c2".id_d LEFT JOIN "a" AS "a2" ON "b1".id = "a2".id_b LEFT JOIN "a" AS "a3" ON "c2".id = "a3".id_c  ORDER BY "d0".id,"b1".id,"a2".id,"c2".id,"a3".id ASC LIMIT ALL"#
  );

  d_table_defs.update_table_fields(&D);

  buffer.clear();
  d_table_defs
    .write_insert::<InitialInsertValue>(
      &mut [Default::default(); MAX_NODES_NUM],
      &mut buffer,
      &mut None,
    )
    .unwrap();
  assert_eq!(
    &buffer,
    r#"INSERT INTO "d" (id,name) VALUES ('4','foo4');INSERT INTO "b" (id,name,id_d) VALUES ('2','foo2','4');INSERT INTO "a" (id,name,id_b) VALUES ('1','foo1','2');INSERT INTO "c" (id,name,id_d) VALUES ('3','foo3','4');"#
  );

  buffer.clear();
  d_table_defs.write_update(&mut [Default::default(); MAX_NODES_NUM], &mut buffer).unwrap();
  assert_eq!(
    &buffer,
    r#"UPDATE d SET id='4',name='foo4' WHERE id='4';UPDATE b SET id='2',name='foo2' WHERE id='2';UPDATE a SET id='1',name='foo1' WHERE id='1';UPDATE c SET id='3',name='foo3' WHERE id='3';"#
  );
}

#[test]
fn referred_table_has_correct_statements() {
  let mut buffer = String::new();
  let mut b_table_defs = Table::<BTableDefs>::default();
  b_table_defs
    .write_select(&mut buffer, SelectOrderBy::Ascending, SelectLimit::All, &mut |_| Ok(()))
    .unwrap();
  assert_eq!(
    &buffer,
    r#"SELECT "b0".id AS b0__id,"b0".name AS b0__name,"a1".id AS a1__id,"a1".name AS a1__name FROM "b" AS "b0" LEFT JOIN "a" AS "a1" ON "b0".id = "a1".id_b  ORDER BY "b0".id,"a1".id ASC LIMIT ALL"#
  );

  b_table_defs.update_table_fields(&B);

  buffer.clear();
  b_table_defs
    .write_insert::<InitialInsertValue>(
      &mut [Default::default(); MAX_NODES_NUM],
      &mut buffer,
      &mut None,
    )
    .unwrap();
  assert_eq!(
    &buffer,
    r#"INSERT INTO "b" (id,name) VALUES ('2','foo2');INSERT INTO "a" (id,name,id_b) VALUES ('1','foo1','2');"#
  );

  buffer.clear();
  b_table_defs.write_update(&mut [Default::default(); MAX_NODES_NUM], &mut buffer).unwrap();
  assert_eq!(
    &buffer,
    r#"UPDATE b SET id='2',name='foo2' WHERE id='2';UPDATE a SET id='1',name='foo1' WHERE id='1';"#
  );
}

#[test]
fn standalone_table_has_correct_statements() {
  let mut buffer = String::new();
  let mut a_table_defs = Table::<ATableDefs>::default();
  a_table_defs
    .write_select(&mut buffer, SelectOrderBy::Ascending, SelectLimit::All, &mut |_| Ok(()))
    .unwrap();
  assert_eq!(
    &buffer,
    r#"SELECT "a0".id AS a0__id,"a0".name AS a0__name FROM "a" AS "a0"  ORDER BY "a0".id ASC LIMIT ALL"#
  );

  a_table_defs.update_table_fields(&A);

  buffer.clear();
  a_table_defs
    .write_insert::<InitialInsertValue>(
      &mut [Default::default(); MAX_NODES_NUM],
      &mut buffer,
      &mut None,
    )
    .unwrap();
  assert_eq!(&buffer, r#"INSERT INTO "a" (id,name) VALUES ('1','foo1');"#);

  buffer.clear();
  a_table_defs.write_update(&mut [Default::default(); MAX_NODES_NUM], &mut buffer).unwrap();
  assert_eq!(&buffer, r#"UPDATE a SET id='1',name='foo1' WHERE id='1';"#);
}
