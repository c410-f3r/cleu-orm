use cleu_orm::TableParams;

#[derive(Debug, cleu_orm_derive::TableParams)]
pub struct Foo {
  _id: i32,
  _name: String,
}

#[derive(Debug, cleu_orm_derive::TableParams)]pub struct Bar {
  _id: i32,
  #[cleu_orm(association(from_id = "id_foo", to_id = "id"))]
  _foo: Foo,
  _name: String,
}

fn main() {
  let foo_params = FooParams::new(0);
  let bar_params = BarParams::new(0);
  let _ = foo_params.associations();
  let _ = bar_params.associations();
}
