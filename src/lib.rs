#![allow(dead_code, unreachable_code)] // FIXME: Remove this when the code is complete

wai_bindgen_rust::export!("api.wai");

mod api_trait_impls;
pub mod parser;

use crate::api::{AstNode, ParseError};

mod node {
    pub use crate::api::{
        AstNode, BoxedAstNode, DictionaryEntryNode, DictionaryNode, EmptyNode, ListEntryNode,
        ListNode, LogicalNode, NodeContext, Number, NumberNode, ObjectEntryNode, ObjectNode,
        ParseError, ParseErrorKind, TextNode,
    };
}

pub struct Api;

/// A boxed abstract syntax tree node.
///
/// Used to avoid recursions.
#[derive(Debug, Clone, PartialEq, Eq)]
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
}
