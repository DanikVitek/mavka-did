pub mod empty_node;
pub mod logical_node;
pub mod number_node;
pub mod text_node;

use nom::character::complete::multispace0;

use crate::api::{AstNode, NodeContext, ParseError, ParseErrorKind};

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
    (input, node, node_context) = parse_ast(input, node_context)?;

    (input, offset) = skip_whitespace(input);
    node_context += offset; // line 1 + offset, col 1 + offset, idx offset

    expect_eof(input, node_context)?;

    Ok(node)
}

pub fn parse_ast<'inp>(
    input: &'inp str,
    context: NodeContext,
) -> Result<(&'inp str, AstNode, NodeContext), ParseError> {
    let node_context = NodeContext::default();

    todo!()
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Offset {
    pub line: u64,
    pub column: u64,
    pub index: u64,
}

fn skip_whitespace<'inp>(input: &'inp str) -> (&'inp str, Offset) {
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
        kind: ParseErrorKind::ExpectedEof,
        line,
        column,
        index,
        info: Some(make_info(input)),
    })
}

pub(crate) fn make_info(input: &str) -> String {
    format!("Наступні символи були введені: {:?}", {
        const MAX_CHARS: usize = 10;
        let first_ten_chars = input.chars().take(MAX_CHARS).collect::<String>();
        if input.chars().take(MAX_CHARS + 1).count() > MAX_CHARS {
            format!("{}...", first_ten_chars)
        } else {
            format!("{}", first_ten_chars)
        }
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
