use crate::api::{NodeContext, ObjectEntryNode, ObjectNode, TextNode};

use super::{
    dictionary_node::{dictionary_end, dictionary_start},
    list_of_entries, parse_ident, Parse, ParseResult,
};

impl Parse for ObjectNode {
    fn parse(mut input: &str, mut context: NodeContext) -> ParseResult<'_, Self> {
        let original_context = context;

        let name: TextNode;
        (input, (name, context)) = parse_ident(input, context)?;

        (input, (_, context)) = dictionary_start(input, context)?;

        let entries: Vec<ObjectEntryNode>;
        (input, (entries, context)) = list_of_entries(input, context, ObjectEntryNode::parse)?;

        (input, (_, context)) = dictionary_end(input, context)?;

        Ok((
            input,
            (
                Self {
                    name,
                    entries,
                    context: original_context,
                },
                context,
            ),
        ))
    }
}
