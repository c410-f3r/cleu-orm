use cleu_orm::TableDefs;

#[derive(Debug, cleu_orm_derive::TableDefs)]
pub struct Foo {
  _id: i32,
  _name: String,
}

#[derive(Debug, cleu_orm_derive::TableDefs)]pub struct Bar {
  _id: i32,
  #[cleu_orm(association(from_id = "id_foo", to_id = "id"))]
  _foo: Foo,
  _name: String,
}

fn main() {
  let foo_table = FooTable::new(0);
  let bar_table = BarTable::new(0);
  let _ = foo_table.associations();
  let _ = bar_table.associations();
}
