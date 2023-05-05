use std::{
    fmt,
    ops::{Add, AddAssign, Neg},
};

use wai_bindgen_rust::Handle;

use crate::{
    api::{
        AstNode, DictionaryEntryKey, DictionaryEntryNode, DictionaryNode, EmptyNode, ListNode,
        LogicalNode, NodeContext, Number, NumberNode, ObjectEntryNode, ObjectNode, ParseError,
        ParseErrorExpectation, TextNode,
    },
    parser::Offset,
    BoxedAstNode,
};

impl Default for NodeContext {
    fn default() -> Self {
        Self {
            line: 1,
            column: 1,
            index: 0,
        }
    }
}

impl Add<Offset> for NodeContext {
    type Output = Self;

    fn add(mut self, rhs: Offset) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign<Offset> for NodeContext {
    fn add_assign(&mut self, rhs: Offset) {
        self.index += rhs.index;

        self.add_lines(rhs.line);
        self.column += rhs.column;
    }
}

impl NodeContext {
    fn add_lines(&mut self, lines: u64) {
        self.line += lines;
        if lines > 0 {
            self.column = 1;
        }
    }
}

impl PartialEq for NodeContext {
    fn eq(&self, other: &Self) -> bool {
        self.line == other.line && self.column == other.column && self.index == other.index
    }
}

impl Eq for NodeContext {}

impl Clone for AstNode {
    fn clone(&self) -> Self {
        match self {
            Self::Empty(node) => Self::Empty(*node),
            Self::Logical(node) => Self::Logical(*node),
            Self::Number(node) => Self::Number(*node),
            Self::Text(node) => Self::Text(node.clone()),
            Self::List(node) => Self::List(node.clone()),
            Self::Dictionary(node) => Self::Dictionary(node.clone()),
            Self::Object(node) => Self::Object(node.clone()),
        }
    }
}

impl PartialEq for AstNode {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Empty(l), Self::Empty(r)) => l == r,
            (Self::Logical(l), Self::Logical(r)) => l == r,
            (Self::Number(l), Self::Number(r)) => l == r,
            (Self::Text(l), Self::Text(r)) => l == r,
            (Self::List(l), Self::List(r)) => l == r,
            (Self::Dictionary(l), Self::Dictionary(r)) => l == r,
            (Self::Object(l), Self::Object(r)) => l == r,
            _ => false,
        }
    }
}

impl From<EmptyNode> for AstNode {
    #[inline]
    fn from(value: EmptyNode) -> Self {
        Self::Empty(value)
    }
}

impl PartialEq for EmptyNode {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl From<LogicalNode> for AstNode {
    #[inline]
    fn from(value: LogicalNode) -> Self {
        Self::Logical(value)
    }
}

impl PartialEq for LogicalNode {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Integer(a), Self::Integer(b)) => a == b,
            (Self::Float(a), Self::Float(b)) => a == b,
            (Self::Integer(a), Self::Float(b)) => (*a as f64) == *b,
            (Self::Float(a), Self::Integer(b)) => *a == (*b as f64),
        }
    }
}

impl Neg for Number {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::Integer(value) => Self::Integer(-value),
            Self::Float(value) => Self::Float(-value),
        }
    }
}

impl From<NumberNode> for AstNode {
    #[inline]
    fn from(value: NumberNode) -> Self {
        Self::Number(value)
    }
}

impl PartialEq for NumberNode {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Neg for NumberNode {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            value: -self.value,
            context: self.context,
        }
    }
}

impl From<TextNode> for AstNode {
    #[inline]
    fn from(value: TextNode) -> Self {
        Self::Text(value)
    }
}

impl PartialEq for TextNode {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl From<ListNode> for AstNode {
    #[inline]
    fn from(value: ListNode) -> Self {
        Self::List(value)
    }
}

impl Clone for ListNode {
    fn clone(&self) -> Self {
        Self {
            context: self.context,
            entries: self.entries.clone(),
        }
    }
}

impl PartialEq for ListNode {
    fn eq(&self, other: &Self) -> bool {
        self.entries
            .iter()
            .zip(other.entries.iter())
            .all(|(l, r)| l.0 == r.0)
    }
}

impl From<DictionaryNode> for AstNode {
    #[inline]
    fn from(value: DictionaryNode) -> Self {
        Self::Dictionary(value)
    }
}

impl Clone for DictionaryNode {
    fn clone(&self) -> Self {
        Self {
            context: self.context,
            entries: self.entries.clone(),
        }
    }
}

impl PartialEq for DictionaryNode {
    fn eq(&self, other: &Self) -> bool {
        self.entries == other.entries
    }
}

impl Clone for DictionaryEntryNode {
    fn clone(&self) -> Self {
        Self {
            context: self.context,
            key: self.key.clone(),
            value: self.value.clone(),
        }
    }
}

impl PartialEq for DictionaryEntryKey {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Text(l0), Self::Text(r0)) => l0 == r0,
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl Eq for DictionaryEntryKey {}

impl PartialEq for DictionaryEntryNode {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.value.0 == other.value.0
    }
}

impl From<ObjectNode> for AstNode {
    #[inline]
    fn from(value: ObjectNode) -> Self {
        Self::Object(value)
    }
}

impl Clone for ObjectNode {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            context: self.context,
            entries: self.entries.clone(),
        }
    }
}

impl PartialEq for ObjectNode {
    fn eq(&self, other: &Self) -> bool {
        self.entries == other.entries && self.name == other.name
    }
}

impl Clone for ObjectEntryNode {
    fn clone(&self) -> Self {
        Self {
            context: self.context,
            key: self.key.clone(),
            value: self.value.clone(),
        }
    }
}

impl PartialEq for ObjectEntryNode {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.value.0 == other.value.0
    }
}

impl fmt::Display for ParseErrorExpectation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::EmptyNode => write!(f, "Очікувався вузол `пусто`"),
            Self::LogicalNode => write!(f, "Очікувався логічний вузол (`так` або `ні`)"),
            Self::NumberNode => write!(f, "Очікувався числовий вузол"),
            Self::TextNode => write!(
                f,
                "Очікувався текстовий вузол. Явні перенесення рядків не дозволені"
            ),
            Self::ListNode => write!(f, "Очікувався список"),
            Self::DictionaryNode => write!(f, "Очікувався словник"),
            Self::DictionaryEntryNode => write!(f, "Очікувався запис словника"),
            Self::DictionaryEntryKey => write!(
                f,
                "Очікувався ключ запису словника (ідентифікатор, текст або число)"
            ),
            Self::ObjectNode => write!(f, "Очікувався об'єкт"),
            Self::ObjectEntryNode => write!(f, "Очікувався запис об'єкта"),
            Self::AstNode => write!(f, "Очікувався вузол формату `Дід`"),
            Self::Identifier => write!(
                f,
                "Очікувався ідентифікатор запису (має починатися з літери або `_`)"
            ),
            Self::EqualsSign => write!(f, "Очікувався знак рівності `=`"),
            Self::LeftParenthesis => write!(f, "Очікувався ліва кругла дужка `(`"),
            Self::RightParenthesis => write!(f, "Очікувався права кругла дужка `)`"),
            Self::LeftBracket => write!(f, "Очікувався ліва квадратна дужка `[`"),
            Self::RightBracket => write!(f, "Очікувався права квадратна дужка `]`"),
            Self::EntryValue => write!(f, "Очікувалася значення запису"),
            Self::Eof => write!(f, "Очікувався кінець файлу"),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Вид помилки: {}", self.expectation)?;
        writeln!(
            f,
            "Рядок: {}, Стовпчик: {}, Індекс: {}",
            self.line, self.column, self.index
        )?;
        write!(f, "{}", self.info)
    }
}

impl std::error::Error for ParseError {}

impl PartialEq for ParseError {
    fn eq(&self, other: &Self) -> bool {
        self.expectation == other.expectation
            && self.line == other.line
            && self.column == other.column
            && self.index == other.index
            && self.info == other.info
    }
}

impl<T: Into<AstNode>> From<T> for BoxedAstNode {
    #[inline]
    fn from(value: T) -> Self {
        BoxedAstNode(value.into())
    }
}

impl From<AstNode> for Handle<BoxedAstNode> {
    #[inline]
    fn from(value: AstNode) -> Self {
        Handle::new(value.into())
    }
}

impl AsRef<AstNode> for Handle<BoxedAstNode> {
    fn as_ref(&self) -> &AstNode {
        &self.0
    }
}
