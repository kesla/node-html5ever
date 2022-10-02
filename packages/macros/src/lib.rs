use itertools::multiunzip;
use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse::Parser, parse_macro_input, DeriveInput};

#[derive(Default)]
struct Features {
  children: bool,
  parent: bool,
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
      "children" => features.children = true,
      "parent" => features.parent = true,
      _ => panic!("Unknown feature"),
    }
  }

  // .map(|path| path.get_ident().unwrap().to_string())
  // .collect::<Vec<String>>();

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

  let children_field = match features.children {
    true => quote!(pub(crate) list: std::rc::Rc<std::cell::RefCell<Vec<crate::Handle>>>,),
    false => quote!(),
  };
  let children_init = match features.children {
    true => quote!(list: std::rc::Rc::new(std::cell::RefCell::new(vec![])),),
    false => quote!(),
  };
  let children_impl = match features.children {
    true => quote!(
      #[napi(getter)]
      pub fn get_children(
        &self,
      ) -> napi::Result<Vec<napi::bindgen_prelude::Reference<crate::Element>>> {
        macro_backend::children::get_children(self.list.clone())
      }

      #[napi]
      pub fn append_child(
        &self,
        child: napi::bindgen_prelude::Either4<
          &crate::Comment,
          &crate::DocType,
          &crate::Element,
          &crate::Text,
        >,
      ) {
        macro_backend::children::append_child(self.get_handle(), child);
      }

      #[napi]
      pub fn remove_element(&mut self, child: &crate::Element) {
        macro_backend::children::remove_element(self.get_handle(), child);
      }

      #[napi]
      pub fn get_element_by_id(
        &self,
        id: String,
      ) -> napi::Result<Option<napi::bindgen_prelude::Reference<crate::Element>>> {
        macro_backend::children::get_element_by_id(self.list.clone(), id)
      }

      #[napi]
      pub fn get_elements_by_class_name(
        &self,
        class_name: String,
      ) -> napi::Result<Vec<napi::bindgen_prelude::Reference<crate::Element>>> {
        macro_backend::children::get_elements_by_class_name(self.list.clone(), class_name)
      }
    ),
    false => quote!(),
  };

  let parent_field = match features.parent {
    true => quote!(
      pub(crate) parent:
        std::cell::RefCell<Option<napi::Either<
          napi::bindgen_prelude::WeakReference<crate::Element>,
          napi::bindgen_prelude::WeakReference<crate::Document>
        >>>,
    ),
    false => quote!(),
  };
  let parent_init = match features.parent {
    true => quote!(parent: std::cell::RefCell::new(None),),
    false => quote!(),
  };
  let parent_impl = match features.parent {
    true => quote! {
      #[napi(getter)]
      pub fn get_parent_element(&self) ->
          Option<napi::bindgen_prelude::WeakReference<crate::Element>> {
        macro_backend::parent::get_parent_element(&self.parent)
      }

      #[napi(getter)]
      pub fn get_parent_node(&self) ->
          Option<napi::Either<
            napi::bindgen_prelude::WeakReference<crate::Element>,
            napi::bindgen_prelude::WeakReference<crate::Document>
          >> {
        macro_backend::parent::get_parent_node(&self.parent)
      }

      #[napi]
      pub fn remove(&mut self) -> napi::Result<()> {
        macro_backend::parent::remove(self.env.clone(), &self.parent, &self.get_handle())
      }

      #[napi(getter)]
      pub fn owner_document(
        &self,
      ) -> napi::Result<Option<napi::bindgen_prelude::WeakReference<crate::Document>>> {
        macro_backend::parent::owner_document(self.env.clone(), &self.parent)
      }
    },
    false => quote! {},
  };

  return quote! {
  use crate::macro_backend;

        #[napi]
        pub struct #name {
          #parent_field
          #children_field
          pub(crate) env: napi::Env,
          pub(crate) lazy_weak_handle: crate::LazyWeakHandle,
          pub(crate) weak_reference: Option<napi::bindgen_prelude::WeakReference<Self>>,
          pub(crate) id: usize,

          #(#fields)*
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
              lazy_weak_handle: crate::LazyWeakHandle::default(),
              #parent_init
              #children_init
              weak_reference: None,
            };

            let mut r = Self::into_reference(inner, env)?;
            r.weak_reference = Some(r.clone(env)?.downgrade());
            Ok(r)
          }

          pub(crate) fn get_handle(&self) -> crate::Handle {
            let reference = macro_backend::upgrade_weak_reference(
              self.env,
              &self.weak_reference
            ).unwrap();

            self.lazy_weak_handle.get_or_init(reference)
          }

          #parent_impl
          #children_impl
        }
    }
  .into();
}
