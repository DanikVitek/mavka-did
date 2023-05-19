use nom::character::complete::char;

use crate::api::{
    AstNode, NodeContext, ObjectEntryNode, ParseError, ParseErrorExpectation, TextNode,
};

use super::{make_info, parse_ast, parse_ident, skip_whitespace, Offset, Parse, ParseResult};

impl Parse for ObjectEntryNode {
    fn parse(mut input: &str, mut context: NodeContext) -> ParseResult<'_, Self> {
        let original_context = context;

        let key: TextNode;
        (input, (key, context)) = parse_ident(input, context)?;

        let mut offset: Offset;
        (input, offset) = skip_whitespace(input);
        context += offset;

        (input, _) = char::<_, ()>('=')(input).map_err(|_| ParseError {
            expectation: ParseErrorExpectation::EqualsSign,
            line: context.line,
            column: context.column,
            index: context.index,
            info: make_info(input),
        })?;

        (input, offset) = skip_whitespace(input);
        context += offset;

        let node: AstNode;
        (input, (node, context)) = parse_ast(input, context)?;

        Ok((
            input,
            (
                ObjectEntryNode {
                    key,
                    value: node.into(),
                    context: original_context,
                },
                context,
            ),
        ))
    }
}
