use crate::{
  buffer_try_push_str, buffer_write_fmt, FullTableAssociation, SelectLimit, SelectOrderBy,
  SqlValue, SqlWriter, Table, TableAssociation, TableAssociations, TableDefs, TableField,
  TableFields, TableSourceAssociation, MAX_NODES_NUM,
};
use cl_traits::{CapacityUpperBound, SingleTypeStorage};
use core::{array, fmt::Display};

macro_rules! double_tuple_impls {
  ($(
    $tuple_len:tt {
      $(($idx:tt) -> $T:ident $U:ident)+
    }
  )+) => {
    $(
      impl<'entity, ERR, $($T, $U,)+> TableAssociations for ($( (Table<'entity, $U>, $T, TableAssociation), )+)
      where
        ERR: From<crate::Error>,
        $(
          $T: AsRef<[Table<'entity, $U>]>
            + CapacityUpperBound
            + SingleTypeStorage<Item = Table<'entity, $U>>,
          $U: TableDefs<'entity, Error = ERR>,
        )+
      {
        type FullTableAssociations = array::IntoIter<FullTableAssociation, $tuple_len>;

        #[inline]
        fn full_associations(&self) -> Self::FullTableAssociations {
          [
            $(
              FullTableAssociation::new(
                self.$idx.2,
                $U::TABLE_NAME,
                $U::TABLE_NAME_ALIAS,
                self.$idx.0.suffix()
              ),
            )+
          ].into_iter()
        }
      }

      impl<'entity, BUFFER, ERR, $($T, $U,)+> SqlWriter<BUFFER> for ($( (Table<'entity, $U>, $T, TableAssociation), )+)
      where
        BUFFER: cl_traits::String,
        ERR: From<crate::Error>,
        $(
          $T: AsRef<[Table<'entity, $U>]>
            + CapacityUpperBound
            + SingleTypeStorage<Item = Table<'entity, $U>>,
          $U: TableDefs<'entity, Error = ERR>,
          $U::Associations: SqlWriter<BUFFER, Error = ERR>,
        )+
      {
        type Error = ERR;

        #[inline]
        fn write_insert<'value, VALUE>(
          &self,
          aux: &mut [Option<&'static str>; MAX_NODES_NUM],
          buffer: &mut BUFFER,
          table_source_association: &mut Option<TableSourceAssociation<'value, VALUE>>
        ) -> Result<(), Self::Error>
        where
          VALUE: Display
        {
          $(
            if let Some(ref mut elem) = table_source_association.as_mut() {
              *elem.source_field_mut() = self.$idx.2.to_id();
            }
            if self.$idx.1.capacity_upper_bound() == 0 {
              self.$idx.0.write_insert(aux, buffer, table_source_association)?;
            }
            else {
              for elem in self.$idx.1.as_ref() {
                elem.write_insert(aux, buffer, table_source_association)?;
              }
            }
          )+
          Ok(())
        }

        #[inline]
        fn write_select(
          &self,
          buffer: &mut BUFFER,
          order_by: SelectOrderBy,
          limit: SelectLimit,
          where_cb: &mut impl FnMut(&mut BUFFER) -> Result<(), Self::Error>,
        ) -> Result<(), Self::Error> {
          $(
            self.$idx.0.write_select(buffer, order_by, limit, where_cb)?;
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

        #[inline]
        fn write_update(
          &self,
          aux: &mut [Option<&'static str>; MAX_NODES_NUM],
          buffer: &mut BUFFER,
        ) -> Result<(), Self::Error> {
          $(
            if self.$idx.1.capacity_upper_bound() == 0 {
              self.$idx.0.write_update(aux, buffer)?;
            }
            else {
              for elem in self.$idx.1.as_ref() {
                elem.write_update(aux, buffer)?;
              }
            }
          )+
          Ok(())
        }
      }
    )+
  }
}

macro_rules! tuple_impls {
  ($(
    $tuple_len:tt {
      $(($idx:tt) -> $T:ident)+
    }
  )+) => {
    $(
      impl<ERR, $($T: SqlValue),+> TableFields for ($( TableField<ERR, $T>, )+)
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
        fn write_insert_values<BUFFER>(&self, buffer: &mut BUFFER) -> Result<(), Self::Error>
        where
          BUFFER: cl_traits::String
        {
          $(
            if let &Some(ref elem) = self.$idx.value() {
              elem.write(buffer)?;
              buffer_try_push_str(buffer, ",")?;
            }
          )+
          Ok(())
        }

        #[inline]
        fn write_update_values<BUFFER>(&self, buffer: &mut BUFFER) -> Result<(), Self::Error>
        where
          BUFFER: cl_traits::String
        {
          $(
            if let &Some(ref elem) = self.$idx.value() {
              buffer_write_fmt(buffer, format_args!("{}=", self.$idx.name()))?;
              elem.write(buffer)?;
              buffer_try_push_str(buffer, ",")?;
            }
          )+
          Ok(())
        }
      }
    )+
  }
}

double_tuple_impls! {
  1 {
    (0) -> A B
  }
  2 {
    (0) -> A B
    (1) -> C D
  }
  3 {
    (0) -> A B
    (1) -> C D
    (2) -> E F
  }
  4 {
    (0) -> A B
    (1) -> C D
    (2) -> E F
    (3) -> G H
  }
  5 {
    (0) -> A B
    (1) -> C D
    (2) -> E F
    (3) -> G H
    (4) -> I J
  }
  6 {
    (0) -> A B
    (1) -> C D
    (2) -> E F
    (3) -> G H
    (4) -> I J
    (5) -> K L
  }
  7 {
    (0) -> A B
    (1) -> C D
    (2) -> E F
    (3) -> G H
    (4) -> I J
    (5) -> K L
    (6) -> M N
  }
  8 {
    (0) -> A B
    (1) -> C D
    (2) -> E F
    (3) -> G H
    (4) -> I J
    (5) -> K L
    (6) -> M N
    (7) -> O P
  }
  9 {
    (0) -> A B
    (1) -> C D
    (2) -> E F
    (3) -> G H
    (4) -> I J
    (5) -> K L
    (6) -> M N
    (7) -> O P
    (8) -> Q R
  }
  10 {
    (0) -> A B
    (1) -> C D
    (2) -> E F
    (3) -> G H
    (4) -> I J
    (5) -> K L
    (6) -> M N
    (7) -> O P
    (8) -> Q R
    (9) -> S T
  }
  11 {
    (0) -> A B
    (1) -> C D
    (2) -> E F
    (3) -> G H
    (4) -> I J
    (5) -> K L
    (6) -> M N
    (7) -> O P
    (8) -> Q R
    (9) -> S T
    (10) -> U V
  }
  12 {
    (0) -> A B
    (1) -> C D
    (2) -> E F
    (3) -> G H
    (4) -> I J
    (5) -> K L
    (6) -> M N
    (7) -> O P
    (8) -> Q R
    (9) -> S T
    (10) -> U V
    (11) -> W X
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
