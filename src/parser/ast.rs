use derive_more::{IsVariant, Unwrap};

#[derive(Debug, Clone, Copy)]
pub struct NodeContext {
    pub line: u64,
    pub column: u64,
    pub index: u64,
}

#[derive(Debug, Clone, IsVariant, Unwrap)]
pub enum Did<'inp> {
    Empty(EmptyNode),
    Logical(LogicalNode),
    Number(NumberNode<'inp>),
    Text(TextNode<'inp>),
    Dictionary(DictionaryNode<'inp>),
    Object(ObjectNode<'inp>),
    List(ListNode<'inp>),
}

#[derive(Debug, Clone)]
pub struct EmptyNode {
    pub context: NodeContext,
}

#[derive(Debug, Clone)]
pub struct LogicalNode {
    pub value: bool,
    pub context: NodeContext,
}

#[derive(Debug, Clone)]
pub struct NumberNode<'inp> {
    pub value: &'inp str,
    pub context: NodeContext,
}

#[derive(Debug, Clone)]
pub struct TextNode<'inp> {
    pub value: &'inp str,
    pub context: NodeContext,
}

#[derive(Debug, Clone)]
pub struct DictionaryNode<'inp> {
    pub entries: Vec<DictionaryEntryNode<'inp>>,
    pub context: NodeContext,
}

#[derive(Debug, Clone)]
pub struct DictionaryEntryNode<'inp> {
    pub key: DictionaryEntryKey<'inp>,
    pub value: Did<'inp>,
    pub context: NodeContext,
}

#[derive(Debug, Clone)]
pub enum DictionaryEntryKey<'inp> {
    Number(NumberNode<'inp>),
    Text(TextNode<'inp>),
}

#[derive(Debug, Clone)]
pub struct ObjectNode<'inp> {
    pub name: TextNode<'inp>,
    pub entries: Vec<ObjectEntryNode<'inp>>,
    pub context: NodeContext,
}

#[derive(Debug, Clone)]
pub struct ObjectEntryNode<'inp> {
    pub key: TextNode<'inp>,
    pub value: Did<'inp>,
    pub context: NodeContext,
}

#[derive(Debug, Clone)]
pub struct ListNode<'inp> {
    pub entries: Vec<Did<'inp>>,
    pub context: NodeContext,
}
