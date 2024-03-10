
#[derive(Debug)]
/// Represents a fragment in the template.
pub enum Fragment {
  Text(String),
  Tag(Expression),
  IfBlock(Expression, Box<Fragment>),
  ForBlock(Expression, Box<Fragment>)
}

#[derive(Debug)]
/// Represents an expression in the template.
pub enum Expression {
  Identifier(String),
  Number(i32),
  String(String),
  Binary(Box<Expression>, Operator, Box<Expression>),
  Unary(Operator, Box<Expression>)
}

#[derive(Debug)]
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
  Bang
}

#[derive(Debug)]
pub struct Program {
  pub fragments: Vec<Fragment>
}