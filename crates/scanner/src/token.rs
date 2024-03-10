/// Represents a token in the source string.
#[derive(Debug)]
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
  IfClose
}