use itertools::multiunzip;
use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse::Parser, parse_macro_input, DeriveInput};

#[derive(Default)]
struct Features {
  has_children: bool,
  is_child: bool,
}

#[proc_macro_attribute]
pub fn create_node(args: TokenStream, input: TokenStream) -> TokenStream {
  let mut features: Features = Default::default();
  for f in syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated
    .parse(args)
    .unwrap()
    .into_iter()
  {
    match f.get_ident().unwrap().to_string().as_str() {
      "has_children" => features.has_children = true,
      "is_child" => features.is_child = true,
      _ => panic!("Unknown feature"),
    }
  }

  let ast: DeriveInput = parse_macro_input!(input as DeriveInput).into();

  let named_fields = &match ast.data {
    syn::Data::Struct(syn::DataStruct {
      fields: syn::Fields::Named(fields),
      ..
    }) => fields.named,
    _ => panic!("`this derive macro only works on structs with named fields"),
  };

  let (fields, arguments, argument_fields): (Vec<_>, Vec<_>, Vec<_>) =
    multiunzip(named_fields.into_iter().map(|field| {
      let field_name = &field.ident;
      let field_type = &field.ty;

      (
        quote!(#field,),
        quote!(#field_name: #field_type,),
        quote!(#field_name,),
      )
    }));

  let name = &ast.ident;

  let has_children_impl = match features.has_children {
    true => quote!(
      #[napi(getter)]
      pub fn get_child_nodes(&self) -> Vec<crate::ChildNode> {
        macro_backend::children::children(self.into())
      }

      #[napi(getter)]
      pub fn get_children(&self) -> Vec<napi::bindgen_prelude::Reference<crate::Element>> {
        macro_backend::children::children(self.into())
      }

      #[napi]
      pub fn append_child(&self, child: crate::ChildNode) -> napi::Result<()> {
        macro_backend::children::append_child(self.into(), child.into())
      }

      #[napi]
      pub fn remove_child(&self, child: crate::ChildNode) {
        macro_backend::children::remove_child(self.into(), child.into());
      }

      #[napi]
      pub fn get_element_by_id(
        &self,
        id: String,
      ) -> Option<napi::bindgen_prelude::Reference<crate::Element>> {
        macro_backend::children::get_element_by_id(self.into(), id)
      }

      #[napi]
      pub fn get_elements_by_class_name(
        &self,
        class_name: String,
      ) -> Vec<napi::bindgen_prelude::Reference<crate::Element>> {
        macro_backend::children::get_elements_by_class_name(self.into(), class_name)
      }

      #[napi(getter)]
      pub fn get_first_child(&self) -> Option<crate::ChildNode> {
        macro_backend::children::first_child(self.into())
      }

      #[napi(getter)]
      pub fn get_first_element_child(
        &self,
      ) -> Option<napi::bindgen_prelude::Reference<crate::Element>> {
        macro_backend::children::first_child(self.into())
      }

      #[napi(getter)]
      pub fn get_last_child(&self) -> Option<crate::ChildNode> {
        macro_backend::children::last_child(self.into())
      }

      #[napi(getter)]
      pub fn get_last_element_child(
        &self,
      ) -> Option<napi::bindgen_prelude::Reference<crate::Element>> {
        macro_backend::children::last_child(self.into())
      }
    ),
    false => quote!(),
  };

  let is_child_impl = match features.is_child {
    true => quote! {
      #[napi(getter)]
      pub fn get_parent_element(&self) ->
          napi::Result<Option<napi::bindgen_prelude::WeakReference<crate::Element>>> {
        macro_backend::parent::parent(self.into())
      }

      #[napi(getter)]
      pub fn get_parent_node(&self) ->
          napi::Result<Option<crate::ParentNode>> {
        macro_backend::parent::parent(self.into())
      }

      #[napi(getter)]
      pub fn get_owner_document(
        &self,
      ) -> napi::Result<Option<napi::bindgen_prelude::WeakReference<crate::Document>>> {
        macro_backend::parent::parent(self.into())
      }

      #[napi]
      pub fn remove(&self) -> napi::Result<()> {
        macro_backend::parent::remove(self.into())
      }

      #[napi(getter)]
      pub fn get_previous_sibling(&self) ->
        napi::Result<Option<crate::ChildNode>> {
        macro_backend::parent::previous(self.into())
      }

      #[napi(getter)]
      pub fn get_previous_element_sibling(&self) ->
        napi::Result<Option<napi::bindgen_prelude::Reference<crate::Element>>> {
        macro_backend::parent::previous(self.into())
      }

      #[napi(getter)]
      pub fn get_next_sibling(&self) ->
        napi::Result<Option<crate::ChildNode>> {
        macro_backend::parent::next(self.into())
      }

      #[napi(getter)]
      pub fn get_next_element_sibling(&self) ->
        napi::Result<Option<napi::bindgen_prelude::Reference<crate::Element>>> {
        macro_backend::parent::next(self.into())
      }
    },
    false => quote! {},
  };

  return quote! {
    use crate::macro_backend;

    #[napi]
    pub struct #name {
      pub(crate) env: napi::Env,
      pub(crate) node_handler: crate::NodeHandler,
      pub(crate) weak_reference: Option<napi::bindgen_prelude::WeakReference<Self>>,
      pub(crate) id: usize,

      #(#fields)*
    }

    impl std::fmt::Debug for #name {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#name {{ id: {} }}", self.id)
      }
    }

    #[napi]
    #[automatically_derived]
    impl #name {
      pub(crate) fn new_reference(env: napi::Env, #(#arguments)*) ->
          napi::Result<napi::bindgen_prelude::Reference<Self>> {

        let inner = Self {
          #(#argument_fields)*
          env,
          id: crate::get_id(),
          node_handler: crate::NodeHandler::new(env),
          weak_reference: None,
        };

        let mut r = Self::into_reference(inner, env)?;
        r.clone(env)?.weak_reference = Some(r.clone(env)?.downgrade());
        Ok(r)
      }

      pub(crate) fn get_node_handler(&self) -> crate::NodeHandler {
        self.node_handler.clone()
      }

      #[napi(getter)]
      pub fn get_node_name(&self) -> String {
        crate::macro_backend::get_node_name(self.into())
      }

      #is_child_impl
      #has_children_impl
    }
  }
  .into();
}
