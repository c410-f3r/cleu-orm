//! # Cleu ORM - Derive

#![allow(clippy::shadow_reuse)]

mod utils;

use quote::{format_ident, quote};
use syn::{parse_macro_input, spanned::Spanned, Data, DataStruct, DeriveInput, Fields};
use utils::*;

/// Implements [cleu_orm::TableParams].
#[proc_macro_derive(TableParams, attributes(cleu_orm))]
pub fn table_params(ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input = parse_macro_input!(ts as DeriveInput);
  do_table_params(input).unwrap_or_else(|err| err.to_compile_error()).into()
}

fn do_table_params(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
  let input_span = input.span();

  let table_struct_name = input.ident.to_string();
  let (error_ty, table_name_alias, table_name) =
    container_attrs(&input.attrs, input_span, &table_struct_name)?;

  let fields = if let Data::Struct(DataStruct { fields: Fields::Named(elem), .. }) = input.data {
    elem.named
  } else {
    return Err(syn::Error::new(input_span, "Table element must be a structure with named fields"));
  };

  let associations_ty_name = format_ident!("{}ParamsAssociationsTy", table_struct_name);
  let fields_ty_name = format_ident!("{}ParamsFieldsTy", table_struct_name);
  let table_params_struct_name = format_ident!("{}Params", table_struct_name);
  let table_struct_ty = input.ident;

  let does_not_have_associations = fields.iter().all(|elem| elem.attrs.is_empty());

  let fields_with_attrs = fields.iter().filter(|elem| !elem.attrs.is_empty());
  let fields_without_attrs = fields.iter().filter(|elem| elem.attrs.is_empty());

  let (association_exprs, association_types) = if does_not_have_associations {
    (
      vec![quote! { cleu_orm::NoAssociation::new() }],
      vec![quote! { cleu_orm::NoAssociation<#error_ty> }],
    )
  } else {
    let association_exprs = fields_with_attrs.clone().filter_map(|elem| {
      let snake_case = elem.ident.as_ref().map(|i| i.to_string())?;
      let a = if let Some("_") = snake_case.get(..1) {
        snake_case.get(1..).unwrap_or_default()
      } else {
        &snake_case
      };
      let associated_table_params_struct_name = format_ident!("{}Params", to_camel_case(a));

      let params = group_params(elem.attrs.get(0)?).ok()?;

      let mut iter = params.1.into_iter();

      let (from_id_ident, from_id_expr) = iter.next()?;
      if from_id_ident != "from_id" {
        return None;
      }

      let (to_id_ident, to_id_expr) = iter.next()?;
      if to_id_ident != "to_id" {
        return None;
      }

      Some(quote! {
        (#associated_table_params_struct_name::new({
          incrementing_suffix = incrementing_suffix.wrapping_add(1);
          incrementing_suffix
        }),
        cleu_orm::Association::new(#from_id_expr, #to_id_expr))
      })
    });

    let association_types = fields_with_attrs.clone().filter_map(|elem| {
      let snake_case = elem.ident.as_ref().map(|i| i.to_string())?;
      let a = if let Some("_") = snake_case.get(..1) {
        snake_case.get(1..).unwrap_or_default()
      } else {
        &snake_case
      };
      let associated_table_params_struct_name = format_ident!("{}Params", to_camel_case(a));

      Some(quote! { (#associated_table_params_struct_name, cleu_orm::Association) })
    });

    (association_exprs.collect(), association_types.collect())
  };

  let field_exprs =
    fields_without_attrs.clone().filter_map(|elem| elem.ident.as_ref().map(|i| i.to_string()));
  let field_types = fields_without_attrs.map(|elem| &elem.ty);

  Ok(quote! {
    #[automatically_derived]
    type #associations_ty_name = (
      #( #association_types, )*
    );
    #[automatically_derived]
    type #fields_ty_name = (
      #( cleu_orm::Field<#field_types>, )*
    );

    #[automatically_derived]
    pub struct #table_params_struct_name(
      #associations_ty_name,
      #fields_ty_name,
      u8
    );

    #[automatically_derived]
    impl #table_params_struct_name {
      #[inline]
      pub const fn new(suffix: u8) -> Self {
        let mut incrementing_suffix = suffix;
        Self(
          (
            #( #association_exprs, )*
          ),
          (
            #( cleu_orm::Field::new(#field_exprs), )*
          ),
          suffix
        )
      }
    }

    #[automatically_derived]
    impl cleu_orm::TableParams for #table_params_struct_name {
      type Associations = #associations_ty_name;
      type Error = #error_ty;
      type Fields = #fields_ty_name;
      type Table = #table_struct_ty;

      #[inline]
      fn associations(&self) -> &Self::Associations {
        &self.0
      }

      #[inline]
      fn fields(&self) -> &Self::Fields {
        &self.1
      }

      #[inline]
      fn id_field(&self) -> &str {
        self.1.0.name()
      }

      #[inline]
      fn suffix(&self) -> u8 {
        self.2
      }

      #[inline]
      fn table_name() -> &'static str {
        #table_name
      }

      #[inline]
      fn table_name_alias() -> Option<&'static str> {
        #table_name_alias
      }
    }
  })
}
