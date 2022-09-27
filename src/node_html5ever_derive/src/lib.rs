use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse::Parser, parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn add_node_fields(_args: TokenStream, input: TokenStream) -> TokenStream {
  let mut ast = parse_macro_input!(input as DeriveInput);
  match &mut ast.data {
    syn::Data::Struct(ref mut struct_data) => {
      match &mut struct_data.fields {
        syn::Fields::Named(fields) => {
          fields.named.push(
            syn::Field::parse_named
              .parse2(quote! {
                pub(crate) parent:
                  Option<napi::Either<
                    napi::bindgen_prelude::WeakReference<crate::element::Element>,
                    napi::bindgen_prelude::WeakReference<crate::document::Document>
                  >>
              })
              .unwrap(),
          );
          fields.named.push(
            syn::Field::parse_named
              .parse2(quote! {
                pub(crate) env: napi::Env
              })
              .unwrap(),
          );

          fields.named.push(
            syn::Field::parse_named
              .parse2(quote! {
                pub(crate) id: usize
              })
              .unwrap(),
          );
        }
        _ => (),
      }

      return quote! {
          #ast
      }
      .into();
    }
    _ => panic!("`add_field` has to be used with structs "),
  }
}

#[proc_macro_derive(NodeType, attributes(default))]
pub fn node_macro_derive(input: TokenStream) -> TokenStream {
  let ast: &syn::DeriveInput = &syn::parse(input).unwrap();
  let name = &ast.ident;

  let struct_properties = match &ast.data {
    syn::Data::Struct(syn::DataStruct {
      fields: syn::Fields::Named(fields),
      ..
    }) => &fields.named,
    _ => panic!("this derive macro only works on structs with named fields"),
  }
  .into_iter()
  .map(|field| {
    let default_attribute = field
      .attrs
      .iter()
      .find(|attr| attr.path.is_ident("default"));

    (field, default_attribute)
  });

  let arguments = struct_properties
    .clone()
    .filter_map(|(field, default_attribute)| {
      let ident = &field.ident;
      let ty = &field.ty;

      default_attribute.map_or_else(|| Some(quote!(#ident: #ty,)), |_default| None)
    });

  let argument_fields = struct_properties
    .clone()
    .filter_map(|(field, default_attribute)| {
      let ident = &field.ident;

      default_attribute.map_or_else(|| Some(quote!(#ident,)), |_default| None)
    });

  let default_fields = struct_properties
    .into_iter()
    .filter_map(|(field, default_attribute)| {
      default_attribute.map(|attr| {
        let ident = &field.ident;
        let default_value = &attr.tokens;
        quote!(#ident: #default_value,)
      })
    });

  let gen = quote!(
    #[napi]
    #[automatically_derived]
    impl #name {
      pub(crate) fn new_reference(env: napi::Env, #(#arguments)*) ->
          napi::Result<napi::bindgen_prelude::Reference<Self>> {

        let inner = Self {
          #(#default_fields)*
          #(#argument_fields)*
          parent: None,
          env,
          id: crate::id::get_id(),
        };
        Self::into_reference(inner, env)
      }

      #[napi(getter)]
      pub fn get_parent_element(&self) ->
          Option<napi::bindgen_prelude::WeakReference<crate::element::Element>> {

        let parent_node = self.parent.as_ref();

        match parent_node {
          Some(element_or_document) => {
            match (element_or_document) {
              napi::Either::A(element) => Some(element.clone()),
              napi::Either::B(_) => None,
            }
          },
          None => None
        }
      }

      #[napi(getter)]
      pub fn get_parent_node(&self) ->
          Option<napi::Either<
            napi::bindgen_prelude::WeakReference<crate::element::Element>,
            napi::bindgen_prelude::WeakReference<crate::document::Document>
          >> {

            let maybe_reference = self.parent.as_ref();

        maybe_reference.map(|value| match value {
          napi::Either::A(element) => napi::Either::A(element.clone()),
          napi::Either::B(document) => napi::Either::B(document.clone()),
        })
      }
    }
  );
  gen.into()
}
