mod boxed_ast_node;
mod dictionary_entry_node;
mod dictionary_node;
mod empty_node;
mod list_node;
mod logical_node;
mod number_node;
mod object_entry_node;
mod object_node;
mod text_node;

use nom::{
    branch::alt,
    character::complete::{char, multispace0, one_of},
    combinator::{map, recognize},
    multi::many0,
    sequence::tuple,
    AsChar, IResult,
};

use crate::api::{
    AstNode, DictionaryNode, EmptyNode, ListNode, LogicalNode, NodeContext, NumberNode, ObjectNode,
    ParseError, ParseErrorExpectation, TextNode,
};

pub type ParseResult<'inp, T> = Result<(&'inp str, (T, NodeContext)), ParseError>;

pub trait Parse: Sized {
    fn parse(input: &str, context: NodeContext) -> ParseResult<'_, Self>;
}

pub trait ParseNode: Parse + Into<AstNode> {}

impl<T: Parse + Into<AstNode>> ParseNode for T {}

pub fn parse(mut input: &str) -> Result<AstNode, ParseError> {
    let mut offset: Offset;

    (input, offset) = skip_whitespace(input);

    let node: AstNode;
    let mut node_context = NodeContext::default() + offset;
    (input, (node, node_context)) = parse_ast(input, node_context)?;

    (input, offset) = skip_whitespace(input);
    node_context += offset; // line 1 + offset, col 1 + offset, idx offset

    expect_eof(input, node_context)?;

    Ok(node)
}

pub(crate) fn parse_ast(
    input: &str,
    context @ NodeContext {
        line,
        column,
        index,
    }: NodeContext,
) -> ParseResult<'_, AstNode> {
    fn parse_map<T: ParseNode>(input: &str, context: NodeContext) -> ParseResult<'_, AstNode> {
        T::parse(input, context).map(|(i, (n, c))| (i, (n.into(), c)))
    }
    let mut errors = Vec::new();
    parse_map::<EmptyNode>(input, context)
        .map_err(|e| errors.push(e))
        .or_else(|_| parse_map::<LogicalNode>(input, context).map_err(|e| errors.push(e)))
        .or_else(|_| parse_map::<NumberNode>(input, context).map_err(|e| errors.push(e)))
        .or_else(|_| parse_map::<TextNode>(input, context).map_err(|e| errors.push(e)))
        .or_else(|_| parse_map::<DictionaryNode>(input, context).map_err(|e| errors.push(e)))
        .or_else(|_| parse_map::<ObjectNode>(input, context).map_err(|e| errors.push(e)))
        .or_else(|_| parse_map::<ListNode>(input, context).map_err(|e| errors.push(e)))
        .map_err(|_| ParseError {
            expectation: ParseErrorExpectation::AstNode,
            line,
            column,
            index,
            info: make_info(input),
        })
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct Offset {
    pub line: u64,
    pub column: u64,
    pub index: u64,
}

pub(crate) fn skip_whitespace<'inp>(input: &'inp str) -> (&'inp str, Offset) {
    let (input, ws) = multispace0::<_, ()>(input).unwrap();

    let lines = ws.lines().collect::<Vec<_>>();
    let offset = Offset {
        index: ws.chars().count() as u64,
        column: lines.last().copied().unwrap_or_default().chars().count() as u64,
        line: (!ws.is_empty())
            .then(|| {
                let mut line = lines.len() as u64;
                if !ws.ends_with('\n') {
                    line -= 1;
                }
                line
            })
            .unwrap_or_default(),
    };

    (input, offset)
}

pub(crate) fn list_of_entries<'inp, T, F>(
    mut input: &'inp str,
    mut context: NodeContext,
    mut entry_parser: F,
) -> ParseResult<'inp, Vec<T>>
where
    F: FnMut(&'inp str, NodeContext) -> ParseResult<'inp, T>,
{
    let mut entries: Vec<T> = Vec::new();
    let mut ws1;
    let mut ws2;
    loop {
        let entry_res = entry_parser(input, context);
        if let Err(err) = entry_res {
            if entries.is_empty() {
                return Ok((input, (entries, context)));
            }
            return Err(err);
        }
        let entry: T;
        (input, (entry, context)) = entry_res.unwrap();
        entries.push(entry);

        (input, ws1) = skip_whitespace(input);
        input = match char::<_, ()>(',')(input) {
            Ok((input, _)) => input,
            Err(_) => break,
        };
        (input, ws2) = skip_whitespace(input);
        context += ws1;
        context += Offset {
            line: 0,
            column: 1,
            index: 1,
        };
        context += ws2;
    }

    Ok((input, (entries, context)))
}

pub(crate) fn make_info(input: &str) -> String {
    format!(
        "Наступні символи були введені: {:?}",
        {
            const MAX_CHARS: usize = 10;
            let first_ten_chars = input.chars().take(MAX_CHARS).collect::<String>();
            if input.chars().take(MAX_CHARS + 1).count() > MAX_CHARS {
                format!("{}...", first_ten_chars)
            } else {
                first_ten_chars
            }
        }
    )
}

pub(crate) fn parse_ident(
    input: &str,
    context @ NodeContext {
        line,
        column,
        index,
    }: NodeContext,
) -> ParseResult<'_, TextNode> {
    fn alpha(input: &str) -> IResult<&str, char, ()> {
        match input.chars().next().filter(|c| match *c {
            'a'..='z' | 'A'..='Z' | 'а'..='я' | 'А'..='Я' => true,
            c => "іІїЇєЄґҐ".contains(c),
        }) {
            Some(c) => Ok((&input[c.len()..], c)),
            None => Err(nom::Err::Error(())),
        }
    }

    fn alphanumeric(input: &str) -> IResult<&str, char, ()> {
        match input.chars().next().filter(|c| match *c {
            'a'..='z' | 'A'..='Z' | 'а'..='я' | 'А'..='Я' => true,
            c => "іІїЇєЄґҐ".contains(c),
        }) {
            Some(c) => Ok((&input[c.len()..], c)),
            None => Err(nom::Err::Error(())),
        }
    }

    fn alphanumeric_or_underscore<E>(input: &str) -> IResult<&str, Vec<char>, E> {
        Ok(many0(alt((char('_'), alphanumeric)))(input).unwrap())
    }

    map(
        recognize(tuple((
            alt((char('_'), alpha)),
            alphanumeric_or_underscore,
            many0(tuple((one_of("'ʼ"), alpha, alphanumeric_or_underscore))),
        ))),
        |ident| {
            let len = ident.chars().count() as u64;
            (
                TextNode {
                    value: ident.to_string(),
                    context,
                },
                context
                    + Offset {
                        line: 0,
                        column: len,
                        index: len,
                    },
            )
        },
    )(input)
    .map_err(|_| ParseError {
        expectation: ParseErrorExpectation::Identifier,
        line,
        column,
        index,
        info: make_info(input),
    })
}

fn expect_eof(
    input: &str,
    NodeContext {
        line,
        column,
        index,
    }: NodeContext,
) -> Result<(), ParseError> {
    if input.is_empty() {
        return Ok(());
    }
    Err(ParseError {
        expectation: ParseErrorExpectation::Eof,
        line,
        column,
        index,
        info: make_info(input),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skip_whitespace_should_make_valid_offset_for_empty_string() {
        let input = "";
        let (rest, offset) = skip_whitespace(input);
        assert_eq!(rest, "");
        assert_eq!(offset, Offset::default());
    }

    #[test]
    fn skip_whitespace_should_make_valid_offset_for_string_with_only_spaces() {
        let input = "   ";
        let (rest, offset) = skip_whitespace(input);
        assert_eq!(rest, "");
        assert_eq!(
            offset,
            Offset {
                line: 0,
                column: 3,
                index: 3,
            }
        );
    }

    #[test]
    fn skip_whitespace_should_make_valid_offset_for_string_with_only_newlines() {
        let input = "\n\n\n";
        let (rest, offset) = skip_whitespace(input);
        assert_eq!(rest, "");
        assert_eq!(
            offset,
            Offset {
                line: 3,
                column: 0,
                index: 3,
            }
        );
    }

    #[test]
    fn skip_whitespace_should_make_valid_offset_for_string_with_only_tabs() {
        let input = "\t\t\t";
        let (rest, offset) = skip_whitespace(input);
        assert_eq!(rest, "");
        assert_eq!(
            offset,
            Offset {
                line: 0,
                column: 3,
                index: 3,
            }
        );
    }

    #[test]
    fn skip_whitespace_should_make_valid_offset_for_string_with_only_spaces_and_newlines1() {
        let input = "   \n\n\n";
        dbg!(input.lines().collect::<Vec<_>>());
        let (rest, offset) = skip_whitespace(input);
        assert_eq!(rest, "");
        assert_eq!(
            offset,
            Offset {
                line: 3,
                column: 0,
                index: 6,
            }
        );
    }

    #[test]
    fn skip_whitespace_should_make_valid_offset_for_string_with_only_spaces_and_newlines2() {
        let input = "   \n\n\n   ";
        let (rest, offset) = skip_whitespace(input);
        assert_eq!(rest, "");
        assert_eq!(
            offset,
            Offset {
                line: 3,
                column: 3,
                index: 9,
            }
        );
    }
}
