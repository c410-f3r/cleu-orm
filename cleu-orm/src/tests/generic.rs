// D
// |--> B |
// |--> C |
//        |--> A

use crate::{
  Association, Field, InitialInsertValue, Limit, NoAssociation, OrderBy, SqlWriter, Suffix,
  TableParams, UpdateFieldValues, MAX_NODES_NUM,
};

const A: A = A { id: 1, name: "foo1" };
const B: B = B { a: A, id: 2, name: "foo2" };
const C: C = C { a: A, id: 3, name: "foo3" };
const D: D = D { b: B, c: C, id: 4, name: "foo4" };

#[derive(Debug)]
struct A {
  id: i32,
  name: &'static str,
}

struct AParams<'table>(
  (NoAssociation<()>,),
  (Field<(), &'table i32>, Field<(), &'static str>),
  Suffix,
);

impl<'table> AParams<'table> {
  fn new(suffix: Suffix) -> Self {
    Self((NoAssociation::new(),), (Field::new("id"), Field::new("name")), suffix)
  }
}

impl<'table> TableParams for AParams<'table> {
  type Associations = (NoAssociation<()>,);
  type Error = ();
  type Fields = (Field<(), &'table i32>, Field<(), &'static str>);
  type IdValue = &'table i32;
  type Table = A;

  fn associations(&self) -> &Self::Associations {
    &self.0
  }

  fn associations_mut(&mut self) -> &mut Self::Associations {
    &mut self.0
  }

  fn fields(&self) -> &Self::Fields {
    &self.1
  }

  fn fields_mut(&mut self) -> &mut Self::Fields {
    &mut self.1
  }

  fn id_field(&self) -> &Field<(), &'table i32> {
    &self.1.0
  }

  fn suffix(&self) -> Suffix {
    self.2
  }

  fn table_name() -> &'static str {
    "a"
  }
}

impl<'table> UpdateFieldValues<&'table A> for AParams<'table> {
  fn update_field_values(&mut self, from: &'table A) {
    *self.fields_mut().0.value_mut() = Some(&from.id);
    *self.fields_mut().1.value_mut() = Some(&from.name);
  }
}

struct B {
  a: A,
  id: i32,
  name: &'static str,
}

struct BParams<'table>(
  ((AParams<'table>, Association),),
  (Field<(), &'table i32>, Field<(), &'static str>),
  Suffix,
);

impl<'table> BParams<'table> {
  fn new(suffix: Suffix) -> Self {
    Self(
      ((AParams::new(suffix + 1), Association::new("id", "id_b")),),
      (Field::new("id"), Field::new("name")),
      suffix,
    )
  }
}

impl<'table> TableParams for BParams<'table> {
  type Associations = ((AParams<'table>, Association),);
  type Error = ();
  type Fields = (Field<(), &'table i32>, Field<(), &'static str>);
  type IdValue = &'table i32;
  type Table = B;

  fn associations(&self) -> &Self::Associations {
    &self.0
  }

  fn associations_mut(&mut self) -> &mut Self::Associations {
    &mut self.0
  }

  fn fields(&self) -> &Self::Fields {
    &self.1
  }

  fn fields_mut(&mut self) -> &mut Self::Fields {
    &mut self.1
  }

  fn id_field(&self) -> &Field<(), &'table i32> {
    &self.1.0
  }

  fn suffix(&self) -> Suffix {
    self.2
  }

  fn table_name() -> &'static str {
    "b"
  }
}

impl<'table> UpdateFieldValues<&'table B> for BParams<'table> {
  fn update_field_values(&mut self, from: &'table B) {
    *self.fields_mut().0.value_mut() = Some(&from.id);
    *self.fields_mut().1.value_mut() = Some(&from.name);
    self.associations_mut().0.0.update_field_values(&from.a);
  }
}

struct C {
  a: A,
  id: i32,
  name: &'static str,
}

struct CParams<'table>(
  ((AParams<'table>, Association),),
  (Field<(), &'table i32>, Field<(), &'static str>),
  Suffix,
);

impl<'table> CParams<'table> {
  fn new(suffix: Suffix) -> Self {
    Self(
      ((AParams::new(suffix + 1), Association::new("id", "id_c")),),
      (Field::new("id"), Field::new("name")),
      suffix,
    )
  }
}

impl<'table> TableParams for CParams<'table> {
  type Associations = ((AParams<'table>, Association),);
  type Error = ();
  type Fields = (Field<(), &'table i32>, Field<(), &'static str>);
  type IdValue = &'table i32;
  type Table = C;

  fn associations(&self) -> &Self::Associations {
    &self.0
  }

  fn associations_mut(&mut self) -> &mut Self::Associations {
    &mut self.0
  }

  fn fields(&self) -> &Self::Fields {
    &self.1
  }

  fn fields_mut(&mut self) -> &mut Self::Fields {
    &mut self.1
  }

  fn id_field(&self) -> &Field<(), &'table i32> {
    &self.1.0
  }

  fn suffix(&self) -> Suffix {
    self.2
  }

  fn table_name() -> &'static str {
    "c"
  }
}

impl<'table> UpdateFieldValues<&'table C> for CParams<'table> {
  fn update_field_values(&mut self, from: &'table C) {
    *self.fields_mut().0.value_mut() = Some(&from.id);
    *self.fields_mut().1.value_mut() = Some(&from.name);
    self.associations_mut().0.0.update_field_values(&from.a);
  }
}

struct D {
  b: B,
  c: C,
  id: i32,
  name: &'static str,
}

struct DParams<'table>(
  ((BParams<'table>, Association), (CParams<'table>, Association)),
  (Field<(), &'table i32>, Field<(), &'static str>),
  Suffix,
);

impl<'table> DParams<'table> {
  fn new(suffix: Suffix) -> Self {
    Self(
      (
        (BParams::new(suffix + 1), Association::new("id", "id_d")),
        (CParams::new(suffix + 2), Association::new("id", "id_d")),
      ),
      (Field::new("id"), Field::new("name")),
      suffix,
    )
  }
}

impl<'table> TableParams for DParams<'table> {
  type Associations = ((BParams<'table>, Association), (CParams<'table>, Association));
  type Error = ();
  type Fields = (Field<(), &'table i32>, Field<(), &'static str>);
  type IdValue = &'table i32;
  type Table = D;

  fn associations(&self) -> &Self::Associations {
    &self.0
  }

  fn associations_mut(&mut self) -> &mut Self::Associations {
    &mut self.0
  }

  fn fields(&self) -> &Self::Fields {
    &self.1
  }

  fn fields_mut(&mut self) -> &mut Self::Fields {
    &mut self.1
  }

  fn id_field(&self) -> &Field<(), &'table i32> {
    &self.1.0
  }

  fn suffix(&self) -> Suffix {
    self.2
  }

  fn table_name() -> &'static str {
    "d"
  }
}

impl<'table> UpdateFieldValues<&'table D> for DParams<'table> {
  fn update_field_values(&mut self, from: &'table D) {
    *self.fields_mut().0.value_mut() = Some(&from.id);
    *self.fields_mut().1.value_mut() = Some(&from.name);
    self.associations_mut().0.0.update_field_values(&from.b);
    self.associations_mut().1.0.update_field_values(&from.c);
  }
}

#[test]
fn multi_referred_table_has_correct_statements() {
  let mut buffer = String::new();
  let mut d_params = DParams::new(0);
  d_params.write_select(&mut buffer, OrderBy::Ascending, Limit::All, &mut |_| Ok(())).unwrap();
  assert_eq!(
    &buffer,
    r#"SELECT "d0".id AS d0__id,"d0".name AS d0__name,"b1".id AS b1__id,"b1".name AS b1__name,"a2".id AS a2__id,"a2".name AS a2__name,"c2".id AS c2__id,"c2".name AS c2__name,"a3".id AS a3__id,"a3".name AS a3__name FROM "d" AS "d0" LEFT JOIN "b" AS "b1" ON "d0".id = "b1".id_d LEFT JOIN "c" AS "c2" ON "d0".id = "c2".id_d LEFT JOIN "a" AS "a2" ON "b1".id = "a2".id_b LEFT JOIN "a" AS "a3" ON "c2".id = "a3".id_c  ORDER BY "d0".id,"b1".id,"a2".id,"c2".id,"a3".id ASC LIMIT ALL"#
  );
  buffer.clear();
  d_params.update_field_values(&D);
  d_params
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
}

#[test]
fn referred_table_has_correct_statements() {
  let mut buffer = String::new();
  let mut b_params = BParams::new(0);
  b_params.write_select(&mut buffer, OrderBy::Ascending, Limit::All, &mut |_| Ok(())).unwrap();
  assert_eq!(
    &buffer,
    r#"SELECT "b0".id AS b0__id,"b0".name AS b0__name,"a1".id AS a1__id,"a1".name AS a1__name FROM "b" AS "b0" LEFT JOIN "a" AS "a1" ON "b0".id = "a1".id_b  ORDER BY "b0".id,"a1".id ASC LIMIT ALL"#
  );
  buffer.clear();
  b_params.update_field_values(&B);
  b_params
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
}

#[test]
fn standalone_table_has_correct_statements() {
  let mut buffer = String::new();
  let mut a_params = AParams::new(0);
  a_params.write_select(&mut buffer, OrderBy::Ascending, Limit::All, &mut |_| Ok(())).unwrap();
  assert_eq!(
    &buffer,
    r#"SELECT "a0".id AS a0__id,"a0".name AS a0__name FROM "a" AS "a0"  ORDER BY "a0".id ASC LIMIT ALL"#
  );
  buffer.clear();
  a_params.update_field_values(&A);
  a_params
    .write_insert::<InitialInsertValue>(
      &mut [Default::default(); MAX_NODES_NUM],
      &mut buffer,
      &mut None,
    )
    .unwrap();
  assert_eq!(&buffer, r#"INSERT INTO "a" (id,name) VALUES ('1','foo1');"#);
}
