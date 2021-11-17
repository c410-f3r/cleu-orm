use crate::{
  Association, Associations, Buffer, Field, Fields, FullAssociation, SqlWriter, TableParams,
};
use core::array;

macro_rules! tuple_impls {
  ($(
    $tuple_len:tt {
      $(($idx:tt) -> $T:ident)+
    }
  )+) => {
    $(
      impl<ERR, $($T: TableParams<Error = ERR>),+> Associations for ($( ($T, Association), )+)
      where
        ERR: From<crate::Error>
      {
        type FullAssociations<'x> where $($T: 'x,)+ = array::IntoIter<FullAssociation<'x>, $tuple_len>;

        #[inline]
        fn full_associations<'a>(&'a self) -> Self::FullAssociations<'a> {
          [
            $(
              FullAssociation::new(
                &self.$idx.1,
                $T::table_name(),
                $T::table_name_alias(),
                self.$idx.0.suffix()
              ),
            )+
          ].into_iter()
        }
      }

      impl<BUFFER, ERR, $($T,)+> SqlWriter<BUFFER> for ($( ($T, Association), )+)
      where
        BUFFER: Buffer,
        ERR: From<crate::Error>,
        $(
          $T: TableParams<Error = ERR>,
          $T::Associations: SqlWriter<BUFFER, Error = ERR>,
        )+
      {
        type Error = ERR;

        #[inline]
        fn write_select(
          &self,
          buffer: &mut BUFFER,
          where_cb: impl FnMut(&mut BUFFER) -> Result<(), Self::Error> + Clone,
        ) -> Result<(), Self::Error> {
          $(
            self.$idx.0.write_select(buffer, where_cb.clone())?;
          )+
          Ok(())
        }

        #[inline]
        fn write_select_associations(
          &self,
            buffer: &mut BUFFER,
        ) -> Result<(), Self::Error> {
          $(
            self.$idx.0.write_select_associations(buffer)?;
          )+
          Ok(())
        }

        #[inline]
        fn write_select_fields(
          &self,
            buffer: &mut BUFFER,
        ) -> Result<(), Self::Error> {
          $(
            self.$idx.0.write_select_fields(buffer)?;
          )+
          Ok(())
        }

        #[inline]
        fn write_select_orders_by(&self, buffer: &mut BUFFER) -> Result<(), Self::Error> {
          $(
            self.$idx.0.write_select_orders_by(buffer)?;
          )+
          Ok(())
        }
      }

      impl<$($T),+> Fields for ($( Field<$T>, )+) {
        type FieldNames = array::IntoIter<&'static str, $tuple_len>;

        #[inline]
        fn field_names(&self) -> Self::FieldNames {
          [ $( self.$idx.name(), )+ ].into_iter()
        }
      }
    )+
  }
}

tuple_impls! {
  1 {
    (0) -> A
  }
  2 {
    (0) -> A
    (1) -> B
  }
  3 {
    (0) -> A
    (1) -> B
    (2) -> C
  }
  4 {
    (0) -> A
    (1) -> B
    (2) -> C
    (3) -> D
  }
  5 {
    (0) -> A
    (1) -> B
    (2) -> C
    (3) -> D
    (4) -> E
  }
  6 {
    (0) -> A
    (1) -> B
    (2) -> C
    (3) -> D
    (4) -> E
    (5) -> F
  }
  7 {
    (0) -> A
    (1) -> B
    (2) -> C
    (3) -> D
    (4) -> E
    (5) -> F
    (6) -> G
  }
  8 {
    (0) -> A
    (1) -> B
    (2) -> C
    (3) -> D
    (4) -> E
    (5) -> F
    (6) -> G
    (7) -> H
  }
  9 {
    (0) -> A
    (1) -> B
    (2) -> C
    (3) -> D
    (4) -> E
    (5) -> F
    (6) -> G
    (7) -> H
    (8) -> I
  }
  10 {
    (0) -> A
    (1) -> B
    (2) -> C
    (3) -> D
    (4) -> E
    (5) -> F
    (6) -> G
    (7) -> H
    (8) -> I
    (9) -> J
  }
  11 {
    (0) -> A
    (1) -> B
    (2) -> C
    (3) -> D
    (4) -> E
    (5) -> F
    (6) -> G
    (7) -> H
    (8) -> I
    (9) -> J
    (10) -> K
  }
  12 {
    (0) -> A
    (1) -> B
    (2) -> C
    (3) -> D
    (4) -> E
    (5) -> F
    (6) -> G
    (7) -> H
    (8) -> I
    (9) -> J
    (10) -> K
    (11) -> L
  }
}
