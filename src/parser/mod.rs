pub mod ast;

use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

#[cfg(not(feature = "wasm"))]
use self::ast::*;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
struct DidParser;

#[cfg(not(feature = "wasm"))]
pub fn parse(input: &str) -> Result<Did<'_>, Error<Rule>> {
    let input = DidParser::parse(Rule::did, input)?.next().unwrap();

    fn ast_node(pair: Pair<'_, Rule>) -> Did<'_> {
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
                value: pair.as_str(),
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
                                value: key_pair.as_str(),
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

#[cfg(feature = "wasm")]
use crate::api::*;

#[cfg(feature = "wasm")]
pub fn parse(input: &str) -> Result<AstNode, Error<Rule>> {
    use wai_bindgen_rust::Handle;

    use crate::BoxedAstNode;

    let input = DidParser::parse(Rule::did, input)?.next().unwrap();

    fn ast_node(pair: Pair<'_, Rule>) -> AstNode {
        let (line, col) = pair.line_col();
        let context = NodeContext {
            line: line as u64,
            column: col as u64,
            index: pair.as_span().start() as u64,
        };
        match pair.as_rule() {
            Rule::empty => AstNode::Empty(EmptyNode { context }),
            Rule::logical => AstNode::Logical(LogicalNode {
                value: match pair.as_str() {
                    "так" => true,
                    "ні" => false,
                    _ => unreachable!(),
                },
                context,
            }),
            Rule::number => AstNode::Number(NumberNode {
                value: pair.as_str().to_owned(),
                context,
            }),
            Rule::text => AstNode::Text(TextNode {
                value: pair.into_inner().next().unwrap().as_str().to_owned(),
                context,
            }),
            Rule::dict => AstNode::Dictionary(DictionaryNode {
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
                                value: key_pair.as_str().to_owned(),
                                context: key_context,
                            }),
                            Rule::text => DictionaryEntryKey::Text(TextNode {
                                value: key_pair.into_inner().next().unwrap().as_str().to_owned(),
                                context: key_context,
                            }),
                            Rule::ident => DictionaryEntryKey::Text(TextNode {
                                value: key_pair.as_str().to_owned(),
                                context: key_context,
                            }),
                            rule => unreachable!("{rule:?} {:?}", key_pair.as_str()),
                        };
                        let value: Handle<BoxedAstNode> =
                            ast_node(inner_rules.next().unwrap()).into();
                        DictionaryEntryNode {
                            context: key_context,
                            key,
                            value,
                        }
                    })
                    .collect(),
                context,
            }),
            Rule::object => AstNode::Object({
                let mut inner_rules = pair.into_inner();
                let name: String = inner_rules.next().unwrap().as_str().to_owned();
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
                                    value: key_pair.as_str().to_owned(),
                                    context: key_context,
                                },
                                _ => unreachable!(),
                            };
                            let value: Handle<BoxedAstNode> =
                                ast_node(inner_rules.next().unwrap()).into();
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
            Rule::list => AstNode::List(ListNode {
                entries: pair.into_inner().map(ast_node).map(Into::into).collect(),
                context,
            }),
            rule => unreachable!("{rule:?} {:?}", pair.as_str()),
        }
    }

    Ok(ast_node(input))
}
