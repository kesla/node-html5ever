use html5ever::{
    parse_document,
    parse_fragment,
    tendril::TendrilSink,
    tree_builder::{
        NodeOrText,
        TreeSink,
    },
    ParseOpts,
};
use napi::{
    bindgen_prelude::Reference,
    Env,
    Result,
};

use crate::{
    serialize,
    ChildNode,
    Comment,
    Document,
    DocumentFragment,
    DocumentType,
    Element,
    InsertPosition,
    LazyReference,
    Node,
    QuirksMode,
    Text,
    Window,
};

#[napi]
pub struct Html5everDom {
    document_reference: Reference<Document>,
    window_reference: Reference<Window>,

    #[napi(writable = false)]
    pub errors: Vec<String>,

    env: Env,
}

#[napi]
impl Html5everDom {
    #[napi(constructor)]
    pub fn new(
        env: Env,
        html: String,
    ) -> Result<Html5everDom> {
        let dom: Html5everDom =
            parse_document(Self::create_sink(env)?, ParseOpts::default())
                .one(html);

        Ok(dom)
    }

    #[napi]
    pub fn create_document_fragment(
        env: Env,
        html: String,
        maybe_quirks_mode: Option<QuirksMode>,
    ) -> Result<Reference<DocumentFragment>> {
        let quirks_mode = maybe_quirks_mode.unwrap_or(QuirksMode::NoQuirks);
        let fragment: Reference<DocumentFragment> =
            DocumentFragment::new_reference(env, quirks_mode)?;
        let fragment_node: Node = fragment.clone(env)?.into();

        Self::parse_and_append(env, fragment_node, html)?;

        Ok(fragment)
    }

    pub(crate) fn parse_and_append(
        env: Env,
        node: Node,
        html: String,
    ) -> Result<()> {
        let dom: Html5everDom = parse_fragment(
            Self::create_sink(env)?,
            ParseOpts::default(),
            node.get_qual_name(),
            Vec::new(),
        )
        .one(html);

        let document_node: Node =
            dom.document_reference.get_document_element()?.into();

        let tmp: Vec<ChildNode> =
            document_node.shallow_child_nodes_iter().collect();

        for child in tmp {
            node.insert_node(&child.clone(), &InsertPosition::Append)?;
        }

        Ok(())
    }

    fn create_sink(env: Env) -> Result<Html5everDom> {
        let document_reference =
            Document::new_reference(env, QuirksMode::NoQuirks)?;
        let window_reference =
            Window::new_reference(env, document_reference.clone(env)?)?;

        let sink = Html5everDom {
            window_reference,
            document_reference,
            errors: vec![],
            env,
        };

        Ok(sink)
    }

    #[napi(getter)]
    pub fn get_window(&mut self) -> Result<Reference<Window>> {
        self.window_reference.clone(self.env)
    }

    #[napi(getter)]
    pub fn get_quirks_mode(&self) -> QuirksMode {
        self.document_reference.quirks_mode
    }

    #[napi]
    pub fn serialize(&self) -> Result<String> {
        let node: Node = self.document_reference.clone(self.env)?.into();

        Ok(serialize(
            node,
            html5ever::serialize::TraversalScope::ChildrenOnly(None),
        ))
    }
}

#[allow(unused_variables)]
impl TreeSink for Html5everDom {
    type Handle = Node;
    type Output = Self;

    fn finish(self) -> Self::Output {
        self
    }

    fn parse_error(
        &mut self,
        msg: std::borrow::Cow<'static, str>,
    ) {
        self.errors.push(msg.into_owned());
    }

    fn get_document(&mut self) -> Self::Handle {
        self.document_reference.clone(self.env).unwrap().into()
    }

    fn elem_name<'a>(
        &'a self,
        target: &'a Self::Handle,
    ) -> html5ever::ExpandedName<'a> {
        let element: &Reference<Element> = target.as_element().unwrap();
        element.name.expanded()
    }

    fn create_element(
        &mut self,
        name: html5ever::QualName,
        attrs: Vec<html5ever::Attribute>,
        // TODO: set flags
        _flags: html5ever::tree_builder::ElementFlags,
    ) -> Self::Handle {
        let r = Element::new_reference(
            self.env,
            attrs.into(),
            name,
            LazyReference::new(self.env),
            LazyReference::new(self.env),
        )
        .unwrap();
        r.into()
    }

    fn create_comment(
        &mut self,
        text: html5ever::tendril::StrTendril,
    ) -> Self::Handle {
        let r = Comment::new_reference(self.env, text.to_string()).unwrap();
        r.into()
    }

    fn create_pi(
        &mut self,
        target: html5ever::tendril::StrTendril,
        data: html5ever::tendril::StrTendril,
    ) -> Self::Handle {
        todo!()
    }

    fn append(
        &mut self,
        parent: &Self::Handle,
        child: NodeOrText<Self::Handle>,
    ) {
        let child: ChildNode = match child {
            NodeOrText::AppendNode(node) => node.into(),
            NodeOrText::AppendText(content) => {
                let r =
                    Text::new_reference(self.env, content.to_string()).unwrap();
                r.into()
            },
        };

        parent.insert_node(&child, &InsertPosition::Append).unwrap();
    }

    fn append_based_on_parent_node(
        &mut self,
        element: &Self::Handle,
        prev_element: &Self::Handle,
        child: html5ever::tree_builder::NodeOrText<Self::Handle>,
    ) {
        todo!()
    }

    fn append_doctype_to_document(
        &mut self,
        name: html5ever::tendril::StrTendril,
        public_id: html5ever::tendril::StrTendril,
        system_id: html5ever::tendril::StrTendril,
    ) {
        let r = DocumentType::new_reference(
            self.env,
            name.to_string(),
            public_id.to_string(),
            system_id.to_string(),
        )
        .unwrap();
        let doc_type: Node = r.into();
        let child = NodeOrText::AppendNode(doc_type);
        let node = self.get_document();
        self.append(&node, child);
    }

    fn get_template_contents(
        &mut self,
        target: &Self::Handle,
    ) -> Self::Handle {
        todo!()
    }

    fn same_node(
        &self,
        x: &Self::Handle,
        y: &Self::Handle,
    ) -> bool {
        todo!()
    }

    fn set_quirks_mode(
        &mut self,
        mode: html5ever::tree_builder::QuirksMode,
    ) {
        self.document_reference.quirks_mode = mode.into();
    }

    fn append_before_sibling(
        &mut self,
        sibling: &Self::Handle,
        new_node: html5ever::tree_builder::NodeOrText<Self::Handle>,
    ) {
        todo!()
    }

    fn add_attrs_if_missing(
        &mut self,
        target: &Self::Handle,
        attrs: Vec<html5ever::Attribute>,
    ) {
        todo!()
    }

    fn remove_from_parent(
        &mut self,
        target: &Self::Handle,
    ) {
        todo!()
    }

    fn reparent_children(
        &mut self,
        node: &Self::Handle,
        new_parent: &Self::Handle,
    ) {
        todo!()
    }
}
