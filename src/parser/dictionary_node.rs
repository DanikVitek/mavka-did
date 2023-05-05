use nom::character::complete::char;

use crate::{
    api::{DictionaryEntryNode, DictionaryNode, NodeContext, ParseError, ParseErrorExpectation},
    parser::list_of_entries,
};

use super::{make_info, skip_whitespace, Offset, Parse, ParseResult};

impl Parse for DictionaryNode {
    fn parse(mut input: &str, mut context: NodeContext) -> ParseResult<'_, Self> {
        let original_context = context;

        (input, (_, context)) = dictionary_start(input, context)?;

        let entries: Vec<DictionaryEntryNode>;
        (input, (entries, context)) = list_of_entries(input, context, DictionaryEntryNode::parse)?;

        (input, (_, context)) = dictionary_end(input, context)?;

        Ok((
            input,
            (
                Self {
                    entries,
                    context: original_context,
                },
                context,
            ),
        ))
    }
}

pub(super) fn dictionary_start(
    mut input: &str,
    mut context @ NodeContext {
        line,
        column,
        index,
    }: NodeContext,
) -> ParseResult<'_, ()> {
    (input, _) = char::<_, ()>('(')(input).map_err(|_| ParseError {
        expectation: ParseErrorExpectation::LeftParenthesis,
        line,
        column,
        index,
        info: make_info(input),
    })?;
    let mut offset: Offset;
    (input, offset) = skip_whitespace(input);
    offset.index += 1;
    if offset.line == 0 {
        offset.column += 1;
    }
    context += offset;

    Ok((input, ((), context)))
}

pub(super) fn dictionary_end(mut input: &str, mut context: NodeContext) -> ParseResult<'_, ()> {
    let offset: Offset;
    (input, offset) = skip_whitespace(input);

    context += offset;

    (input, _) = char::<_, ()>(')')(input).map_err(|_| ParseError {
        expectation: ParseErrorExpectation::RightParenthesis,
        line: context.line,
        column: context.column,
        index: context.index,
        info: make_info(input),
    })?;

    context.index += 1;
    context.column += 1;

    Ok((input, ((), context)))
}
