pub use crate::api::{
    AstNode, BoxedAstNode, DictionaryEntryNode, DictionaryNode, EmptyNode, ListEntryNode, ListNode,
    LogicalNode, NodeContext, NumberNode, ObjectEntryNode, ObjectNode, ParseError,
    ParseErrorExpectation, TextNode,
};

pub trait EqIgnoreContext {
    fn eq_ignore_context(&self, other: &Self) -> bool;

    fn ne_ignore_context(&self, other: &Self) -> bool {
        !self.eq_ignore_context(other)
    }
}

impl AstNode {
    pub const fn is_empty(&self) -> bool {
        matches!(self, AstNode::Empty(_))
    }

    pub const fn is_logical(&self) -> bool {
        matches!(self, AstNode::Logical(_))
    }

    pub const fn is_number(&self) -> bool {
        matches!(self, AstNode::Number(_))
    }

    pub const fn is_text(&self) -> bool {
        matches!(self, AstNode::Text(_))
    }

    pub const fn is_object(&self) -> bool {
        matches!(self, AstNode::Object(_))
    }

    pub const fn is_dictionary(&self) -> bool {
        matches!(self, AstNode::Dictionary(_))
    }

    pub const fn is_list(&self) -> bool {
        matches!(self, AstNode::List(_))
    }

    pub fn unwrap_empty(self) -> EmptyNode {
        match self {
            AstNode::Empty(node) => node,
            _ => panic!("Expected an empty node"),
        }
    }

    pub fn unwrap_logical(self) -> LogicalNode {
        match self {
            AstNode::Logical(node) => node,
            _ => panic!("Expected a logical node"),
        }
    }

    pub fn unwrap_number(self) -> NumberNode {
        match self {
            AstNode::Number(node) => node,
            _ => panic!("Expected a number node"),
        }
    }

    pub fn unwrap_text(self) -> TextNode {
        match self {
            AstNode::Text(node) => node,
            _ => panic!("Expected a text node"),
        }
    }

    pub fn unwrap_object(self) -> ObjectNode {
        match self {
            AstNode::Object(node) => node,
            _ => panic!("Expected an object node"),
        }
    }

    pub fn unwrap_dictionary(self) -> DictionaryNode {
        match self {
            AstNode::Dictionary(node) => node,
            _ => panic!("Expected a dictionary node"),
        }
    }

    pub fn unwrap_list(self) -> ListNode {
        match self {
            AstNode::List(node) => node,
            _ => panic!("Expected a list node"),
        }
    }
}
