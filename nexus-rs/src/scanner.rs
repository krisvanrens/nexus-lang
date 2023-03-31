use crate::cursor::Cursor;
use crate::token::Token;
use lazy_static::lazy_static;
use std::{collections::HashMap, iter::Peekable, str::Chars};

#[cfg(test)]
use std::f64::consts::PI;

pub type Tokens = Vec<Token>;

pub struct Scanner {
    comment_: bool, //<! Indicates multiline comment state.
}

type TokenMap = HashMap<&'static str, Token>;

impl Scanner {
    pub fn new() -> Self {
        Scanner { comment_: false }
    }

    pub fn scan(&mut self, line: String) -> Tokens {
        lazy_static! {
            static ref KEYWORDS: TokenMap = {
                let mut map: TokenMap = HashMap::new();
                map.insert("false", Token::False);
                map.insert("fn", Token::Function);
                map.insert("for", Token::For);
                map.insert("if", Token::If);
                map.insert("let", Token::Let);
                map.insert("node", Token::Node);
                map.insert("print", Token::Print);
                map.insert("return", Token::Return);
                map.insert("true", Token::True);
                map.insert("while", Token::While);
                map
            };
        }

        let mut tokens = Vec::new();

        // XXX
        let mut cursor = Cursor::new(&line);
        println!("{:?}", cursor);
        cursor.advance();
        println!("{:?}", cursor);
        println!("{}", cursor.eol());

        let mut char_iter = line.chars().peekable();
        while let Some(c) = char_iter.next() {
            if !self.comment_ {
                match c {
                    ' ' | '\r' | '\t' => continue,
                    '(' => tokens.push(Token::LeftParen),
                    ')' => tokens.push(Token::RightParen),
                    '{' => tokens.push(Token::LeftBrace),
                    '}' => tokens.push(Token::RightBrace),
                    ';' => tokens.push(Token::SemiColon),
                    '+' => tokens.push(Token::Plus),
                    '-' => match char_iter.peek() {
                        Some('>') => {
                            char_iter.next();
                            tokens.push(Token::Arrow);
                        }
                        _ => tokens.push(Token::Minus),
                    },
                    '*' => tokens.push(Token::Star),
                    '\\' => tokens.push(Token::BackSlash),
                    '%' => tokens.push(Token::Percent),
                    ',' => tokens.push(Token::Comma),
                    '.' => tokens.push(Token::Dot),
                    '_' => tokens.push(Token::Underscore),
                    '=' => match char_iter.peek() {
                        Some('=') => {
                            char_iter.next();
                            tokens.push(Token::Eq);
                        }
                        _ => tokens.push(Token::Is),
                    },
                    '|' => match char_iter.peek() {
                        Some('|') => {
                            char_iter.next();
                            tokens.push(Token::Or);
                        }
                        _ => tokens.push(Token::Pipe),
                    },
                    '>' => match char_iter.peek() {
                        Some('=') => {
                            char_iter.next();
                            tokens.push(Token::GtEq);
                        }
                        _ => tokens.push(Token::Gt),
                    },
                    '<' => match char_iter.peek() {
                        Some('=') => {
                            char_iter.next();
                            tokens.push(Token::LtEq);
                        }
                        _ => tokens.push(Token::Lt),
                    },
                    '!' => match char_iter.peek() {
                        Some('=') => {
                            char_iter.next();
                            tokens.push(Token::NotEq);
                        }
                        _ => tokens.push(Token::Bang),
                    },
                    '&' => {
                        if char_iter.peek() == Some(&'&') {
                            char_iter.next();
                            tokens.push(Token::And);
                        }
                    }
                    '/' => match char_iter.peek() {
                        Some('/') => break,
                        Some('*') => {
                            char_iter.next();
                            self.comment_ = true;
                        }
                        _ => tokens.push(Token::Slash),
                    },
                    '"' => {
                        tokens.push(Token::String(parse_string(&mut char_iter)));
                    }
                    x if x.is_alphanumeric() => {
                        if c.is_ascii_digit() {
                            tokens.push(Token::Number(parse_number(c, &mut char_iter)));
                        } else {
                            let word = parse_word(c, &mut char_iter);
                            tokens.push(if let Some(token) = KEYWORDS.get(&word.as_str()) {
                                token.clone()
                            } else {
                                Token::Identifier(word)
                            });
                        }
                    }
                    _ => panic!("unexpected character: '{c}'"), // TODO: Handle lexing error.
                }
            } else if (c == '*') && (char_iter.peek() == Some(&'/')) {
                char_iter.next();
                self.comment_ = false;
            }
        }

        tokens
    }
}

impl Default for Scanner {
    fn default() -> Self {
        Self::new()
    }
}

fn parse_string(char_iter: &mut Peekable<Chars>) -> String {
    let mut result = String::new();
    let mut escaped = false;
    let mut ended = false;

    for c in &mut *char_iter {
        match c {
            '"' => {
                if !escaped {
                    ended = true;
                    break;
                } else {
                    result.push(c);
                }
            }
            '\\' => {
                if escaped {
                    result.push(c);
                }

                escaped = !escaped;
            }
            _ => result.push(c),
        }

        escaped = escaped && (c == '\\');
    }

    if !ended {
        panic!("unterminated string"); // TODO: Handle lexing error.
    }

    if escaped {
        panic!("malformed string with escaping"); // TODO: Handle lexing error.
    }

    result
}

#[test]
fn parse_string_test() {
    let test = |input: &str| {
        let s = input.to_string() + "\"";
        let mut iter = s.chars().peekable();
        assert!(
            parse_string(&mut iter)
                == input
                    .to_string()
                    .replace("\\\"", "\"")
                    .replace("\\\\", "\\")
        );
    };

    test("");
    test("x");
    test("abc");
    test("With spaces");
    test("With multiple spaces");
    test("W1th num63r5");
    test("W|]h $pec!@l ch@r@ct#r5!");
    test("With ŮñĭçøƋɇ characters");
    test("With \newli\nes and \tab");

    test(r#"With escaped \"quotes\""#);
    test(r#"With backslashes \\\\"#);
    test(r#"\"quotes at the sides\""#);
}

fn parse_number(current_char: char, char_iter: &mut Peekable<Chars>) -> f64 {
    let mut result = current_char.to_string();

    for c in char_iter {
        match c {
            '0'..='9' | '.' => result.push(c),
            _ => break,
        }
    }

    result.parse::<f64>().expect("failed to parse number") // TODO: Handle lexing error.
}

#[test]
fn parse_number_test() {
    let test = |input: &str, expected: f64| {
        // We need to use indexing here for Unicode support.
        let (index, _) = input.char_indices().nth(1).unwrap_or_default();
        let subword = if index > 0 {
            input[index..].to_string()
        } else {
            "".to_string()
        };
        let mut iter = subword.chars().into_iter().peekable();

        assert!(parse_number(input.chars().next().unwrap(), &mut iter) - expected < 0.001);
    };

    test("0", 0.0);
    test("1", 1.0);
    test("0.0", 0.0);
    test("1.0", 1.0);
    test("0.0000", 0.0);
    test("1.0000", 1.0);
    test("1000", 1000.0);
    test("123456", 123456.0);
    test("123.456", 123.456);
    test("123.456", 123.456);
    test("3.1415926535", PI);
}

fn parse_word(current_char: char, char_iter: &mut Peekable<Chars>) -> String {
    let mut result = current_char.to_string();
    loop {
        match char_iter.peek() {
            Some(c) if c.is_alphanumeric() || c == &'_' => {
                result.push(*c);
                char_iter.next();
            }
            _ => break,
        }
    }

    result
}

#[test]
fn parse_word_test() {
    let test = |word: &str| {
        // We need to use indexing here for Unicode support.
        let (index, _) = word.char_indices().nth(1).unwrap_or_default();
        let subword = if index > 0 {
            word[index..].to_string()
        } else {
            "".to_string()
        };
        let mut iter = subword.chars().into_iter().peekable();

        assert!(parse_word(word.chars().next().unwrap(), &mut iter) == word.to_string());
    };

    test("x");
    test("ah");
    test("word");
    test("function");
    test("CamelCase");
    test("snake_case");
    test("ALLUPPER");
    test("ŮñĭçøƋɇ");
    test("trailing_numbers012");
    test("numbers1n8etw33n");
    test("veeeeeeeerylooooooongidentifier");
}
