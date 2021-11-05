use cleu_orm::TableParams;

#[derive(Debug, cleu_orm_derive::TableParams)]
#[cleu_orm(table(alias = "FOO", name = "BAR"))]
pub struct Foo {
  _id: i32,
  _name: String,
}


fn main() {
  assert_eq!(FooParams::table_name(), "BAR");
  assert_eq!(FooParams::table_name_alias(), Some("FOO"));
}