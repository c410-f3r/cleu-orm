// A
// |--> B --|
// |--> C --|
//          |--> D

use crate::{Association, Field, NoAssociation, SqlWriter, TableParams};
use arrayvec::ArrayString;

#[derive(Debug)]
struct A {
  _id: i32,
  _name: String,
}

struct AParams((NoAssociation<()>,), (Field<i32>, Field<String>), u8);

impl AParams {
  fn new(suffix: u8) -> Self {
    Self((NoAssociation::new(),), (Field::new("id"), Field::new("name")), suffix)
  }
}

impl TableParams for AParams {
  type Associations = (NoAssociation<()>,);
  type Error = ();
  type Fields = (Field<i32>, Field<String>);
  type Table = A;

  fn associations(&self) -> &Self::Associations {
    &self.0
  }

  fn fields(&self) -> &Self::Fields {
    &self.1
  }

  fn id_field(&self) -> &str {
    self.1.0.name()
  }

  fn suffix(&self) -> u8 {
    self.2
  }

  fn table_name() -> &'static str {
    "a"
  }
}

struct B {
  _a: A,
  _id: i32,
  _name: String,
}

struct BParams(((AParams, Association),), (Field<i32>, Field<String>), u8);

impl BParams {
  fn new(suffix: u8) -> Self {
    Self(
      ((AParams::new(suffix + 1), Association::new("id_a", "id")),),
      (Field::new("id"), Field::new("name")),
      suffix,
    )
  }
}

impl TableParams for BParams {
  type Associations = ((AParams, Association),);
  type Error = ();
  type Fields = (Field<i32>, Field<String>);
  type Table = B;

  fn associations(&self) -> &Self::Associations {
    &self.0
  }

  fn fields(&self) -> &Self::Fields {
    &self.1
  }

  fn id_field(&self) -> &str {
    self.1.0.name()
  }

  fn suffix(&self) -> u8 {
    self.2
  }

  fn table_name() -> &'static str {
    "b"
  }
}

struct C {
  _a: A,
  _id: i32,
  _name: String,
}

struct CParams(((AParams, Association),), (Field<i32>, Field<String>), u8);

impl CParams {
  fn new(suffix: u8) -> Self {
    Self(
      ((AParams::new(suffix + 1), Association::new("id_a", "id")),),
      (Field::new("id"), Field::new("name")),
      suffix,
    )
  }
}

impl TableParams for CParams {
  type Associations = ((AParams, Association),);
  type Error = ();
  type Fields = (Field<i32>, Field<String>);
  type Table = C;

  fn associations(&self) -> &Self::Associations {
    &self.0
  }

  fn fields(&self) -> &Self::Fields {
    &self.1
  }

  fn id_field(&self) -> &str {
    self.1.0.name()
  }

  fn suffix(&self) -> u8 {
    self.2
  }

  fn table_name() -> &'static str {
    "c"
  }
}

struct D {
  _b: B,
  _c: C,
  _id: i32,
  _name: String,
}

struct DParams(((BParams, Association), (CParams, Association)), (Field<i32>, Field<String>), u8);

impl DParams {
  fn new(suffix: u8) -> Self {
    Self(
      (
        (BParams::new(suffix + 1), Association::new("id_b", "id")),
        (CParams::new(suffix + 2), Association::new("id_c", "id")),
      ),
      (Field::new("id"), Field::new("name")),
      suffix,
    )
  }
}

impl TableParams for DParams {
  type Associations = ((BParams, Association), (CParams, Association));
  type Error = ();
  type Fields = (Field<i32>, Field<String>);
  type Table = D;

  fn associations(&self) -> &Self::Associations {
    &self.0
  }

  fn fields(&self) -> &Self::Fields {
    &self.1
  }

  fn id_field(&self) -> &str {
    self.1.0.name()
  }

  fn suffix(&self) -> u8 {
    self.2
  }

  fn table_name() -> &'static str {
    "d"
  }
}

#[test]
fn multi_referred_table_has_correct_select_query() {
  let mut buffer = ArrayString::<512>::new();
  let d = DParams::new(0);
  d.write_select(&mut buffer, "").unwrap();
  assert_eq!(
    &buffer,
    r#"SELECT "d0".id AS d0__id,"d0".name AS d0__name,"b1".id AS b1__id,"b1".name AS b1__name,"a2".id AS a2__id,"a2".name AS a2__name,"c2".id AS c2__id,"c2".name AS c2__name,"a3".id AS a3__id,"a3".name AS a3__name FROM "d" AS "d0" LEFT JOIN "b" AS "b1" ON "d0".id_b = "b1".id LEFT JOIN "c" AS "c2" ON "d0".id_c = "c2".id LEFT JOIN "a" AS "a2" ON "b1".id_a = "a2".id LEFT JOIN "a" AS "a3" ON "c2".id_a = "a3".id  ORDER BY "d0".id,"b1".id,"a2".id,"c2".id,"a3".id"#
  )
}

#[test]
fn referred_table_has_correct_select_query() {
  let mut buffer = ArrayString::<256>::new();
  let b = BParams::new(0);
  b.write_select(&mut buffer, "").unwrap();
  assert_eq!(
    &buffer,
    r#"SELECT "b0".id AS b0__id,"b0".name AS b0__name,"a1".id AS a1__id,"a1".name AS a1__name FROM "b" AS "b0" LEFT JOIN "a" AS "a1" ON "b0".id_a = "a1".id  ORDER BY "b0".id,"a1".id"#
  );
}

#[test]
fn standalone_table_has_correct_select_query() {
  let mut buffer = ArrayString::<256>::new();
  let a = AParams::new(0);
  a.write_select(&mut buffer, "").unwrap();
  assert_eq!(
    &buffer,
    r#"SELECT "a0".id AS a0__id,"a0".name AS a0__name FROM "a" AS "a0"  ORDER BY "a0".id"#
  )
}
