use nom::{
    branch::alt,
    character::complete::{char, digit1, i64},
    combinator::{map, map_res, opt, recognize},
    sequence::{pair, tuple},
    IResult,
};

use crate::api::{NodeContext, Number, NumberNode, ParseError, ParseErrorExpectation};

use super::{make_info, Offset, Parse, ParseResult};

impl Parse for NumberNode {
    fn parse(
        input: &str,
        context @ NodeContext {
            line,
            column,
            index,
        }: NodeContext,
    ) -> ParseResult<'_, Self> {
        number(context)(input).map_err(|_| ParseError {
            expectation: ParseErrorExpectation::NumberNode,
            line,
            column,
            index,
            info: make_info(input),
        })
    }
}

fn number<'inp>(
    context: NodeContext,
) -> impl FnMut(&'inp str) -> IResult<&'inp str, (NumberNode, NodeContext)> {
    map(
        pair(opt(char('-')), alt((float(context), int(context)))),
        |(s, (n, c))| match s {
            None => (n, c),
            Some(_) => (
                -n,
                c + Offset {
                    line: 0,
                    column: 1,
                    index: 1,
                },
            ),
        },
    )
}

fn float<'inp>(
    context: NodeContext,
) -> impl FnMut(&'inp str) -> IResult<&'inp str, (NumberNode, NodeContext)> {
    map_res(
        recognize(tuple((digit1, char('.'), digit1))),
        move |s: &str| {
            s.parse::<f64>().map(|n| {
                (
                    NumberNode {
                        value: Number::Float(n),
                        context,
                    },
                    context
                        + Offset {
                            line: 0,
                            column: s.len() as u64,
                            index: s.len() as u64,
                        },
                )
            })
        },
    )
}

fn int<'inp>(
    context: NodeContext,
) -> impl FnMut(&'inp str) -> IResult<&'inp str, (NumberNode, NodeContext)> {
    map(i64, move |n| {
        (
            NumberNode {
                value: Number::Integer(n),
                context,
            },
            context
                + Offset {
                    line: 0,
                    column: n.to_string().len() as u64,
                    index: n.to_string().len() as u64,
                },
        )
    })
}
