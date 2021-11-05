use proc_macro2::{Ident, Span};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{spanned::Spanned, Attribute, Lit, Meta, NestedMeta, Type};

pub(crate) struct QuoteOption<T>(pub(crate) Option<T>);

impl<T> ToTokens for QuoteOption<T>
where
  T: ToTokens,
{
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    tokens.append_all(match self.0 {
      Some(ref t) => quote! { core::option::Option::Some(#t) },
      None => quote! { core::option::Option::None },
    });
  }
}

pub(crate) fn container_attrs(
  attrs: &[Attribute],
  span: Span,
  table_struct_name: &str,
) -> syn::Result<(Type, QuoteOption<String>, String)> {
  let mut error_ty_opt = None;
  let mut table_name_alias = None;
  let mut table_name_opt = None;

  for attr in attrs {
    let (group_ident, group_pairs) =
      if let Ok(elem) = group_params(attr) { elem } else { continue };

    if group_ident == "error" {
      let err = |err_span| {
        Err(syn::Error::new(
          err_span,
          "A standalone `error` attribute with a `ty` key must be provided",
        ))
      };

      let (group_pair_ident, group_pair_lit) = if let &[(ref a, ref b)] = group_pairs.as_slice() {
        (a, b)
      } else {
        return err(span);
      };

      if group_pair_ident == "ty" {
        error_ty_opt = Some(syn::parse_str(group_pair_lit.as_str())?);
      } else {
        return err(span);
      }
    }

    if group_ident == "table" {
      for (group_pair_ident, group_pair_lit) in group_pairs {
        if group_pair_ident == "alias" {
          table_name_alias = Some(group_pair_lit.clone());
        }

        if group_pair_ident == "name" {
          table_name_opt = Some(group_pair_lit.clone());
        }
      }
    }
  }

  let error_ty = if let Some(elem) = error_ty_opt { elem } else { syn::parse_str("()")? };
  let table_name =
    if let Some(elem) = table_name_opt { elem } else { to_snake_case(table_struct_name) };

  Ok((error_ty, QuoteOption(table_name_alias), table_name))
}

pub(crate) fn group_params(attr: &Attribute) -> syn::Result<(Ident, Vec<(Ident, String)>)> {
  let bad_attribute_content = |span| {
    Err(syn::Error::new(
      span,
      "`cley_orm` attribute must have a single group element separated by key and value pairs",
    ))
  };
  let bad_attribute_start =
    |span| Err(syn::Error::new(span, "All attributes must start with `cleu_orm`"));

  let meta = attr.parse_meta()?;

  let mut vec = vec![];

  let attr_meta_list = if let Meta::List(elem) = meta {
    elem
  } else {
    return bad_attribute_content(meta.span());
  };

  let attribute_ident = if let Some(elem) = attr_meta_list.path.segments.iter().next() {
    &elem.ident
  } else {
    return bad_attribute_start(attr.path.segments.span());
  };

  if attribute_ident != "cleu_orm" {
    return bad_attribute_start(attribute_ident.span());
  }

  let group_meta_list =
    if let Some(&NestedMeta::Meta(Meta::List(ref elem))) = attr_meta_list.nested.iter().next() {
      elem
    } else {
      return bad_attribute_start(attr_meta_list.nested.span());
    };

  let group_ident = if let Some(elem) = group_meta_list.path.segments.iter().next() {
    &elem.ident
  } else {
    return bad_attribute_start(attr.path.segments.span());
  };

  for nested_meta in group_meta_list.nested.iter() {
    let mnv = if let &NestedMeta::Meta(Meta::NameValue(ref elem)) = nested_meta {
      elem
    } else {
      return bad_attribute_content(nested_meta.span());
    };

    let segment = if let Some(elem) = mnv.path.segments.iter().next() {
      elem.ident.clone()
    } else {
      return bad_attribute_start(attr_meta_list.path.segments.span());
    };

    let s = if let Lit::Str(ref elem) = mnv.lit {
      elem.value()
    } else {
      return bad_attribute_content(mnv.lit.span());
    };

    vec.push((segment, s))
  }

  Ok((group_ident.clone(), vec))
}

pub(crate) fn to_camel_case(s: &str) -> String {
  let mut chars = s.chars();
  let mut rslt = String::new();

  let mut previous = if let Some(elem) = chars.next() {
    elem
  } else {
    return rslt;
  };

  if previous.is_lowercase() {
    rslt.push(previous.to_ascii_uppercase());
  } else {
    rslt.push(previous);
  }

  for current in chars {
    if previous == '_' && current.is_lowercase() {
      rslt.push(current.to_ascii_uppercase());
    } else {
      rslt.push(current);
    }
    previous = current;
  }

  rslt
}

pub(crate) fn to_snake_case(s: &str) -> String {
  let mut chars = s.chars();
  let mut rslt = String::new();

  let mut previous = if let Some(elem) = chars.next() {
    elem
  } else {
    return rslt;
  };

  if previous.is_lowercase() {
    rslt.push(previous);
  } else {
    rslt.push(previous.to_ascii_lowercase());
  }

  for current in chars {
    if previous.is_lowercase() && current.is_uppercase() {
      rslt.push('_');
      rslt.push(current.to_ascii_lowercase());
    } else {
      rslt.push(current);
    }
    previous = current;
  }

  rslt
}
