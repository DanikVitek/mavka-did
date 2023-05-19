use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

use crate::api::{AstNode, NodeContext, Number};

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
struct DidParser;

pub fn parse(input: &str) -> Result<Did, Error<Rule>> {
    let input = DidParser::parse(Rule::did, input)?.next().unwrap();

    fn ast_node(pair: Pair<Rule>) -> Did {
        let (line, col) = pair.line_col();
        let context = NodeContext {
            line: line as u64,
            column: col as u64,
            index: pair.as_span().start() as u64,
        };
        match pair.as_rule() {
            Rule::empty => Did::Empty(EmptyNode { context }),
            Rule::logical => Did::Logical(LogicalNode {
                value: match pair.as_str() {
                    "так" => true,
                    "ні" => false,
                    _ => unreachable!(),
                },
                context,
            }),
            Rule::number => Did::Number(NumberNode {
                value: {
                    let as_str = pair.as_str();
                    if as_str.contains('.') {
                        Number::Float(as_str.parse().unwrap())
                    } else {
                        Number::Integer(as_str.parse().unwrap())
                    }
                },
                context,
            }),
            Rule::text => Did::Text(TextNode {
                value: pair.into_inner().next().unwrap().as_str(),
                context,
            }),
            Rule::dict => Did::Dictionary(DictionaryNode {
                entries: pair
                    .into_inner()
                    .map(|pair| {
                        let mut inner_rules = pair.into_inner();
                        let key_pair = inner_rules.next().unwrap();
                        let (line, col) = key_pair.line_col();
                        let key_context = NodeContext {
                            line: line as u64,
                            column: col as u64,
                            index: key_pair.as_span().start() as u64,
                        };
                        let key = match key_pair.as_rule() {
                            Rule::number => DictionaryEntryKey::Number(NumberNode {
                                value: {
                                    let as_str = key_pair.as_str();
                                    if as_str.contains('.') {
                                        Number::Float(as_str.parse().unwrap())
                                    } else {
                                        Number::Integer(as_str.parse().unwrap())
                                    }
                                },
                                context: key_context,
                            }),
                            Rule::text => DictionaryEntryKey::Text(TextNode {
                                value: key_pair.into_inner().next().unwrap().as_str(),
                                context: key_context,
                            }),
                            Rule::ident => DictionaryEntryKey::Text(TextNode {
                                value: key_pair.as_str(),
                                context: key_context,
                            }),
                            rule => unreachable!("{rule:?} {:?}", key_pair.as_str()),
                        };
                        let value = ast_node(inner_rules.next().unwrap());
                        DictionaryEntryNode {
                            context: key_context,
                            key,
                            value,
                        }
                    })
                    .collect(),
                context,
            }),
            Rule::object => Did::Object({
                let mut inner_rules = pair.into_inner();
                let name: &str = inner_rules.next().unwrap().as_str();
                ObjectNode {
                    name: TextNode {
                        value: name,
                        context,
                    },
                    entries: inner_rules
                        .map(|pair| {
                            let mut inner_rules = pair.into_inner();
                            let key_pair = inner_rules.next().unwrap();
                            let (line, col) = key_pair.line_col();
                            let key_context = NodeContext {
                                line: line as u64,
                                column: col as u64,
                                index: key_pair.as_span().start() as u64,
                            };
                            let key = match key_pair.as_rule() {
                                Rule::ident => TextNode {
                                    value: key_pair.as_str(),
                                    context: key_context,
                                },
                                _ => unreachable!(),
                            };
                            let value = ast_node(inner_rules.next().unwrap());
                            ObjectEntryNode {
                                context: key_context,
                                key,
                                value,
                            }
                        })
                        .collect(),
                    context,
                }
            }),
            Rule::list => Did::List(ListNode {
                entries: pair.into_inner().map(ast_node).collect(),
                context,
            }),
            rule => unreachable!("{rule:?} {:?}", pair.as_str()),
        }
    }

    Ok(ast_node(input))
}

#[derive(Debug, Clone)]
pub enum Did<'inp> {
    Empty(EmptyNode),
    Logical(LogicalNode),
    Number(NumberNode),
    Text(TextNode<'inp>),
    Dictionary(DictionaryNode<'inp>),
    Object(ObjectNode<'inp>),
    List(ListNode<'inp>),
}

pub type EmptyNode = crate::api::EmptyNode;
pub type LogicalNode = crate::api::LogicalNode;
pub type NumberNode = crate::api::NumberNode;

#[derive(Debug, Clone)]
pub struct TextNode<'inp> {
    pub value: &'inp str,
    pub context: NodeContext,
}

#[derive(Debug, Clone)]
pub struct DictionaryNode<'inp> {
    pub entries: Vec<DictionaryEntryNode<'inp>>,
    pub context: NodeContext,
}

#[derive(Debug, Clone)]
pub struct DictionaryEntryNode<'inp> {
    pub key: DictionaryEntryKey<'inp>,
    pub value: Did<'inp>,
    pub context: NodeContext,
}

#[derive(Debug, Clone)]
pub enum DictionaryEntryKey<'inp> {
    Number(NumberNode),
    Text(TextNode<'inp>),
}

#[derive(Debug, Clone)]
pub struct ObjectNode<'inp> {
    pub name: TextNode<'inp>,
    pub entries: Vec<ObjectEntryNode<'inp>>,
    pub context: NodeContext,
}

#[derive(Debug, Clone)]
pub struct ObjectEntryNode<'inp> {
    pub key: TextNode<'inp>,
    pub value: Did<'inp>,
    pub context: NodeContext,
}

#[derive(Debug, Clone)]
pub struct ListNode<'inp> {
    pub entries: Vec<Did<'inp>>,
    pub context: NodeContext,
}

impl From<TextNode<'_>> for crate::api::TextNode {
    fn from(value: TextNode<'_>) -> Self {
        crate::api::TextNode {
            value: value.value.to_string(),
            context: value.context,
        }
    }
}

impl From<DictionaryNode<'_>> for crate::api::DictionaryNode {
    fn from(value: DictionaryNode<'_>) -> Self {
        crate::api::DictionaryNode {
            entries: value
                .entries
                .into_iter()
                .map(|entry| crate::api::DictionaryEntryNode {
                    key: match entry.key {
                        DictionaryEntryKey::Number(node) => {
                            crate::api::DictionaryEntryKey::Number(node)
                        }
                        DictionaryEntryKey::Text(node) => {
                            crate::api::DictionaryEntryKey::Text(node.into())
                        }
                    },
                    value: AstNode::from(entry.value).into(),
                    context: entry.context,
                })
                .collect(),
            context: value.context,
        }
    }
}

impl From<ObjectNode<'_>> for crate::api::ObjectNode {
    fn from(value: ObjectNode<'_>) -> Self {
        crate::api::ObjectNode {
            name: value.name.into(),
            entries: value
                .entries
                .into_iter()
                .map(|entry| crate::api::ObjectEntryNode {
                    key: entry.key.into(),
                    value: AstNode::from(entry.value).into(),
                    context: entry.context,
                })
                .collect(),
            context: value.context,
        }
    }
}

impl From<ListNode<'_>> for crate::api::ListNode {
    fn from(value: ListNode<'_>) -> Self {
        crate::api::ListNode {
            entries: value
                .entries
                .into_iter()
                .map(AstNode::from)
                .map(Into::into)
                .collect(),
            context: value.context,
        }
    }
}

impl From<Did<'_>> for AstNode {
    fn from(value: Did) -> Self {
        match value {
            Did::Empty(node) => AstNode::Empty(node),
            Did::Logical(node) => AstNode::Logical(node),
            Did::Number(node) => AstNode::Number(node),
            Did::Text(node) => AstNode::Text(node.into()),
            Did::Dictionary(node) => AstNode::Dictionary(node.into()),
            Did::Object(node) => AstNode::Object(node.into()),
            Did::List(node) => AstNode::List(node.into()),
        }
    }
}
