#![deny(clippy::all)]

wai_bindgen_rust::export!("api.wai");

mod api_trait_impls;
pub mod node;
pub mod parser;

use std::fmt::Debug;

use node::{DictionaryEntryNode, ObjectEntryNode};
use wai_bindgen_rust::Handle;

use crate::api::{AstNode, ParseError};

pub struct Api;

/// A boxed abstract syntax tree node.
///
/// Used to avoid recursions.
#[derive(Debug, Clone, PartialEq)]
pub struct BoxedAstNode(pub AstNode);

impl api::BoxedAstNode for BoxedAstNode {
    fn get(&self) -> AstNode {
        self.0.clone()
    }
}

impl api::Api for Api {
    #[inline]
    fn parse(input: String) -> Result<AstNode, ParseError> {
        parser::parse(&input)
    }

    fn display(root: AstNode, pretty: bool) -> String {
        let root = DisplayableAst::from(&root);
        if pretty {
            format!("{:#?}", root)
        } else {
            format!("{:?}", root)
        }
    }
}

enum DisplayableAst<'a> {
    Empty(&'a node::EmptyNode),
    Logical(&'a node::LogicalNode),
    Number(&'a node::NumberNode),
    Text(&'a node::TextNode),
    Object(&'a node::ObjectNode),
    Dictionary(&'a node::DictionaryNode),
    List(&'a node::ListNode),
    Boxed(&'a Handle<BoxedAstNode>),
}

impl<'a> From<&'a AstNode> for DisplayableAst<'a> {
    fn from(value: &'a AstNode) -> Self {
        match value {
            AstNode::Empty(n) => Self::Empty(n),
            AstNode::Logical(n) => Self::Logical(n),
            AstNode::Number(n) => Self::Number(n),
            AstNode::Text(n) => Self::Text(n),
            AstNode::Object(n) => Self::Object(n),
            AstNode::Dictionary(n) => Self::Dictionary(n),
            AstNode::List(n) => Self::List(n),
        }
    }
}

impl Debug for DisplayableAst<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Empty(n) => n.fmt(f),
            Self::Logical(n) => n.fmt(f),
            Self::Number(n) => n.fmt(f),
            Self::Text(n) => n.fmt(f),
            Self::Object(n) => f
                .debug_struct("ObjectNode")
                .field("name", &n.name)
                .field(
                    "entries",
                    &n.entries
                        .iter()
                        .map(DisplayableObjectEntry)
                        .collect::<Vec<_>>(),
                )
                .field("context", &n.context)
                .finish(),
            Self::Dictionary(n) => f
                .debug_struct("DictionaryNode")
                .field(
                    "entries",
                    &n.entries
                        .iter()
                        .map(DisplayableDictionaryEntry)
                        .collect::<Vec<_>>(),
                )
                .field("context", &n.context)
                .finish(),
            Self::List(n) => f
                .debug_struct("ListNode")
                .field(
                    "entries",
                    &n.entries
                        .iter()
                        .map(DisplayableAst::from)
                        .collect::<Vec<_>>(),
                )
                .field("context", &n.context)
                .finish(),
            Self::Boxed(n) => DisplayableAst::from(&n.0).fmt(f),
        }
    }
}

struct DisplayableObjectEntry<'a>(&'a ObjectEntryNode);

impl Debug for DisplayableObjectEntry<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ObjectEntryNode")
            .field("key", &self.0.key)
            .field("value", &DisplayableAst::Boxed(&self.0.value))
            .field("context", &self.0.context)
            .finish()
    }
}

struct DisplayableDictionaryEntry<'a>(&'a DictionaryEntryNode);

impl Debug for DisplayableDictionaryEntry<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DictionaryEntryNode")
            .field("key", &self.0.key)
            .field("value", &DisplayableAst::Boxed(&self.0.value))
            .field("context", &self.0.context)
            .finish()
    }
}

impl<'a> From<&'a Handle<BoxedAstNode>> for DisplayableAst<'a> {
    fn from(handle: &'a Handle<BoxedAstNode>) -> Self {
        DisplayableAst::Boxed(handle)
    }
}
