use itertools::multiunzip;
use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn create_node(args: TokenStream, input: TokenStream) -> TokenStream {
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
  let has_parent = args.to_string().contains("parent");

  let parent_field = match has_parent {
    true => quote!(
      pub(crate) parent:
        std::cell::RefCell<Option<napi::Either<
          napi::bindgen_prelude::WeakReference<crate::Element>,
          napi::bindgen_prelude::WeakReference<crate::Document>
        >>>,
    ),
    false => quote!(),
  };
  let parent_init = match has_parent {
    true => quote!(parent: std::cell::RefCell::new(None),),
    false => quote!(),
  };
  let parent_impl = match has_parent {
    true => quote! {
      #[napi(getter)]
      pub fn get_parent_element(&self) ->
          Option<napi::bindgen_prelude::WeakReference<crate::Element>> {

        let parent_node = self.parent.borrow();

        match parent_node.as_ref() {
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
            napi::bindgen_prelude::WeakReference<crate::Element>,
            napi::bindgen_prelude::WeakReference<crate::Document>
          >> {

            let maybe_reference = self.parent.borrow();

        maybe_reference.as_ref().map(|value| match value {
          napi::Either::A(element) => napi::Either::A(element.clone()),
          napi::Either::B(document) => napi::Either::B(document.clone()),
        })
      }

      #[napi]
      pub fn remove(&mut self) -> napi::Result<()> {
        let maybe_handle = self.get_parent_handle()?;

        match maybe_handle {
          Some(parent) => {
            let child: crate::Handle = self.get_handle();

            parent.remove_handle(child);
          }
          None => {}
        }

        Ok(())
      }

      pub(crate) fn get_parent_handle(&self) -> napi::Result<Option<crate::Handle>> {
        let parent_node = self.parent.borrow();

        let maybe_handle: Option<crate::Handle> = match parent_node.as_ref() {
          Some(element_or_document) => match element_or_document {
            napi::Either::A(weak_reference) => {
              weak_reference.upgrade(self.env)?.map(|r| r.get_handle())
            }
            napi::Either::B(weak_reference) => {
              weak_reference.upgrade(self.env)?.map(|r| r.get_handle())
            }
          },
          None => None,
        };
        Ok(maybe_handle)
      }
    },
    false => quote! {},
  };

  return quote! {
      #[napi]
      pub struct #name {
        #parent_field
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
            weak_reference: None,
          };

          let mut r = Self::into_reference(inner, env)?;
          r.weak_reference = Some(r.clone(env)?.downgrade());
          Ok(r)
        }

        pub(crate) fn get_handle(&self) -> crate::Handle {
          let weak_reference = self.weak_reference.as_ref().unwrap();
          let reference = weak_reference.upgrade(self.env).unwrap().unwrap();

          self.lazy_weak_handle.get_or_init(reference)
        }

        #parent_impl
      }
  }
  .into();
}
