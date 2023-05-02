use nom::{
    bytes::complete::escaped,
    character::complete::{char, one_of, satisfy},
    combinator::{map, recognize},
    multi::many0,
    sequence::delimited,
};

use crate::api::{NodeContext, ParseError, ParseErrorExpectation, TextNode};

use super::{make_info, Offset, Parse, ParseResult};

impl Parse for TextNode {
    fn parse(
        input: &str,
        context @ NodeContext {
            line,
            column,
            index,
        }: NodeContext,
    ) -> ParseResult<'_, Self> {
        delimited::<_, _, _, _, (), _, _, _>(
            char('"'),
            map(
                escaped(
                    recognize(many0(satisfy(|c| c != '\n' && c != '"'))),
                    '\\',
                    one_of(r#"\nrtbf""#),
                ),
                |s: &str| {
                    (
                        Self {
                            context,
                            value: s.to_string(),
                        },
                        context + {
                            let len = s.chars().count() as u64 + 2;
                            Offset {
                                line: 0,
                                column: len,
                                index: len,
                            }
                        },
                    )
                },
            ),
            char('"'),
        )(input)
        .map_err(|_| ParseError {
            expectation: ParseErrorExpectation::TextNode,
            line,
            column,
            index,
            info: Some(make_info(input)),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        api::{NodeContext, ParseError, ParseErrorExpectation, TextNode},
        parser::Parse,
    };

    #[test]
    fn test_text_node_parse_for_empty_str_input() {
        let input = r#""""#;
        let expected_output = (
            TextNode {
                context: NodeContext {
                    line: 0,
                    column: 0,
                    index: 0,
                },
                value: String::new(),
            },
            NodeContext {
                line: 0,
                column: 2,
                index: 2,
            },
        );
        assert_eq!(
            TextNode::parse(input, expected_output.0.context).unwrap(),
            ("", expected_output)
        );
    }

    #[test]
    fn test_text_node_parse_for_valid_input() {
        let input = r#""Hello, World!""#;
        let expected_output = (
            TextNode {
                context: NodeContext {
                    line: 0,
                    column: 0,
                    index: 0,
                },
                value: "Hello, World!".to_string(),
            },
            NodeContext {
                line: 0,
                column: 15,
                index: 15,
            },
        );
        assert_eq!(
            TextNode::parse(input, expected_output.0.context).unwrap(),
            ("", expected_output)
        );
    }

    #[test]
    fn test_text_node_parse_for_valid_input_with_escape_sqs() {
        let input = r#""Hello,\nWorld!""#;
        let expected_output = (
            TextNode {
                context: NodeContext {
                    line: 0,
                    column: 0,
                    index: 0,
                },
                value: r#"Hello,\nWorld!"#.to_string(),
            },
            NodeContext {
                line: 0,
                column: 16,
                index: 16,
            },
        );
        assert_eq!(
            TextNode::parse(input, expected_output.0.context).unwrap(),
            ("", expected_output)
        );
    }

    #[test]
    fn test_text_node_parse_for_invalid_input() {
        let input = "\"Invalid\nInput\"";
        let expected_error = ParseError {
            expectation: ParseErrorExpectation::TextNode,
            line: 0,
            column: 0,
            index: 0,
            info: Some(String::from(
                r#"Наступні символи були введені: "\"Invalid\nI...""#,
            )),
        };
        assert_eq!(
            TextNode::parse(
                input,
                NodeContext {
                    line: 0,
                    column: 0,
                    index: 0
                }
            )
            .unwrap_err(),
            expected_error
        );
    }
}
