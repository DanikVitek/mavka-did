use nom::character::complete::char;

use crate::api::{ListEntryNode, ListNode, NodeContext, ParseError, ParseErrorExpectation};

use super::{list_of_entries, make_info, skip_whitespace, Offset, Parse, ParseResult};

impl Parse for ListNode {
    fn parse(mut input: &str, mut context: NodeContext) -> ParseResult<'_, Self> {
        let original_context = context;

        (input, (_, context)) = list_start(input, context)?;

        let entries: Vec<ListEntryNode>;
        (input, (entries, context)) = list_of_entries(input, context, ListEntryNode::parse)?;

        (input, (_, context)) = list_end(input, context)?;

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

pub fn list_start(
    mut input: &str,
    mut context @ NodeContext {
        line,
        column,
        index,
    }: NodeContext,
) -> ParseResult<'_, ()> {
    (input, _) = char::<_, ()>('[')(input).map_err(|_| ParseError {
        expectation: ParseErrorExpectation::LeftBracket,
        line,
        column,
        index,
        info: Some(make_info(input)),
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

pub fn list_end(mut input: &str, mut context: NodeContext) -> ParseResult<'_, ()> {
    let offset: Offset;
    (input, offset) = skip_whitespace(input);

    context += offset;

    (input, _) = char::<_, ()>(']')(input).map_err(|_| ParseError {
        expectation: ParseErrorExpectation::RightBracket,
        line: context.line,
        column: context.column,
        index: context.index,
        info: Some(make_info(input)),
    })?;

    context.index += 1;
    context.column += 1;

    Ok((input, ((), context)))
}
