use nom::{bytes::complete::tag, combinator::value};

use crate::api::{EmptyNode, NodeContext, ParseError, ParseErrorExpectation};

use super::{Offset, Parse, ParseResult, make_info};

impl Parse for EmptyNode {
    fn parse(
        input: &str,
        context @ NodeContext {
            line,
            column,
            index,
        }: NodeContext,
    ) -> ParseResult<'_, Self> {
        value::<_, _, _, (), _>(
            (
                Self { context },
                context
                    + Offset {
                        line: 0,
                        column: 5,
                        index: 5,
                    },
            ),
            tag("пусто"),
        )(input)
        .map_err(|_| ParseError {
            expectation: ParseErrorExpectation::EmptyNode,
            line,
            column,
            index,
            info: Some(make_info(input)),
        })
    }
}
