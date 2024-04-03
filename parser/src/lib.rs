mod models;
mod scanner;

use models::{Expression, Fragment, Operator, Token};
use std::iter::Peekable;

pub use models::Program;

pub fn parse(source: &str) -> Program {
    let tokens = &mut scanner::scan(source).into_iter().peekable();

    Program {
        fragments: parse_fragments(tokens),
    }
}

fn parse_fragments<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> Vec<Fragment> {
    let mut fragments: Vec<Fragment> = Vec::new();

    while let Some(token) = tokens.peek().cloned() {
        match token {
            Token::Text(text) => {
                tokens.next();
                fragments.push(Fragment::Text(text));
            }
            Token::Space(amount) => {
                tokens.next();
                fragments.push(Fragment::Text(" ".repeat(amount as usize).to_string()));
            }
            Token::NewLine => {
                tokens.next();
                fragments.push(Fragment::Text("\n".to_string()));
            }
            Token::DelimiterLeft => {
                tokens.next(); // Consume Delimiter-left.
                fragments.push(Fragment::Tag(parse_expression(tokens)));
                tokens.next(); // Consume Delimiter-right. Todo: Add check.
            }
            _ => {
                tokens.next();
            }
        };
    }

    fragments
}

fn parse_expression<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> Expression {
    parse_equality(tokens)
}

fn parse_equality<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> Expression {
    let mut expression = parse_comparison(tokens);

    while let Some(token) = tokens.next_if(|t| matches!(t, Token::Equal | Token::BangEqual)) {
        expression = Expression::Binary(
            Box::new(expression),
            if let Token::Equal = token {
                Operator::Equal
            } else {
                Operator::BangEqual
            },
            Box::new(parse_equality(tokens)),
        )
    }

    expression
}

fn parse_comparison<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> Expression {
    let mut expression = parse_term(tokens);

    while let Some(token) = tokens.next_if(|t| {
        matches!(
            t,
            Token::Greater | Token::Less | Token::GreaterEqual | Token::LessEqual
        )
    }) {
        expression = Expression::Binary(
            Box::new(expression),
            if let Token::Greater = token {
                Operator::Greater
            } else if let Token::Less = token {
                Operator::Less
            } else if let Token::GreaterEqual = token {
                Operator::GreaterEqual
            } else {
                Operator::LessEqual
            },
            Box::new(parse_comparison(tokens)),
        )
    }

    expression
}

fn parse_term<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> Expression {
    let mut expression = parse_factor(tokens);

    while let Some(token) = tokens.next_if(|t| matches!(t, Token::Plus | Token::Minus)) {
        expression = Expression::Binary(
            Box::new(expression),
            if let Token::Plus = token {
                Operator::Plus
            } else {
                Operator::Minus
            },
            Box::new(parse_term(tokens)),
        )
    }

    expression
}

fn parse_factor<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> Expression {
    let mut expression = parse_unary(tokens);

    while let Some(token) = tokens.next_if(|t| matches!(t, Token::Star | Token::Slash)) {
        expression = Expression::Binary(
            Box::new(expression),
            if let Token::Star = token {
                Operator::Star
            } else {
                Operator::Slash
            },
            Box::new(parse_factor(tokens)),
        )
    }

    expression
}

fn parse_unary<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> Expression {
    match tokens.peek() {
        token @ Some(Token::Bang | Token::Plus) => Expression::Unary(
            if let Some(Token::Plus) = token {
                Operator::Plus
            } else {
                Operator::Bang
            },
            Box::new(parse_unary(tokens)),
        ),
        _ => parse_primary(tokens),
    }
}

fn parse_primary<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> Expression {
    if let Some(token) = tokens.next() {
        return match token {
            Token::Identifier(identifier) => Expression::Identifier(identifier),
            Token::Number(number) => Expression::Number(number),
            Token::String(string) => Expression::String(string),
            c => panic!("Unknown token {:?}", c),
        };
    }

    panic!("Invalid token");
}
