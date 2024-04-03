use boa_gc::{Finalize, Trace};

/// Represents a token in the source string.
#[derive(Debug, Clone)]
pub enum Token {
    Text(String),
    Space(i32),
    NewLine,
    Identifier(String),
    Number(i32),
    String(String),

    DelimiterLeft,
    DelimiterRight,
    Dot,
    Plus,
    Minus,
    Star,
    Slash,
    Hash,
    Colon,
    Bang,

    Equal,
    BangEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    ForOpen,
    ForClose,
    IfOpen,
    Else,
    IfClose,
}

#[derive(Debug, Trace, Finalize)]
/// Represents a fragment in the template.
pub enum Fragment {
    Text(String),
    Tag(Expression),
    IfBlock(Expression, Box<Vec<Fragment>>),
    ForBlock(Expression, Box<Vec<Fragment>>),
}

#[derive(Debug, Trace, Finalize)]
/// Represents an expression in the template.
pub enum Expression {
    Identifier(String),
    Number(i32),
    String(String),
    Binary(Box<Expression>, Operator, Box<Expression>),
    Unary(Operator, Box<Expression>),
}

#[derive(Debug, Trace, Finalize)]
/// Represents an operator in the expression.
pub enum Operator {
    Plus,
    Minus,
    Star,
    Slash,
    BangEqual,
    Equal,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Bang,
}

#[derive(Debug, Trace, Finalize)]
pub struct Program {
    pub fragments: Vec<Fragment>,
}
