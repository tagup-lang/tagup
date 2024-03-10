mod models;

use models::*;
use tagup_scanner::Token;

pub fn parse(tokens: Vec<Token>) -> Program {
  let iterator = &mut tokens.iter().peekable();

  Program {
    fragments: vec![]//parse_fragments(iterator)
  }
}