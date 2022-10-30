use itertools::multiunzip;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    self,
    parse::Parser,
    parse_macro_input,
    DeriveInput,
};

#[derive(Default)]
struct Features {
    has_children: bool,
    is_child: bool,
}

#[proc_macro_attribute]
pub fn create_node(
    args: TokenStream,
    input: TokenStream,
) -> TokenStream {
    let mut features: Features = Default::default();
    for f in
    syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated
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
        _ => {
            panic!("`this derive macro only works on structs with named fields")
        },
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
                self.as_node().shallow_child_nodes_iter().collect()
            }

            #[napi(getter)]
            pub fn get_children(
                &self
            ) -> Vec<napi::bindgen_prelude::Reference<crate::Element>>
            {
                self.as_node().shallow_child_nodes_iter().collect()
            }

            #[napi]
            pub fn append(
                &self,
                child_node_or_text: napi::Either<crate::ChildNode, String>,
            ) -> napi::Result<()> {
                self.as_node().append(self.env, child_node_or_text)
            }

            #[napi]
            pub fn prepend(
                &self,
                child_node_or_text: napi::Either<crate::ChildNode, String>,
            ) -> napi::Result<()> {
                self.as_node().prepend(self.env, child_node_or_text)
            }

            #[napi(
                ts_generic_types = "T extends ChildNode",
                ts_args_type = "child: T",
                ts_return_type = "T"
            )]
            pub fn append_child(
                &self,
                child: crate::ChildNode,
            ) -> napi::Result<crate::ChildNode> {
                self.as_node()
                    .insert_node(&child, &crate::InsertPosition::Append)?;

                Ok(child)
            }

            #[napi(
                ts_generic_types = "T extends ChildNode",
                ts_args_type = "new_node: T, reference_node: ChildNode",
                ts_return_type = "T"
            )]
            pub fn insert_before(
                &self,
                new_node: crate::ChildNode,
                reference_node: crate::ChildNode,
            ) -> napi::Result<crate::ChildNode> {
                self.as_node()
                    .insert_before(&new_node, &reference_node.into())?;

                Ok(new_node)
            }

            #[napi(
                ts_generic_types = "T extends ChildNode",
                ts_args_type = "child: T",
                ts_return_type = "T"
            )]
            pub fn remove_child(
                &self,
                child: crate::ChildNode,
            ) -> napi::Result<crate::ChildNode> {
                self.as_node().remove_node(&child)?;

                Ok(child)
            }

            #[napi]
            pub fn get_element_by_id(
                &self,
                id: String,
            ) -> Option<napi::bindgen_prelude::Reference<crate::Element>>
            {
                self.as_node().deep_child_nodes_iter().find(
                    |e: &napi::bindgen_prelude::Reference<crate::Element>| {
                        e.get_id() == id
                    },
                )
            }

            #[napi]
            pub fn get_elements_by_class_name(
                &self,
                class_name: String,
            ) -> Vec<napi::bindgen_prelude::Reference<crate::Element>>
            {
                self.as_node()
                    .deep_child_nodes_iter()
                    .filter(
                        |e: &napi::bindgen_prelude::Reference<
                            crate::Element,
                        >| {
                            e.get_class_name() == class_name
                        },
                    )
                    .collect()
            }

            #[napi]
            pub fn get_elements_by_tag_name(
                &self,
                qualified_name: String,
            ) -> Vec<napi::bindgen_prelude::Reference<crate::Element>>
            {
                let tag_name: &str = &qualified_name;

                self.as_node()
                    .deep_child_nodes_iter()
                    .filter(
                        |e: &napi::bindgen_prelude::Reference<
                            crate::Element,
                        >| {
                            e.get_tag_name().eq_ignore_ascii_case(tag_name)
                        },
                    )
                    .collect()
            }

            #[napi]
            pub fn query_selector(
                &self,
                selectors: String,
            ) -> napi::Result<
                Option<napi::bindgen_prelude::Reference<crate::Element>>,
            > {
                self.as_node().selectors_iter(selectors)?.try_next()
            }

            #[napi]
            pub fn query_selector_all(
                &self,
                selectors: String,
            ) -> napi::Result<
                Vec<napi::bindgen_prelude::Reference<crate::Element>>,
            > {
                self.as_node().selectors_iter(selectors)?.collect()
            }

            #[napi(getter)]
            pub fn get_first_child(&self) -> Option<crate::ChildNode> {
                self.as_node().shallow_child_nodes_iter().next()
            }

            #[napi(getter)]
            pub fn get_first_element_child(
                &self
            ) -> Option<napi::bindgen_prelude::Reference<crate::Element>>
            {
                self.as_node().shallow_child_nodes_iter().next()
            }

            #[napi(getter)]
            pub fn get_last_child(&self) -> Option<crate::ChildNode> {
                self.as_node().shallow_child_nodes_iter().next_back()
            }

            #[napi(getter)]
            pub fn get_last_element_child(
                &self
            ) -> Option<napi::bindgen_prelude::Reference<crate::Element>>
            {
                self.as_node().shallow_child_nodes_iter().next_back()
            }

            #[napi]
            pub fn normalize(&self) -> napi::Result<()> {
                self.as_node().normalize()
            }
        ),
        false => quote!(),
    };

    let is_child_impl = match features.is_child {
        true => quote! {
            fn as_child_node(&self) -> crate::ChildNode {
                let child_node: crate::ChildNode = self.into();
                child_node
            }

            #[napi(getter)]
            pub fn get_parent_element(&self) ->
                napi::Result<Option<napi::bindgen_prelude::WeakReference<crate::Element>>> {
                self.as_node().parent_iterator().try_next()
            }

            #[napi(getter)]
            pub fn get_parent_node(&self) ->
                napi::Result<Option<crate::ParentNode>> {
                self.as_node().parent_iterator().try_next()
            }

            #[napi(getter)]
            pub fn get_owner_document(
                &self,
            ) -> napi::Result<Option<napi::bindgen_prelude::WeakReference<crate::Document>>> {
                self.as_node().parent_iterator().try_next()
            }

            #[napi]
            pub fn remove(&self) -> napi::Result<()> {
                self.as_child_node().remove()
            }

            #[napi(getter)]
            pub fn get_previous_sibling(&self) ->
                napi::Result<Option<crate::ChildNode>> {

                Ok(self.as_node().previous_iterator()?.next())
            }

            #[napi(getter)]
            pub fn get_previous_element_sibling(&self) ->
                napi::Result<Option<napi::bindgen_prelude::Reference<crate::Element>>> {

                Ok(self.as_node().previous_iterator()?.next())
            }

            #[napi(getter)]
            pub fn get_next_sibling(&self) ->
                napi::Result<Option<crate::ChildNode>> {

                Ok(self.as_node().next_iterator()?.next())
            }

            #[napi(getter)]
            pub fn get_next_element_sibling(&self) ->
                napi::Result<Option<napi::bindgen_prelude::Reference<crate::Element>>> {

                Ok(self.as_node().next_iterator()?.next())
            }
        },
        false => quote! {},
    };

    return quote! {
      #[napi]
      pub struct #name {
        pub(crate) env: napi::Env,
        pub(crate) node_data: crate::NodeData,
        pub(crate) cyclic_reference: crate::CyclicReference<Self>,
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

          crate::CyclicReference::<Self>::new_cyclic(env, |cyclic_reference| {
            let inner = Self {
              #(#argument_fields)*
              env,
              id: crate::get_id(),
              node_data: crate::NodeData::new(env),
              cyclic_reference,
            };

            Self::into_reference(inner, env)
          })
        }

        pub(crate) fn get_node_data(&self) -> crate::NodeData {
            self.node_data.clone()
        }

        #[napi(getter)]
        pub fn get_node_name(&self) -> String {
            self.as_node().get_node_name()
        }

        fn as_node(&self) -> crate::Node {
            let node: crate::Node = self.into();
            node
        }

        #is_child_impl
        #has_children_impl
      }
    }
    .into();
}
