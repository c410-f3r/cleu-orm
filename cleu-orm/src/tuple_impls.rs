use crate::{
  buffer_try_push_str, Association, Associations, Buffer, Field, Fields, FullAssociation,
  SourceAssociation, SqlValue, SqlWriter, TableParams, MAX_NODES_NUM,
};
use core::{array, fmt};

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
        fn write_insert<'value, V>(
          &self,
          aux: &mut [Option<&'static str>; MAX_NODES_NUM],
          buffer: &mut BUFFER,
          source_association: &mut Option<SourceAssociation<'value, V>>
        ) -> Result<(), Self::Error>
        where
          V: fmt::Display
        {
          $(
            if let Some(ref mut elem) = source_association.as_mut() {
              *elem.source_field_mut() = self.$idx.1.to_id();
            }
            self.$idx.0.write_insert(aux, buffer, source_association)?;
          )+
          Ok(())
        }

        #[inline]
        fn write_select(
          &self,
          buffer: &mut BUFFER,
          where_cb: &mut impl FnMut(&mut BUFFER) -> Result<(), Self::Error>,
        ) -> Result<(), Self::Error> {
          $( self.$idx.0.write_select(buffer, where_cb)?; )+
          Ok(())
        }

        #[inline]
        fn write_select_associations(
          &self,
            buffer: &mut BUFFER,
        ) -> Result<(), Self::Error> {
          $( self.$idx.0.write_select_associations(buffer)?; )+
          Ok(())
        }

        #[inline]
        fn write_select_fields(
          &self,
            buffer: &mut BUFFER,
        ) -> Result<(), Self::Error> {
          $( self.$idx.0.write_select_fields(buffer)?; )+
          Ok(())
        }

        #[inline]
        fn write_select_orders_by(&self, buffer: &mut BUFFER) -> Result<(), Self::Error> {
          $( self.$idx.0.write_select_orders_by(buffer)?; )+
          Ok(())
        }
      }

      impl<ERR, $($T: SqlValue),+> Fields for ($( Field<ERR, $T>, )+)
      where
        ERR: From<crate::Error>
      {
        type Error = ERR;
        type FieldNames = array::IntoIter<&'static str, $tuple_len>;

        #[inline]
        fn field_names(&self) -> Self::FieldNames {
          [ $( self.$idx.name(), )+ ].into_iter()
        }

        #[inline]
        fn write_table_values<BUFFER>(&self, buffer: &mut BUFFER) -> Result<(), Self::Error>
        where
          BUFFER: Buffer
        {
          $(
            if let Some(ref elem) = *self.$idx.value() {
              buffer_try_push_str(buffer, "'")?;
              elem.write(buffer)?;
              buffer_try_push_str(buffer, "',")?;
            }
          )+
          Ok(())
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
