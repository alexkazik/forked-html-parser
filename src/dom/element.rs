use super::node::Node;
#[cfg(feature = "source-span")]
use super::span::SourceSpan;
use crate::VecSet;
use crate::dom::vecmap::VecMap;
use serde::Serialize;
use std::default::Default;

/// Normal: `<div></div>` or Void: `<meta/>`and `<meta>`
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
// TODO: Align with: https://html.spec.whatwg.org/multipage/syntax.html#elements-2
pub enum ElementVariant {
    /// A normal element can have children, ex: <div></div>.
    Normal,
    /// A void element can't have children, ex: <meta /> and <meta>
    Void,
}

pub type Attributes = VecMap<String, Option<String>>;

/// Most of the parsed html nodes are elements, except for text
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Element {
    /// The id of the element
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The name / tag of the element
    pub name: String,

    /// The element variant, if it is of type void or not
    pub variant: ElementVariant,

    /// All of the elements attributes, except id and class
    #[serde(skip_serializing_if = "VecMap::is_empty")]
    pub attributes: Attributes,

    /// All of the elements classes
    #[serde(skip_serializing_if = "VecSet::is_empty")]
    pub classes: VecSet<String>,

    /// All of the elements child nodes
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<Node>,

    #[cfg(feature = "source-span")]
    /// Span of the element in the parsed source
    #[serde(skip)]
    pub source_span: SourceSpan,
}

impl Default for Element {
    fn default() -> Self {
        Self {
            id: None,
            name: "".to_string(),
            variant: ElementVariant::Void,
            classes: VecSet::default(),
            attributes: VecMap::default(),
            children: vec![],
            #[cfg(feature = "source-span")]
            source_span: SourceSpan::default(),
        }
    }
}
