use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse_macro_input, DeriveInput, parse::Parser};

#[proc_macro_attribute]
pub fn add_node_fields(_args: TokenStream, input: TokenStream) -> TokenStream  {
    let mut ast = parse_macro_input!(input as DeriveInput);
    match &mut ast.data {
        syn::Data::Struct(ref mut struct_data) => {           
            match &mut struct_data.fields {
                syn::Fields::Named(fields) => {
                    fields
                        .named
                        .push(syn::Field::parse_named.parse2(quote! {
                          pub(crate) parent: Option<Either<WeakReference<Element>, WeakReference<Document>>>
                        }).unwrap());
                }   
                _ => {
                    ()
                }
            }              
            
            return quote! {
                #ast
            }.into();
        }
        _ => panic!("`add_field` has to be used with structs "),
    }
}

#[proc_macro_derive(Node)]
pub fn node_macro_derive(input: TokenStream) -> TokenStream {
  let ast: syn::DeriveInput = syn::parse(input).unwrap();
  let name = ast.ident;
  let gen = quote!(
    #[napi]
    impl #name {
      #[napi(getter)]
      pub fn get_parent_element(&self) -> Option<WeakReference<Element>> {
        let parent_node = self.get_parent_node();

        match parent_node {
          Some(element_or_document) => {
            match (element_or_document) {
              Either::A(element) => Some(element),
              Either::B(_) => None,
            }
          },
          None => None
        }
      }

      #[napi(getter)]
      pub fn get_parent_node(&self) -> Option<Either<WeakReference<Element>, WeakReference<Document>>> {
        let maybe_reference = self.parent.as_ref();

        maybe_reference.map(|value| match value {
          Either::A(element) => Either::A(element.clone()),
          Either::B(document) => Either::B(document.clone()),
        })
      }
    }
  );
  gen.into()
}
