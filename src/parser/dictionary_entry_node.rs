use nom::character::complete::char;

use crate::{
    api::{
        AstNode, DictionaryEntryKey, DictionaryEntryNode, NodeContext, NumberNode, ParseError,
        ParseErrorExpectation, TextNode,
    },
    parser::{make_info, parse_ast, skip_whitespace, Offset},
};

use super::{parse_ident, Parse, ParseResult};

impl Parse for DictionaryEntryNode {
    fn parse(mut input: &str, mut context: NodeContext) -> ParseResult<'_, Self> {
        let original_context = context;

        let key: DictionaryEntryKey;
        (input, (key, context)) = parse_entry_key(input, context)?;

        let mut offset: Offset;
        (input, offset) = skip_whitespace(input);
        context += offset;

        (input, _) = char::<_, ()>('=')(input).map_err(|_| ParseError {
            expectation: ParseErrorExpectation::EqualsSign,
            line: context.line,
            column: context.column,
            index: context.index,
            info: Some(make_info(input)),
        })?;

        (input, offset) = skip_whitespace(input);
        context += offset;

        let node: AstNode;
        (input, (node, context)) = parse_ast(input, context)?;

        Ok((
            input,
            (
                DictionaryEntryNode {
                    key,
                    value: node.into(),
                    context: original_context,
                },
                context,
            ),
        ))
    }
}

fn parse_entry_key(
    input: &str,
    context: NodeContext,
) -> Result<(&str, (DictionaryEntryKey, NodeContext)), ParseError> {
    parse_ident(input, context)
        .map(|(i, (k, c))| (i, (DictionaryEntryKey::Text(k), c)))
        .or_else(|_| {
            TextNode::parse(input, context).map(|(i, (k, c))| (i, (DictionaryEntryKey::Text(k), c)))
        })
        .or_else(|_| {
            NumberNode::parse(input, context)
                .map(|(i, (k, c))| (i, (DictionaryEntryKey::Number(k), c)))
        })
        .map_err(|_| ParseError {
            expectation: ParseErrorExpectation::DictionaryEntryKey,
            line: context.line,
            column: context.column,
            index: context.index,
            info: Some(make_info(input)),
        })
}
