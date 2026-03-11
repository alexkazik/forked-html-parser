use super::node::{Node, write_html_list};
#[cfg(feature = "source-span")]
use super::span::SourceSpan;
use crate::dom::vecmap::VecMap;
use crate::for_each::ForEach;
use crate::{Attribute, VecSet};
use ownable::{IntoOwned, ToBorrowed, ToOwned};
use serde::Serialize;
use std::borrow::Cow;
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

pub type Attributes<'a> = VecMap<Cow<'a, str>, Option<Attribute<'a>>>;

/// Most of the parsed html nodes are elements, except for text
#[derive(Debug, Serialize, PartialEq, IntoOwned, ToBorrowed, ToOwned)]
#[serde(rename_all = "camelCase")]
pub struct Element<'a> {
    /// The id of the element
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Attribute<'a>>,

    /// The name / tag of the element
    pub name: Cow<'a, str>,

    /// The element variant, if it is of type void or not
    #[ownable(clone)]
    pub variant: ElementVariant,

    /// All of the elements attributes, except id and class
    #[serde(skip_serializing_if = "VecMap::is_empty")]
    pub attributes: Attributes<'a>,

    /// All of the elements classes
    #[serde(skip_serializing_if = "VecSet::is_empty")]
    pub classes: VecSet<Attribute<'a>>,

    /// All of the elements child nodes
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<Node<'a>>,

    #[cfg(feature = "source-span")]
    /// Span of the element in the parsed source
    #[serde(skip)]
    pub source_span: SourceSpan<'a>,
}

impl Default for Element<'static> {
    fn default() -> Self {
        Self {
            id: None,
            name: "".into(),
            variant: ElementVariant::Void,
            classes: Default::default(),
            attributes: Default::default(),
            children: vec![],
            #[cfg(feature = "source-span")]
            source_span: SourceSpan::default(),
        }
    }
}

impl<'a> ForEach<'a> for Element<'a> {
    #[inline]
    fn root(&self) -> &[Node<'a>] {
        self.children.as_slice()
    }

    #[inline]
    fn root_mut(&mut self) -> &mut [Node<'a>] {
        self.children.as_mut_slice()
    }
}

impl Element<'_> {
    #[inline(always)]
    pub fn to_html(&self) -> String {
        let mut result = String::new();
        self.write_html(&mut result);
        result
    }

    pub fn write_html(&self, writer: &mut String) {
        let e = self;

        writer.push('<');
        writer.push_str(&e.name);
        if !e.classes.is_empty() {
            writer.push_str(" class=\"");
            writer.push_str(
                &e.classes
                    .iter()
                    .map(|c| c.as_str())
                    .collect::<Vec<_>>()
                    .join(" "),
            );
            writer.push('"');
        }
        if let Some(id) = &e.id {
            writer.push(' ');
            writer.push_str("id");
            writer.push('=');
            writer.push('"');
            writer.push_str(id);
            writer.push('"');
        }
        for (k, v) in e.attributes.iter() {
            writer.push(' ');
            writer.push_str(k);
            if let Some(v) = v {
                writer.push('=');
                writer.push('"');
                writer.push_str(v);
                writer.push('"');
            }
        }
        if e.variant == ElementVariant::Normal {
            writer.push('>');
            write_html_list(writer, &e.children);
            writer.push('<');
            writer.push('/');
            writer.push_str(&e.name);
            writer.push('>');
        } else {
            writer.push('/');
            writer.push('>');
        }
    }
}
