use nom::{branch::alt, bytes::complete::tag, combinator::value};

use crate::api::{LogicalNode, NodeContext, ParseError, ParseErrorKind};

use super::{Offset, Parse, ParseResult, make_info};

impl Parse for LogicalNode {
    fn parse(
        input: &str,
        context @ NodeContext {
            line,
            column,
            index,
        }: NodeContext,
    ) -> ParseResult<'_, Self> {
        alt::<_, _, (), _>((
            value(
                (
                    Self {
                        context,
                        value: false,
                    },
                    context
                        + Offset {
                            line: 0,
                            column: 2,
                            index: 2,
                        },
                ),
                tag("ні"),
            ),
            value(
                (
                    Self {
                        context,
                        value: true,
                    },
                    context
                        + Offset {
                            line: 0,
                            column: 3,
                            index: 3,
                        },
                ),
                tag("так"),
            ),
        ))(input)
        .map_err(|_| ParseError {
            kind: ParseErrorKind::ExpectedLogicalNode,
            line,
            column,
            index,
            info: Some(make_info(input)),
        })
    }
}
