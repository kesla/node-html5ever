use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Node)]
pub fn node_macro_derive(input: TokenStream) -> TokenStream {
  let ast: syn::DeriveInput = syn::parse(input).unwrap();
  let name = ast.ident;
  let gen = quote!(
    #[napi]
    impl #name {
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
