use std::collections::HashMap;

pub enum NodeTypeEnum {
    Element = 1,
    Attribute = 2,
    Text = 3,
    CDATASection = 4,
    ProcessingInstruction = 7,
    Comment = 8,
    Document = 9,
    DocumentType = 10,
    DocumentFragment = 11,
}

lazy_static! {
    pub static ref NODE_TYPE_MAP: HashMap<&'static str, u32> = vec![
        ("ELEMENT_NODE", NodeTypeEnum::Element as u32),
        ("ATTRIBUTE_NODE", NodeTypeEnum::Attribute as u32),
        ("TEXT_NODE", NodeTypeEnum::Text as u32),
        ("CDATA_SECTION_NODE", NodeTypeEnum::CDATASection as u32),
        (
            "PROCESSING_INSTRUCTION_NODE",
            NodeTypeEnum::ProcessingInstruction as u32
        ),
        ("COMMENT_NODE", NodeTypeEnum::Comment as u32),
        ("DOCUMENT_NODE", NodeTypeEnum::Document as u32),
        ("DOCUMENT_TYPE_NODE", NodeTypeEnum::DocumentType as u32),
        (
            "DOCUMENT_FRAGMENT_NODE",
            NodeTypeEnum::DocumentFragment as u32
        ),
    ]
    .into_iter()
    .collect();
}
