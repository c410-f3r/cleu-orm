#[derive(Debug)]
pub enum Error {
  Bar,
}

impl From<cleu_orm::Error> for Error {
  fn from(_: cleu_orm::Error) -> Self {
    Self::Bar
  }
}

#[derive(Debug, cleu_orm_derive::TableParams)]
#[cleu_orm(error(ty = "Error"))]
pub struct Foo {
  _id: i32,
  _name: String,
}

fn main() {
}