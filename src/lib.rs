#![deny(clippy::all, rust_2018_idioms)]

#[cfg(feature = "wasm")]
wai_bindgen_rust::export!("api.wai");

#[cfg(feature = "wasm")]
mod api_trait_impls;
#[cfg(feature = "wasm")]
pub mod node;
pub mod parser;

#[cfg(feature = "wasm")]
use std::fmt::Debug;

#[cfg(feature = "wasm")]
use node::{DictionaryEntryNode, ObjectEntryNode, ParseErrorExpectation};
#[cfg(feature = "wasm")]
use pest::error::{InputLocation, LineColLocation};
#[cfg(feature = "wasm")]
use wai_bindgen_rust::Handle;

#[cfg(feature = "wasm")]
use crate::api::{AstNode, ParseError};
#[cfg(feature = "wasm")]
pub struct Api;

#[cfg(feature = "wasm")]
/// A boxed abstract syntax tree node.
///
/// Used to avoid recursions.
#[derive(Debug, Clone, PartialEq)]
pub struct BoxedAstNode(pub AstNode);

#[cfg(feature = "wasm")]
impl api::BoxedAstNode for BoxedAstNode {
    fn get(&self) -> AstNode {
        self.0.clone()
    }
}

#[cfg(feature = "wasm")]
impl api::Api for Api {
    #[inline]
    fn parse(input: String) -> Result<AstNode, ParseError> {
        parser::parse(&input).map_err(|err| {
            let (line, column) = match err.line_col {
                LineColLocation::Pos((line, col)) => (line as u64, col as u64),
                LineColLocation::Span((line, col), _) => (line as u64, col as u64),
            };
            let index = match err.location {
                InputLocation::Pos(pos) => pos as u64,
                InputLocation::Span((start, _)) => start as u64,
            };
            ParseError {
                expectation: ParseErrorExpectation::AstNode,
                line,
                column,
                index,
                info: err.to_string(),
            }
        })
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

#[cfg(feature = "wasm")]
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

#[cfg(feature = "wasm")]
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

#[cfg(feature = "wasm")]
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

#[cfg(feature = "wasm")]
struct DisplayableObjectEntry<'a>(&'a ObjectEntryNode);

#[cfg(feature = "wasm")]
impl Debug for DisplayableObjectEntry<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ObjectEntryNode")
            .field("key", &self.0.key)
            .field("value", &DisplayableAst::Boxed(&self.0.value))
            .field("context", &self.0.context)
            .finish()
    }
}

#[cfg(feature = "wasm")]
struct DisplayableDictionaryEntry<'a>(&'a DictionaryEntryNode);

#[cfg(feature = "wasm")]
impl Debug for DisplayableDictionaryEntry<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DictionaryEntryNode")
            .field("key", &self.0.key)
            .field("value", &DisplayableAst::Boxed(&self.0.value))
            .field("context", &self.0.context)
            .finish()
    }
}

#[cfg(feature = "wasm")]
impl<'a> From<&'a Handle<BoxedAstNode>> for DisplayableAst<'a> {
    fn from(handle: &'a Handle<BoxedAstNode>) -> Self {
        DisplayableAst::Boxed(handle)
    }
}
