use mavka_did::{
    node::{LogicalNode, NodeContext},
    parser::Parse,
};

fn main() {
    let result = LogicalNode::parse("так", NodeContext::default()).unwrap();
    println!("{:?}", result);
}
