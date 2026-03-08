use super::element::Element;
use crate::Text;
use ownable::{IntoOwned, ToBorrowed, ToOwned};
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq, IntoOwned, ToBorrowed, ToOwned)]
#[serde(untagged)]
pub enum Node<'a> {
    Text(Text<'a>),
    Element(Element<'a>),
    Comment(Text<'a>),
}

impl<'a> Node<'a> {
    pub fn text(&self) -> Option<&Text<'a>> {
        match self {
            Node::Text(t) => Some(t),
            _ => None,
        }
    }

    pub fn text_str(&self) -> Option<&str> {
        match self {
            Node::Text(t) => Some(t),
            _ => None,
        }
    }

    pub fn element(&self) -> Option<&Element<'a>> {
        match self {
            Node::Element(e) => Some(e),
            _ => None,
        }
    }

    pub fn comment(&self) -> Option<&Text<'a>> {
        match self {
            Node::Comment(t) => Some(t),
            _ => None,
        }
    }

    pub fn comment_str(&self) -> Option<&str> {
        match self {
            Node::Comment(t) => Some(t),
            _ => None,
        }
    }
}

impl<'a> IntoIterator for &'a Node<'a> {
    type Item = &'a Node<'a>;
    type IntoIter = NodeIntoIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        NodeIntoIterator {
            node: self,
            index: vec![],
        }
    }
}

pub struct NodeIntoIterator<'a> {
    node: &'a Node<'a>,
    // We add/remove to this vec each time we go up/down a node three
    index: Vec<(usize, &'a Node<'a>)>,
}

impl<'a> Iterator for NodeIntoIterator<'a> {
    type Item = &'a Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        // Get first child
        let child = match self.node {
            Node::Element(e) => e.children.first(),
            _ => None,
        };

        match child {
            // If element has child, return child
            Some(child) => {
                self.index.push((0, self.node));
                self.node = child;
                Some(child)
            }
            // If element doesn't have a child, but is a child of another node
            None if !self.index.is_empty() => {
                let mut has_finished = false;
                let mut next_node = None;

                while !has_finished {
                    // Try to get the next sibling of the parent node
                    if let Some((sibling_index, parent)) = self.index.pop() {
                        let next_sibling = sibling_index + 1;
                        let sibling = if let Node::Element(e) = parent {
                            e.children.get(next_sibling)
                        } else {
                            None
                        };
                        if sibling.is_some() {
                            has_finished = true;
                            self.index.push((next_sibling, parent));
                            next_node = sibling;
                        } else {
                            continue;
                        }
                    // Break of there are no more parents
                    } else {
                        has_finished = true;
                    }
                }

                if let Some(next_node) = next_node {
                    self.node = next_node;
                }

                next_node
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_utillity_functions() {
        let node = Node::Text("test".into());

        assert_eq!(node.text_str(), Some("test"));
        assert_eq!(node.element(), None);
        assert_eq!(node.comment_str(), None);

        let node = Node::Element(Element::default());

        assert_eq!(node.text_str(), None);
        assert_eq!(node.element(), Some(&Element::default()));
        assert_eq!(node.comment_str(), None);

        let node = Node::Comment("test".into());

        assert_eq!(node.text_str(), None);
        assert_eq!(node.element(), None);
        assert_eq!(node.comment_str(), Some("test"));
    }
}
