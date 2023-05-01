use std::{
    fmt,
    ops::{Add, AddAssign, Neg},
};

use crate::{
    api::{
        AstNode, DictionaryEntryNode, DictionaryNode, EmptyNode, ListNode, LogicalNode,
        NodeContext, Number, NumberNode, ObjectEntryNode, ObjectNode, ParseError, ParseErrorKind,
        TextNode,
    },
    parser::Offset,
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

    fn add(self, rhs: Offset) -> Self::Output {
        Self {
            line: self.line + rhs.line,
            column: self.column + rhs.column,
            index: self.index + rhs.index,
        }
    }
}

impl AddAssign<Offset> for NodeContext {
    fn add_assign(&mut self, rhs: Offset) {
        self.line += rhs.line;
        self.column += rhs.column;
        self.index += rhs.index;
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

impl Eq for AstNode {}

impl From<EmptyNode> for AstNode {
    #[inline]
    fn from(value: EmptyNode) -> Self {
        Self::Empty(value)
    }
}

impl PartialEq for EmptyNode {
    fn eq(&self, other: &Self) -> bool {
        self.context == other.context
    }
}

impl Eq for EmptyNode {}

impl From<LogicalNode> for AstNode {
    #[inline]
    fn from(value: LogicalNode) -> Self {
        Self::Logical(value)
    }
}

impl PartialEq for LogicalNode {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.context == other.context
    }
}

impl Eq for LogicalNode {}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Integer(a), Self::Integer(b)) => a == b,
            (Self::Float(a), Self::Float(b)) => a.total_cmp(b).is_eq(),
            (Self::Integer(a), Self::Float(b)) => (*a as f64).total_cmp(b).is_eq(),
            (Self::Float(a), Self::Integer(b)) => a.total_cmp(&(*b as f64)).is_eq(),
        }
    }
}

impl Eq for Number {}

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
        self.value == other.value && self.context == other.context
    }
}

impl Eq for NumberNode {}

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
        self.value == other.value && self.context == other.context
    }
}

impl Eq for TextNode {}

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
        self.context == other.context
            && self
                .entries
                .iter()
                .zip(other.entries.iter())
                .all(|(l, r)| l.0 == r.0)
    }
}

impl Eq for ListNode {}

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
        self.context == other.context && self.entries == other.entries
    }
}

impl Eq for DictionaryNode {}

impl Clone for DictionaryEntryNode {
    fn clone(&self) -> Self {
        Self {
            context: self.context,
            key: self.key.clone(),
            value: self.value.clone(),
        }
    }
}

impl PartialEq for DictionaryEntryNode {
    fn eq(&self, other: &Self) -> bool {
        self.context == other.context && self.key == other.key && self.value.0 == other.value.0
    }
}

impl Eq for DictionaryEntryNode {}

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
        self.name == other.name && self.context == other.context && self.entries == other.entries
    }
}

impl Eq for ObjectNode {}

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
        self.context == other.context && self.key == other.key && self.value.0 == other.value.0
    }
}

impl fmt::Display for ParseErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseErrorKind::ExpectedEmptyNode => write!(f, "Очікувався вузол `пусто`"),
            ParseErrorKind::ExpectedLogicalNode => {
                write!(f, "Очікувався логічний вузол (`так` або `ні`)")
            }
            ParseErrorKind::ExpectedNumberNode => write!(f, "Очікувався числовий вузол"),
            ParseErrorKind::ExpectedTextNode => write!(
                f,
                "Очікувався текстовий вузол. Явні перенесення рядків не дозволені"
            ),
            ParseErrorKind::ExpectedListNode => write!(f, "Очікувався список"),
            ParseErrorKind::ExpectedDictionaryNode => write!(f, "Очікувався словник"),
            ParseErrorKind::ExpectedObjectNode => write!(f, "Очікувався об'єкт"),
            ParseErrorKind::ExpectedEof => write!(f, "Очікувався кінець файлу"),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Тип помилки: {}", self.kind)?;
        write!(
            f,
            "Рядок: {}, Стовпчик: {}, Індекс: {}",
            self.line, self.column, self.index
        )?;
        if let Some(info) = &self.info {
            write!(f, "\n{info}")?;
        }
        Ok(())
    }
}

impl std::error::Error for ParseError {}

impl PartialEq for ParseError {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
            && self.line == other.line
            && self.column == other.column
            && self.index == other.index
            && self.info == other.info
    }
}
