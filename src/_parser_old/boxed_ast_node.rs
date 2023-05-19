use wai_bindgen_rust::Handle;

use crate::{api::NodeContext, parser::parse_ast, BoxedAstNode};

use super::{Parse, ParseResult};

impl Parse for Handle<BoxedAstNode> {
    fn parse(input: &str, context: NodeContext) -> ParseResult<'_, Self> {
        parse_ast(input, context).map(|(i, (n, c))| (i, (n.into(), c)))
    }
}
