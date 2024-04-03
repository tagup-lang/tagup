use crate::models::Token;
use std::iter::Peekable;

/// Scans the source string and returns a vector of tokens.
pub fn scan(source: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut start = 0;

    // Create a peekable iterator of the source string.
    let chars = &mut source.char_indices().peekable();

    // Iterate over the source string.
    while let Some((index, current)) = chars.peek().cloned() {
        match current {
            '\n' => {
                chars.next();
                if start < index {
                    tokens.push(Token::Text(source[start..index].to_string()));
                }
                tokens.push(Token::NewLine);
                start = index + 1;
            }
            ' ' => {
                chars.next();
                let mut counter = 1;
                while let Some((_, current)) = chars.peek() {
                    if current == &' ' || current == &'\t' {
                        counter += 1;
                        chars.next();
                    } else {
                        break;
                    }
                }

                if counter > 1 || start == index {
                    if start < index {
                        tokens.push(Token::Text(source[start..index].to_string()));
                    }
                    tokens.push(Token::Space(counter));
                    start = index + counter as usize;
                }
            }
            ':' => {
                chars.next(); // Move past first.
                if let Some(_) = chars.next_if(|(_, c)| c == &':') {
                    if start < index {
                        tokens.push(Token::Text(source[start..index].to_string()));
                    }
                    tokens.push(Token::DelimiterLeft);
                    start = index + 1;

                    while let Some((index, current)) = chars.peek().cloned() {
                        if current == ':' {
                            if let Some(_) = chars.next_if(|(_, c)| c == &':') {
                                tokens.push(Token::DelimiterRight);
                                start = index + 2;
                                break;
                            }
                        }

                        if let Some(token) = consume(chars) {
                            tokens.push(token);
                        }
                    }
                }
            }
            _ => {
                chars.next();
            }
        }
    }

    if start < source.len() {
        // Push the remaining text to the tokens vector.
        tokens.push(Token::Text(source[start..].to_string()));
    }

    tokens
}

/// Consumes the next character(s) from the iterator and returns a token.
fn consume<I: Iterator<Item = (usize, char)>>(chars: &mut Peekable<I>) -> Option<Token> {
    match chars.next() {
        Some((_, current)) => {
            match current {
                '.' => Some(Token::Dot),
                '+' => Some(Token::Plus),
                '-' => Some(Token::Minus),
                '*' => Some(Token::Star),
                '!' => {
                    if let Some((_, current)) = chars.peek() {
                        if current == &'=' {
                            chars.next();
                            return Some(Token::BangEqual);
                        }
                    }
                    Some(Token::Bang)
                }
                '=' => {
                    if let Some((_, current)) = chars.peek() {
                        if current == &'=' {
                            chars.next();
                            return Some(Token::Equal);
                        }
                    }
                    None
                }
                '>' => {
                    if let Some((_, current)) = chars.peek() {
                        if current == &'=' {
                            chars.next();
                            return Some(Token::GreaterEqual);
                        }
                    }
                    Some(Token::Greater)
                }
                '<' => {
                    if let Some((_, current)) = chars.peek() {
                        if current == &'=' {
                            chars.next();
                            return Some(Token::LessEqual);
                        }
                    }
                    Some(Token::Less)
                }
                '/' => {
                    let reserved: String = chars
                        .take_while(|&(_, current)| current.is_alphanumeric())
                        .map(|(_, current)| current)
                        .collect();

                    if reserved == "for" {
                        Some(Token::ForClose)
                    } else if reserved == "if" {
                        Some(Token::IfClose)
                    } else {
                        Some(Token::Slash)
                    }
                }
                '#' => {
                    let reserved: String = chars
                        .take_while(|&(_, current)| current.is_alphanumeric())
                        .map(|(_, current)| current)
                        .collect();

                    if reserved == "for" {
                        Some(Token::ForOpen)
                    } else if reserved == "if" {
                        Some(Token::IfOpen)
                    } else {
                        Some(Token::Hash)
                    }
                }
                ':' => {
                    let reserved: String = chars
                        .take_while(|&(_, current)| current.is_alphanumeric())
                        .map(|(_, current)| current)
                        .collect();

                    if reserved == "else" {
                        Some(Token::Else)
                    } else {
                        Some(Token::Colon)
                    }
                }
                '"' => {
                    let mut buffer = String::new();

                    while let Some((_, current)) = chars.peek() {
                        if current != &'"' {
                            buffer.push(*current);
                            chars.next();
                        } else {
                            break;
                        }
                    }

                    // Skip the closing quote.
                    chars.next();

                    Some(Token::String(buffer))
                }
                'a'..='z' | 'A'..='Z' => {
                    let mut buffer: String = current.to_string();

                    while let Some((_, current)) = chars.peek() {
                        if current.is_alphanumeric() {
                            buffer.push(*current);
                            chars.next();
                        } else {
                            break;
                        }
                    }

                    Some(Token::Identifier(buffer))
                }
                '0'..='9' => {
                    let mut buffer: String = current.to_string();

                    while let Some((_, current)) = chars.peek() {
                        if current.is_alphanumeric() {
                            buffer.push(*current);
                            chars.next();
                        } else {
                            break;
                        }
                    }

                    if let Ok(number) = buffer.parse() {
                        Some(Token::Number(number))
                    } else {
                        None
                    }
                }
                _ => None,
            }
        }
        None => None,
    }
}
